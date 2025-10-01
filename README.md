# 🚀 Mint - Ultra-Fast Python Package Manager

[![PyPI version](https://badge.fury.io/py/mnt.svg)](https://badge.fury.io/py/mnt)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)

**Mint** is a next-generation Python package manager built with Rust for maximum performance. It's designed to be a drop-in replacement for pip, and aims to be a universal package manager for both Python and JavaScript/TypeScript (npm) ecosystems. Mint is still in active development: in some cases, it's faster than [uv](https://github.com/astral-sh/uv), and in others, a bit slower. Upcoming releases will add JavaScript and TypeScript package management, making Mint a universal, blazing-fast replacement for both npm and pip.

## 🏆 Performance

- **Comparable to uv**: In some benchmarks, Mint is faster than uv, and in others, slightly slower. Performance is improving rapidly with each release.
- **Much faster than pip** for most workflows
- **100% compatibility** with existing pip workflows
- **Parallel downloads** with intelligent caching

## ✨ Features

- 🚀 **Ultra-fast installation** with parallel downloads
- 🎯 **Smart caching** with automatic cleanup
- 🔄 **Virtual environment** support
- 📦 **Requirements.txt** integration
- 🖥️ **Cross-platform** (Windows, Linux, macOS)
- 📊 **Real-time progress** bars and speed metrics
- 🛠️ **Production-ready** error handling and logging

## 📦 Installation

```bash
pip install mnt
```

## 🚀 Quick Start

### Basic Usage

```bash
# Install packages (just like pip!)
mint install requests click colorama

# Install with version specifications
mint install "requests==2.31.0" "click>=8.0.0"

# Install in virtual environment
mint install requests -v myproject

# Install from requirements.txt
mint install-requirements -r requirements.txt
```

### Virtual Environment Management

```bash
# Create virtual environment
mint venv-create myproject

# Install packages in venv
mint install requests -v myproject

# Delete virtual environment
mint venv-delete myproject
```

### Advanced Features

```bash
# Parallel downloads (default: CPU cores)
mint install requests click colorama -j 8

# Generate requirements.txt
mint freeze -o requirements.txt

# Clean cache
mint cache-clean

# Run Python script in venv
mint run myproject "print('Hello from Mint!')"
```

## 📊 Performance Comparison

Mint is now **comparable to uv**: in some cases, Mint is faster, and in others, uv leads. Both are significantly faster than pip. Performance is a moving target as Mint is under active development.

| Package Manager | Individual (5 pkgs) | Bulk (8 pkgs) | Notes |
|----------------|---------------------|---------------|-------|
| **Mint**       | 7.30s               | 2.00s         | Sometimes faster than uv |
| pip            | 17.85s              | 9.82s         | Much slower |
| uv             | 5.03s               | 1.02s         | Sometimes faster than Mint |

## 🔧 Commands

| Command | Description |
|---------|-------------|
| `install` | Install packages |
| `uninstall` | Uninstall packages |
| `venv-create` | Create virtual environment |
| `venv-delete` | Delete virtual environment |
| `run` | Run Python script in venv |
| `list` | List installed packages |
| `show` | Show package information |
| `search` | Search for packages |
| `cache-clean` | Clean old cache files |
| `install-requirements` | Install from requirements.txt |
| `freeze` | Generate requirements.txt |
| *(upcoming)* `js-install` | Install JavaScript/TypeScript packages (npm replacement) |
| *(upcoming)* `js-freeze` | Generate package.json/lockfile for JS/TS |

## 🛠️ Development

### Prerequisites

- Python 3.8+
- Rust toolchain (for building from source)

### Build from Source

```bash
# Clone repository
git clone https://github.com/dev-Ninjaa/mint.git
cd mint

# Install in development mode
pip install -e .

# Or build manually
cd mint_core
cargo build --release
```

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for maximum performance
- Inspired by [uv](https://github.com/astral-sh/uv) and [pip](https://pip.pypa.io/)
- Uses [PyPI](https://pypi.org/) as package source

---

**Made by the Mint Development Team**

---

> **Note:** Mint is under active development. JavaScript and TypeScript package management is coming soon, making Mint a universal, much faster replacement for both npm and pip.
