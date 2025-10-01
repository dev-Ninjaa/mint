# 🚀 Mint - Ultra-Fast Python Package Manager

[![PyPI version](https://badge.fury.io/py/mnt.svg)](https://badge.fury.io/py/mnt)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Python 3.8+](https://img.shields.io/badge/python-3.8+-blue.svg)](https://www.python.org/downloads/)
[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org/)

**Mint** is a next-generation Python package manager built with Rust for maximum performance. It's designed to be a drop-in replacement for pip with significant speed improvements.

## 🏆 Performance

- **2.4x faster** than pip for individual packages
- **4.9x faster** than pip for bulk installations
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

| Package Manager | Individual (5 pkgs) | Bulk (8 pkgs) | Speedup |
|----------------|-------------------|---------------|---------|
| **Mint**       | **7.30s**         | **2.00s**     | **1.0x** |
| pip            | 17.85s            | 9.82s         | 2.4x slower |
| uv             | 5.03s             | 1.02s         | 0.7x faster |

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

## 🛠️ Development

### Prerequisites

- Python 3.8+
- Rust toolchain (for building from source)

### Build from Source

```bash
# Clone repository
git clone https://github.com/yourusername/mint.git
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

## 📞 Support

- 📧 Email: mint@example.com
- 🐛 Issues: [GitHub Issues](https://github.com/yourusername/mint/issues)
- 📖 Documentation: [GitHub Wiki](https://github.com/yourusername/mint/wiki)

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for maximum performance
- Inspired by [uv](https://github.com/astral-sh/uv) and [pip](https://pip.pypa.io/)
- Uses [PyPI](https://pypi.org/) as package source

---

**Made with ❤️ by the Mint Development Team**
