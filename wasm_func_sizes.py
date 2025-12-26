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

def main():
    parser = argparse.ArgumentParser(description='Analyze WASM function sizes')
    parser.add_argument('wasm_file', help='Path to .wasm file')
    parser.add_argument('--filter', '-f', help='Filter by pattern (case-insensitive)')
    parser.add_argument('--top', '-n', type=int, default=1000000, help='Show top N functions')
    parser.add_argument('--summary', '-s', action='store_true', help='Group by module/crate')
    args = parser.parse_args()

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