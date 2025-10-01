#!/usr/bin/env python3
"""
Performance test script for Mint package manager
Tests against pip and uv to demonstrate speed improvements
"""

import subprocess
import time
import os
import sys
from pathlib import Path

def run_command(cmd, description):
    """Run a command and measure execution time"""
    print(f"\n🔍 {description}")
    print(f"Command: {' '.join(cmd)}")
    
    start_time = time.time()
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=300)
        end_time = time.time()
        duration = end_time - start_time
        
        if result.returncode == 0:
            print(f"✅ Success in {duration:.2f} seconds")
            return duration, True
        else:
            print(f"❌ Failed: {result.stderr}")
            return duration, False
    except subprocess.TimeoutExpired:
        print("⏰ Timeout after 5 minutes")
        return 300, False
    except Exception as e:
        print(f"❌ Error: {e}")
        return 0, False

def test_package_manager(manager_name, install_cmd, packages):
    """Test a package manager with given packages"""
    print(f"\n{'='*60}")
    print(f"🧪 Testing {manager_name}")
    print(f"{'='*60}")
    
    total_time = 0
    success_count = 0
    
    for package in packages:
        cmd = install_cmd + [package]
        duration, success = run_command(cmd, f"Installing {package}")
        total_time += duration
        if success:
            success_count += 1
    
    avg_time = total_time / len(packages) if packages else 0
    print(f"\n📊 {manager_name} Results:")
    print(f"   Total time: {total_time:.2f}s")
    print(f"   Average time per package: {avg_time:.2f}s")
    print(f"   Success rate: {success_count}/{len(packages)} ({success_count/len(packages)*100:.1f}%)")
    
    return total_time, avg_time, success_count

def create_test_venv(name):
    """Create a test virtual environment"""
    venv_path = Path(f"test_venv_{name}")
    if venv_path.exists():
        import shutil
        shutil.rmtree(venv_path)
    
    cmd = ["python", "-m", "venv", str(venv_path)]
    subprocess.run(cmd, check=True)
    return venv_path

def main():
    """Main test function"""
    print("🚀 Mint Package Manager Performance Test")
    print("=" * 60)
    
    # Test packages (small, fast packages for quick testing)
    test_packages = [
        "requests==2.31.0",
        "click==8.1.7", 
        "colorama==0.4.6",
        "tqdm==4.66.1",
        "packaging==23.2"
    ]
    
    print(f"📦 Testing with packages: {', '.join(test_packages)}")
    
    # Test results storage
    results = {}
    
    # Test 1: Mint Package Manager
    mint_venv = create_test_venv("mint")
    mint_cmd = ["python", "mint_py/mint/cli.py", "install", "-v", str(mint_venv)]
    total_time, avg_time, success_count = test_package_manager(
        "Mint (Rust)", mint_cmd, test_packages
    )
    results["Mint"] = {"total": total_time, "avg": avg_time, "success": success_count}
    
    # Test 2: pip
    pip_venv = create_test_venv("pip")
    pip_cmd = [str(pip_venv / "Scripts" / "python.exe"), "-m", "pip", "install"]
    total_time, avg_time, success_count = test_package_manager(
        "pip (Python)", pip_cmd, test_packages
    )
    results["pip"] = {"total": total_time, "avg": avg_time, "success": success_count}
    
    # Test 3: uv (if available)
    try:
        subprocess.run(["uv", "--version"], capture_output=True, check=True)
        uv_venv = create_test_venv("uv")
        uv_cmd = ["uv", "pip", "install", "-p", str(uv_venv)]
        total_time, avg_time, success_count = test_package_manager(
            "uv (Rust)", uv_cmd, test_packages
        )
        results["uv"] = {"total": total_time, "avg": avg_time, "success": success_count}
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("\n⚠️  uv not found, skipping uv test")
    
    # Performance comparison
    print(f"\n{'='*60}")
    print("🏆 PERFORMANCE COMPARISON")
    print(f"{'='*60}")
    
    mint_total = results["Mint"]["total"]
    pip_total = results["pip"]["total"]
    
    if mint_total > 0 and pip_total > 0:
        speedup = pip_total / mint_total
        print(f"⚡ Mint is {speedup:.1f}x faster than pip!")
        
        if speedup >= 10:
            print("🎉 MINT ACHIEVED 10x+ SPEEDUP! 🎉")
        elif speedup >= 5:
            print("🔥 MINT ACHIEVED 5x+ SPEEDUP! 🔥")
        elif speedup >= 2:
            print("✨ MINT ACHIEVED 2x+ SPEEDUP! ✨")
    
    if "uv" in results:
        uv_total = results["uv"]["total"]
        if mint_total > 0 and uv_total > 0:
            uv_speedup = uv_total / mint_total
            print(f"⚡ Mint is {uv_speedup:.1f}x faster than uv!")
    
    print(f"\n📈 Detailed Results:")
    for manager, data in results.items():
        print(f"   {manager:12} | Total: {data['total']:6.2f}s | Avg: {data['avg']:5.2f}s | Success: {data['success']}/5")
    
    # Cleanup
    print(f"\n🧹 Cleaning up test environments...")
    import shutil
    for venv_name in ["mint", "pip", "uv"]:
        venv_path = Path(f"test_venv_{venv_name}")
        if venv_path.exists():
            shutil.rmtree(venv_path)

if __name__ == "__main__":
    main()