#!/usr/bin/env python3
"""
Python wrapper CLI for mint. This script loads the Rust backend as an executable
and delegates commands to it, or imports Rust-based functions if you later
provide a Python binding.

For now this wrapper shells out to the compiled mint_core binary.
"""

import subprocess
import sys
import os

def main():
    # Path to rust binary built by cargo
    repo_root = os.path.dirname(os.path.dirname(__file__))  # mint_py/
    
    # Handle Windows executable extension
    if os.name == 'nt':
        binary_name = "mint_core.exe"
    else:
        binary_name = "mint_core"
    
    rust_bin = os.path.join(repo_root, "..", "mint_core", "target", "release", binary_name)
    rust_bin = os.path.normpath(rust_bin)

    if not os.path.exists(rust_bin):
        print(f"Rust binary not found at {rust_bin}")
        print("Build mint_core first with `cargo build --release` (run in mint/mint_core).")
        sys.exit(1)

    # Forward console IO so progress bars and output are visible
    try:
        proc = subprocess.run([rust_bin] + sys.argv[1:], check=False)
        sys.exit(proc.returncode)
    except KeyboardInterrupt:
        print("\nOperation cancelled by user")
        sys.exit(130)
    except Exception as e:
        print(f"Error running mint: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
