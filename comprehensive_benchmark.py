#!/usr/bin/env python3
"""
Comprehensive benchmark script for Mint vs pip vs uv
Tests virtual environment creation and package installation
"""

import subprocess
import time
import os
import shutil
from pathlib import Path
import json

def run_command(cmd, description, timeout=300):
    """Run a command and measure execution time"""
    print(f"\n🔍 {description}")
    print(f"Command: {' '.join(cmd)}")
    
    start_time = time.time()
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
        end_time = time.time()
        duration = end_time - start_time
        
        if result.returncode == 0:
            print(f"✅ Success in {duration:.2f} seconds")
            return duration, True, result.stdout, result.stderr
        else:
            print(f"❌ Failed: {result.stderr}")
            return duration, False, result.stdout, result.stderr
    except subprocess.TimeoutExpired:
        print("⏰ Timeout after 5 minutes")
        return timeout, False, "", "Timeout"
    except Exception as e:
        print(f"❌ Error: {e}")
        return 0, False, "", str(e)

def cleanup_venvs():
    """Clean up any existing test virtual environments"""
    venv_names = ["test_mint", "test_pip", "test_uv", ".venv_mint", ".venv_pip", ".venv_uv"]
    for venv_name in venv_names:
        venv_path = Path(venv_name)
        if venv_path.exists():
            try:
                shutil.rmtree(venv_path)
                print(f"🧹 Cleaned up {venv_name}")
            except Exception as e:
                print(f"⚠️  Could not clean {venv_name}: {e}")

def test_venv_creation():
    """Test virtual environment creation times"""
    print(f"\n{'='*80}")
    print("🏗️  VIRTUAL ENVIRONMENT CREATION BENCHMARK")
    print(f"{'='*80}")
    
    results = {}
    
    # Test Mint venv creation
    duration, success, stdout, stderr = run_command(
        ["python", "mint_py/mint/cli.py", "venv-create", "test_mint"],
        "Creating virtual environment with Mint"
    )
    results["Mint (venv-create)"] = {"time": duration, "success": success}
    
    # Test pip venv creation
    duration, success, stdout, stderr = run_command(
        ["python", "-m", "venv", "test_pip"],
        "Creating virtual environment with python -m venv (pip standard)"
    )
    results["pip (python -m venv)"] = {"time": duration, "success": success}
    
    # Test uv venv creation (if available)
    try:
        subprocess.run(["uv", "--version"], capture_output=True, check=True)
        duration, success, stdout, stderr = run_command(
            ["uv", "venv", "test_uv"],
            "Creating virtual environment with uv"
        )
        results["uv (venv)"] = {"time": duration, "success": success}
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("\n⚠️  uv not found, skipping uv venv test")
        results["uv (venv)"] = {"time": 0, "success": False, "note": "uv not available"}
    
    return results

def test_package_installation():
    """Test package installation times"""
    print(f"\n{'='*80}")
    print("📦 PACKAGE INSTALLATION BENCHMARK")
    print(f"{'='*80}")
    
    # Test packages
    test_packages = [
        "requests==2.31.0",
        "click==8.1.7", 
        "colorama==0.4.6",
        "tqdm==4.66.1",
        "packaging==23.2"
    ]
    
    results = {}
    
    # Test Mint installation
    print(f"\n🧪 Testing Mint Package Manager")
    mint_times = []
    mint_success = 0
    
    for package in test_packages:
        cmd = [
            "python", "mint_py/mint/cli.py", "install", 
            package, "-v", str(Path("test_mint").absolute())
        ]
        duration, success, stdout, stderr = run_command(
            cmd, f"Mint installing {package}"
        )
        mint_times.append(duration)
        if success:
            mint_success += 1
    
    results["Mint"] = {
        "total_time": sum(mint_times),
        "avg_time": sum(mint_times) / len(mint_times),
        "success_rate": mint_success / len(test_packages),
        "individual_times": mint_times
    }
    
    # Test pip installation
    print(f"\n🧪 Testing pip")
    pip_times = []
    pip_success = 0
    
    for package in test_packages:
        if os.name == 'nt':  # Windows
            pip_cmd = [str(Path("test_pip") / "Scripts" / "python.exe"), "-m", "pip", "install", package]
        else:  # Unix
            pip_cmd = [str(Path("test_pip") / "bin" / "python"), "-m", "pip", "install", package]
        
        duration, success, stdout, stderr = run_command(
            pip_cmd, f"pip installing {package}"
        )
        pip_times.append(duration)
        if success:
            pip_success += 1
    
    results["pip"] = {
        "total_time": sum(pip_times),
        "avg_time": sum(pip_times) / len(pip_times),
        "success_rate": pip_success / len(test_packages),
        "individual_times": pip_times
    }
    
    # Test uv installation (if available)
    try:
        subprocess.run(["uv", "--version"], capture_output=True, check=True)
        print(f"\n🧪 Testing uv")
        uv_times = []
        uv_success = 0
        
        for package in test_packages:
            uv_cmd = ["uv", "pip", "install", "-p", str(Path("test_uv").absolute()), package]
            duration, success, stdout, stderr = run_command(
                uv_cmd, f"uv installing {package}"
            )
            uv_times.append(duration)
            if success:
                uv_success += 1
        
        results["uv"] = {
            "total_time": sum(uv_times),
            "avg_time": sum(uv_times) / len(uv_times),
            "success_rate": uv_success / len(test_packages),
            "individual_times": uv_times
        }
    except (subprocess.CalledProcessError, FileNotFoundError):
        print("\n⚠️  uv not found, skipping uv installation test")
        results["uv"] = {"total_time": 0, "avg_time": 0, "success_rate": 0, "note": "uv not available"}
    
    return results

