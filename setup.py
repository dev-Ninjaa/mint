#!/usr/bin/env python3
"""
Setup script for Mint Package Manager
"""

import os
import sys
import subprocess
from pathlib import Path
from setuptools import setup, find_packages
from setuptools.command.build_py import build_py
from setuptools.command.install import install

# Get the long description from README
def get_long_description():
    readme_path = Path(__file__).parent / "README.md"
    if readme_path.exists():
        return readme_path.read_text(encoding="utf-8")
    return "Ultra-fast Python package manager built with Rust"

class BuildRustBinary(build_py):
    """Custom build command to compile Rust binary"""
    
    def run(self):
        # Build the Rust binary
        rust_dir = Path(__file__).parent / "mint_core"
        if rust_dir.exists():
            print("🔨 Building Rust binary...")
            try:
                # Build release version
                result = subprocess.run(
                    ["cargo", "build", "--release"],
                    cwd=rust_dir,
                    capture_output=True,
                    text=True
                )
                
                if result.returncode != 0:
                    print(f"❌ Rust build failed: {result.stderr}")
                    sys.exit(1)
                
                print("✅ Rust binary built successfully")
                
                # Copy binary to package
                target_dir = Path(__file__).parent / "mint" / "bin"
                target_dir.mkdir(exist_ok=True)
                
                if os.name == 'nt':  # Windows
                    src_binary = rust_dir / "target" / "release" / "mint_core.exe"
                    dst_binary = target_dir / "mint_core.exe"
                else:  # Unix
                    src_binary = rust_dir / "target" / "release" / "mint_core"
                    dst_binary = target_dir / "mint_core"
                
                if src_binary.exists():
                    import shutil
                    shutil.copy2(src_binary, dst_binary)
                    print(f"✅ Binary copied to {dst_binary}")
                else:
                    print(f"❌ Binary not found at {src_binary}")
                    sys.exit(1)
                    
            except FileNotFoundError:
                print("❌ Cargo not found. Please install Rust toolchain.")
                sys.exit(1)
        
        # Run normal build
        super().run()

class InstallWithRust(install):
    """Custom install command"""
    
    def run(self):
        # Ensure Rust binary is built
        BuildRustBinary(self.distribution).run()
        super().run()

# Package metadata
setup(
    name="mnt",
    version="0.1.0",
    author="Mint Development Team",
    author_email="mint@example.com",
    description="Ultra-fast Python package manager built with Rust",
    long_description=get_long_description(),
    long_description_content_type="text/markdown",
    url="https://github.com/yourusername/mint",
    packages=find_packages(),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Rust",
        "Topic :: Software Development :: Build Tools",
        "Topic :: System :: Archiving :: Packaging",
    ],
    python_requires=">=3.8",
    entry_points={
        "console_scripts": [
            "mint=mint.cli:main",
            "mnt=mint.cli:main",
        ],
    },
    include_package_data=True,
    package_data={
        "mint": [
            "bin/mint_core*",
        ],
    },
    cmdclass={
        "build_py": BuildRustBinary,
        "install": InstallWithRust,
    },
    install_requires=[
        # No Python dependencies - everything is in Rust
    ],
    extras_require={
        "dev": [
            "pytest>=6.0",
            "black",
            "flake8",
        ],
    },
    keywords="package-manager python rust fast pip alternative",
    project_urls={
        "Bug Reports": "https://github.com/yourusername/mint/issues",
        "Source": "https://github.com/yourusername/mint",
        "Documentation": "https://github.com/yourusername/mint#readme",
    },
)
