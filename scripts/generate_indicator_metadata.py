#!/usr/bin/env python3
"""
Future generator for quantwave Python metadata.

Long-term goal:
- Parse Rust indicator definitions (or a central registry in quantwave-core)
- Generate the _METADATA dict in quantwave/_metadata.py automatically.

This would eliminate the manual maintenance burden and keep Python metadata
in sync with the actual Rust implementations (including default params, 
warmup behavior, categories, etc.).

For 0.5.2 we are still using a hand-maintained registry in _metadata.py.
This script is the seed for the auto-generation system.
"""

def main():
    print("This is a stub for future Rust -> Python metadata code generation.")
    print("Planned approaches:")
    print("  1. Rust proc-macro / build script that emits JSON")
    print("  2. Parse doc comments + function signatures in Rust")
    print("  3. Central IndicatorDefinition struct in quantwave-core that Python can read")
    print()
    print("For now, edit quantwave-python/python/quantwave/_metadata.py manually.")


if __name__ == "__main__":
    main()