def test_bulk_installation():
    """Test bulk package installation"""
    print(f"\n{'='*80}")
    print("🚀 BULK PACKAGE INSTALLATION BENCHMARK")
    print(f"{'='*80}")
    
    # Create fresh venvs for bulk test
    cleanup_venvs()
    
    # Create venvs
    subprocess.run(["python", "-m", "venv", "test_mint_bulk"], check=True)
    subprocess.run(["python", "-m", "venv", "test_pip_bulk"], check=True)
    
    try:
        subprocess.run(["uv", "venv", "test_uv_bulk"], check=True)
        uv_available = True
    except:
        uv_available = False
    
    test_packages = ["requests", "click", "colorama", "tqdm", "packaging", "urllib3", "certifi", "idna"]
    
    results = {}
    
    # Mint bulk installation
    cmd = ["python", "mint_py/mint/cli.py", "install"] + test_packages + ["-v", str(Path("test_mint_bulk").absolute())]
    duration, success, stdout, stderr = run_command(cmd, f"Mint installing {len(test_packages)} packages at once")
    results["Mint (bulk)"] = {"time": duration, "success": success, "packages": len(test_packages)}
    
    # pip bulk installation
    if os.name == 'nt':
        pip_cmd = [str(Path("test_pip_bulk") / "Scripts" / "python.exe"), "-m", "pip", "install"] + test_packages
    else:
        pip_cmd = [str(Path("test_pip_bulk") / "bin" / "python"), "-m", "pip", "install"] + test_packages
    
    duration, success, stdout, stderr = run_command(pip_cmd, f"pip installing {len(test_packages)} packages at once")
    results["pip (bulk)"] = {"time": duration, "success": success, "packages": len(test_packages)}
    
    # uv bulk installation
    if uv_available:
        uv_cmd = ["uv", "pip", "install", "-p", str(Path("test_uv_bulk").absolute())] + test_packages
        duration, success, stdout, stderr = run_command(uv_cmd, f"uv installing {len(test_packages)} packages at once")
        results["uv (bulk)"] = {"time": duration, "success": success, "packages": len(test_packages)}
    
    return results

