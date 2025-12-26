#!/usr/bin/env python3
"""
Usage: python wasm_func_sizes.py <file.wasm> [--filter PATTERN] [--top N]

Requires: wasm-objdump (apt install wabt)
"""

import subprocess
import re
import sys
import argparse

def demangle(name):
    """Simple Rust symbol demangling"""
    return (name
        .replace('_ZN', '')
        .replace('$LT$', '<')
        .replace('$GT$', '>')
        .replace('$u20$', ' ')
        .replace('$u27$', "'")
        .replace('$C$', ',')
        .replace('$RF$', '&')
        .replace('$BP$', '*')
        .replace('..', '::'))

def get_function_sizes(wasm_file):
    result = subprocess.run(
        ['wasm-objdump', '-d', wasm_file],
        capture_output=True, text=True
    )

    funcs = []
    for line in result.stdout.split('\n'):
        match = re.match(r'^([0-9a-f]+) func\[(\d+)\] <(.+)>:', line)
        if match:
            addr = int(match.group(1), 16)
            idx = int(match.group(2))
            name = match.group(3)
            funcs.append((addr, idx, name))

    # Calculate sizes
    sizes = []
    for i in range(len(funcs) - 1):
        addr, idx, name = funcs[i]
        next_addr = funcs[i + 1][0]
        size = next_addr - addr
        sizes.append((size, idx, name, demangle(name)))

    return sizes

def compare_wasm(file1, file2, top=1000000, filter_pattern=None):
    sizes1 = get_function_sizes(file1)
    sizes2 = get_function_sizes(file2)

    # Build dicts by mangled name (unique identifier)
    dict1 = {name: size for size, idx, name, demangled in sizes1}
    dict2 = {name: size for size, idx, name, demangled in sizes2}

    all_names = set(dict1.keys()) | set(dict2.keys())

    diffs = []
    for name in all_names:
        s1 = dict1.get(name, 0)
        s2 = dict2.get(name, 0)
        diff = s2 - s1
        if diff != 0:
            demangled = demangle(name)
            if filter_pattern and filter_pattern.lower() not in demangled.lower():
                continue
            diffs.append((diff, s1, s2, demangled))

    diffs.sort(key=lambda x: -x[0])

    # New functions
    new_funcs = [(d, s1, s2, n) for d, s1, s2, n in diffs if s1 == 0]
    removed_funcs = [(d, s1, s2, n) for d, s1, s2, n in diffs if s2 == 0]
    changed_funcs = [(d, s1, s2, n) for d, s1, s2, n in diffs if s1 > 0 and s2 > 0]

    total_new = sum(d for d, _, _, _ in new_funcs)
    total_removed = sum(d for d, _, _, _ in removed_funcs)
    total_changed = sum(d for d, _, _, _ in changed_funcs)

    print(f"=== NEW functions ({len(new_funcs)}, +{total_new:,} bytes) ===\n")
    print(f"{'Size':>8}  Function")
    print("-" * 70)
    for diff, s1, s2, name in new_funcs[:top]:
        print(f"{s2:>+8,}  {name}")

    if changed_funcs:
        print(f"\n=== CHANGED functions ({len(changed_funcs)}, {total_changed:+,} bytes) ===\n")
        print(f"{'Diff':>8}  {'Old':>8}  {'New':>8}  Function")
        print("-" * 80)
        for diff, s1, s2, name in changed_funcs[:top]:
            print(f"{diff:>+8,}  {s1:>8,}  {s2:>8,}  {name}")

    if removed_funcs:
        print(f"\n=== REMOVED functions ({len(removed_funcs)}, {total_removed:,} bytes) ===\n")
        print(f"{'Size':>8}  Function")
        print("-" * 70)
        for diff, s1, s2, name in removed_funcs[:top]:
            print(f"{diff:>+8,}  {name}")

    print(f"\n=== SUMMARY ===")
    print(f"New:     {total_new:>+10,} bytes ({len(new_funcs)} functions)")
    print(f"Removed: {total_removed:>+10,} bytes ({len(removed_funcs)} functions)")
    print(f"Changed: {total_changed:>+10,} bytes ({len(changed_funcs)} functions)")
    print(f"TOTAL:   {total_new + total_removed + total_changed:>+10,} bytes")

def main():
    parser = argparse.ArgumentParser(description='Analyze WASM function sizes')
    parser.add_argument('wasm_file', help='Path to .wasm file')
    parser.add_argument('--compare', '-c', help='Compare with another .wasm file')
    parser.add_argument('--filter', '-f', help='Filter by pattern (case-insensitive)')
    parser.add_argument('--top', '-n', type=int, default=1000000, help='Show top N functions')
    parser.add_argument('--summary', '-s', action='store_true', help='Group by module/crate')
    args = parser.parse_args()

    if args.compare:
        compare_wasm(args.wasm_file, args.compare, args.top, args.filter)
        return

    sizes = get_function_sizes(args.wasm_file)

    if args.filter:
        pattern = args.filter.lower()
        sizes = [(s, i, n, d) for s, i, n, d in sizes if pattern in d.lower()]

    sizes.sort(reverse=True, key=lambda x: x[0])

    if args.summary:
        # Group by first component
        groups = {}
        for size, idx, name, demangled in sizes:
            parts = demangled.split('::')
            group = parts[0] if parts else 'unknown'
            # Clean up numbers at start
            group = re.sub(r'^\d+', '', group)
            groups[group] = groups.get(group, 0) + size

        print(f"{'Module':<40} {'Size':>10}")
        print("-" * 52)
        for group, size in sorted(groups.items(), key=lambda x: -x[1])[:args.top]:
            print(f"{group:<40} {size:>10,} bytes")
        print("-" * 52)
        print(f"{'TOTAL':<40} {sum(groups.values()):>10,} bytes")
    else:
        print(f"{'Size':>8}  {'Func':>5}  Function")
        print("-" * 80)
        total = 0
        for size, idx, name, demangled in sizes[:args.top]:
            total += size
            print(f"{size:>8,}  [{idx:>3}]  {demangled[:60]}")
        print("-" * 80)
        print(f"{total:>8,}  total ({len(sizes)} functions matched)")

if __name__ == '__main__':
    main()