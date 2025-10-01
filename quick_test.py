#!/usr/bin/env python3
"""
Quick test script for Mint package manager
"""

import subprocess
import time
import os
import shutil
from pathlib import Path

def run_command(cmd, description):
    """Run a command and measure execution time"""
    print(f"\n🔍 {description}")
    print(f"Command: {' '.join(cmd)}")
    
    start_time = time.time()
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
        end_time = time.time()
        duration = end_time - start_time
        
        if result.returncode == 0:
            print(f"✅ Success in {duration:.2f} seconds")
            return duration, True
        else:
            print(f"❌ Failed: {result.stderr}")
            return duration, False
    except subprocess.TimeoutExpired:
        print("⏰ Timeout after 2 minutes")
        return 120, False
    except Exception as e:
        print(f"❌ Error: {e}")
        return 0, False

def create_venv(name):
    """Create a virtual environment"""
    venv_path = Path(name)
    if venv_path.exists():
        shutil.rmtree(venv_path)
    
    cmd = ["python", "-m", "venv", str(venv_path)]
    subprocess.run(cmd, check=True)
    print(f"✅ Created virtual environment: {venv_path}")
    return venv_path

def main():
    """Main test function"""
    print("🚀 Quick Mint Package Manager Test")
    print("=" * 50)
    
    # Clean up any existing test environments
    for venv_name in ["test_mint", "test_pip"]:
        venv_path = Path(venv_name)
        if venv_path.exists():
            shutil.rmtree(venv_path)
    
    # Create test environments
    mint_venv = create_venv("test_mint")
    pip_venv = create_venv("test_pip")
    
    # Test packages
    test_packages = ["requests", "click", "colorama"]
    
    print(f"\n📦 Testing with packages: {', '.join(test_packages)}")
    
    # Test Mint
    print(f"\n{'='*50}")
    print("🧪 Testing Mint Package Manager")
    print(f"{'='*50}")
    
    mint_times = []
    for package in test_packages:
        cmd = [
            "python", "mint_py/mint/cli.py", "install", 
            package, "-v", str(mint_venv.absolute())
        ]
        duration, success = run_command(cmd, f"Installing {package} with Mint")
        mint_times.append(duration)
    
    # Test pip
    print(f"\n{'='*50}")
    print("🧪 Testing pip")
    print(f"{'='*50}")
    
    pip_times = []
    for package in test_packages:
        if os.name == 'nt':  # Windows
            pip_cmd = [str(pip_venv / "Scripts" / "python.exe"), "-m", "pip", "install", package]
        else:  # Unix
            pip_cmd = [str(pip_venv / "bin" / "python"), "-m", "pip", "install", package]
        
        duration, success = run_command(pip_cmd, f"Installing {package} with pip")
        pip_times.append(duration)
    
    # Results
    print(f"\n{'='*50}")
    print("🏆 PERFORMANCE RESULTS")
    print(f"{'='*50}")
    
    mint_total = sum(mint_times)
    pip_total = sum(pip_times)
    
    print(f"Mint total time: {mint_total:.2f}s")
    print(f"pip total time: {pip_total:.2f}s")
    
    if mint_total > 0 and pip_total > 0:
        speedup = pip_total / mint_total
        print(f"\n⚡ Mint is {speedup:.1f}x faster than pip!")
        
        if speedup >= 10:
            print("🎉 MINT ACHIEVED 10x+ SPEEDUP! 🎉")
        elif speedup >= 5:
            print("🔥 MINT ACHIEVED 5x+ SPEEDUP! 🔥")
        elif speedup >= 2:
            print("✨ MINT ACHIEVED 2x+ SPEEDUP! ✨")
    
    # Individual package times
    print(f"\n📊 Individual Package Times:")
    for i, package in enumerate(test_packages):
        mint_time = mint_times[i]
        pip_time = pip_times[i]
        if mint_time > 0 and pip_time > 0:
            pkg_speedup = pip_time / mint_time
            print(f"   {package:10} | Mint: {mint_time:5.2f}s | pip: {pip_time:5.2f}s | {pkg_speedup:.1f}x faster")
    
    # Cleanup
    print(f"\n🧹 Cleaning up...")
    shutil.rmtree(mint_venv)
    shutil.rmtree(pip_venv)

if __name__ == "__main__":
    main()