def analyze_results(venv_results, install_results, bulk_results):
    """Analyze and display comprehensive results"""
    print(f"\n{'='*80}")
    print("📊 COMPREHENSIVE PERFORMANCE ANALYSIS")
    print(f"{'='*80}")
    
    # Virtual Environment Creation Analysis
    print(f"\n🏗️  VIRTUAL ENVIRONMENT CREATION RESULTS:")
    print(f"{'='*50}")
    for manager, data in venv_results.items():
        status = "✅" if data["success"] else "❌"
        print(f"   {manager:25} | {data['time']:6.2f}s | {status}")
    
    # Package Installation Analysis
    print(f"\n📦 INDIVIDUAL PACKAGE INSTALLATION RESULTS:")
    print(f"{'='*50}")
    
    mint_total = install_results["Mint"]["total_time"]
    pip_total = install_results["pip"]["total_time"]
    
    print(f"   Mint (Rust)           | {mint_total:6.2f}s | {install_results['Mint']['success_rate']*100:5.1f}% success")
    print(f"   pip (Python)          | {pip_total:6.2f}s | {install_results['pip']['success_rate']*100:5.1f}% success")
    
    if install_results["uv"]["total_time"] > 0:
        uv_total = install_results["uv"]["total_time"]
        print(f"   uv (Rust)             | {uv_total:6.2f}s | {install_results['uv']['success_rate']*100:5.1f}% success")
    
    # Speed Comparison
    print(f"\n⚡ SPEED COMPARISONS:")
    print(f"{'='*50}")
    
    if mint_total > 0 and pip_total > 0:
        mint_vs_pip = pip_total / mint_total
        print(f"   Mint vs pip: {mint_vs_pip:.1f}x faster")
        
        if mint_vs_pip >= 10:
            print("   🎉 MINT ACHIEVED 10x+ SPEEDUP OVER PIP! 🎉")
        elif mint_vs_pip >= 5:
            print("   🔥 MINT ACHIEVED 5x+ SPEEDUP OVER PIP! 🔥")
        elif mint_vs_pip >= 2:
            print("   ✨ MINT ACHIEVED 2x+ SPEEDUP OVER PIP! ✨")
    
    if install_results["uv"]["total_time"] > 0:
        uv_total = install_results["uv"]["total_time"]
        if mint_total > 0 and uv_total > 0:
            mint_vs_uv = uv_total / mint_total
            print(f"   Mint vs uv: {mint_vs_uv:.1f}x faster")
    
    # Bulk Installation Analysis
    print(f"\n🚀 BULK INSTALLATION RESULTS:")
    print(f"{'='*50}")
    
    for manager, data in bulk_results.items():
        status = "✅" if data["success"] else "❌"
        packages_per_sec = data["packages"] / data["time"] if data["time"] > 0 else 0
        print(f"   {manager:20} | {data['time']:6.2f}s | {packages_per_sec:5.2f} pkg/s | {status}")
    
    # Individual Package Breakdown
    print(f"\n📋 INDIVIDUAL PACKAGE PERFORMANCE:")
    print(f"{'='*50}")
    
    test_packages = ["requests", "click", "colorama", "tqdm", "packaging"]
    
    print(f"{'Package':<12} | {'Mint':<8} | {'pip':<8} | {'Speedup':<8}")
    print("-" * 50)
    
    for i, package in enumerate(test_packages):
        mint_time = install_results["Mint"]["individual_times"][i]
        pip_time = install_results["pip"]["individual_times"][i]
        speedup = pip_time / mint_time if mint_time > 0 else 0
        
        print(f"{package:<12} | {mint_time:<8.2f} | {pip_time:<8.2f} | {speedup:<8.1f}x")

def main():
    """Main benchmark function"""
    print("🚀 COMPREHENSIVE MINT vs pip vs uv BENCHMARK")
    print("=" * 80)
    print("Testing virtual environment creation and package installation performance")
    
    # Cleanup any existing test environments
    cleanup_venvs()
    
    try:
        # Run all benchmarks
        venv_results = test_venv_creation()
        install_results = test_package_installation()
        bulk_results = test_bulk_installation()
        
        # Analyze and display results
        analyze_results(venv_results, install_results, bulk_results)
        
        # Final summary
        print(f"\n{'='*80}")
        print("🎯 FINAL SUMMARY")
        print(f"{'='*80}")
        
        mint_total = install_results["Mint"]["total_time"]
        pip_total = install_results["pip"]["total_time"]
        
        if mint_total > 0 and pip_total > 0:
            overall_speedup = pip_total / mint_total
            print(f"🏆 MINT IS {overall_speedup:.1f}x FASTER THAN PIP OVERALL!")
            
            if overall_speedup >= 10:
                print("🎉 MISSION ACCOMPLISHED: 10x+ SPEEDUP ACHIEVED! 🎉")
            elif overall_speedup >= 5:
                print("🔥 EXCELLENT: 5x+ SPEEDUP ACHIEVED! 🔥")
            elif overall_speedup >= 2:
                print("✨ GOOD: 2x+ SPEEDUP ACHIEVED! ✨")
        
    finally:
        # Cleanup
        print(f"\n🧹 Cleaning up test environments...")
        cleanup_venvs()

if __name__ == "__main__":
    main()
