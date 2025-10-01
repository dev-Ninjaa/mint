use anyhow::Result;
use std::process::Command;
use tracing::{info, error};
use crate::utils;

/// Install a wheel file into a venv or system python (venv_path: Option<&str>)
pub fn install_wheel(path: &str, venv_path: Option<&str>) -> Result<()> {
    info!("Installing wheel: {}", path);
    
    let (python, pip_args) = if let Some(v) = venv_path {
        if cfg!(target_os = "windows") {
            (format!("{}\\Scripts\\python.exe", v), vec!["-m", "pip", "install", "--no-deps", "--force-reinstall", path])
        } else {
            (format!("{}/bin/python3", v), vec!["-m", "pip", "install", "--no-deps", "--force-reinstall", path])
        }
    } else {
        if cfg!(target_os = "windows") {
            ("python.exe".to_string(), vec!["-m", "pip", "install", "--no-deps", "--force-reinstall", path])
        } else {
            ("python3".to_string(), vec!["-m", "pip", "install", "--no-deps", "--force-reinstall", path])
        }
    };

    // Check if Python executable exists
    if !utils::command_exists(&python) {
        anyhow::bail!("Python executable not found: {}", python);
    }

    let output = Command::new(&python)
        .args(&pip_args)
        .output()?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        error!("pip install failed: {}", error_msg);
        anyhow::bail!("pip install failed for {} using {}: {}", path, python, error_msg);
    }
    
    info!("✅ Successfully installed {}", path);
    Ok(())
}

pub fn uninstall_package(pkg: &str, venv_path: Option<&str>) -> Result<()> {
    let (python, pip_args) = if let Some(v) = venv_path {
        if cfg!(target_os = "windows") {
            (format!("{}\\Scripts\\python.exe", v), vec!["-m", "pip", "uninstall", "-y", pkg])
        } else {
            (format!("{}/bin/python3", v), vec!["-m", "pip", "uninstall", "-y", pkg])
        }
    } else {
        if cfg!(target_os = "windows") {
            ("python.exe".to_string(), vec!["-m", "pip", "uninstall", "-y", pkg])
        } else {
            ("python3".to_string(), vec!["-m", "pip", "uninstall", "-y", pkg])
        }
    };

    let status = Command::new(&python)
        .args(&pip_args)
        .status()?;

    if !status.success() {
        anyhow::bail!("pip uninstall failed for {} using {}", pkg, python);
    }
    println!("✅ Uninstalled {}", pkg);
    Ok(())
}

pub fn create_venv(name: &str) -> Result<()> {
    let python_cmd = if cfg!(target_os = "windows") {
        "python.exe"
    } else {
        "python3"
    };

    let status = Command::new(python_cmd)
        .args(&["-m", "venv", name])
        .status()?;
    if !status.success() {
        anyhow::bail!("Failed to create venv {} using {}", name, python_cmd);
    }
    println!("✅ Created venv {}", name);
    Ok(())
}

pub fn run_in_venv(venv: &str, script: &str) -> Result<()> {
    let python = if cfg!(target_os = "windows") {
        format!("{}\\Scripts\\python.exe", venv)
    } else {
        format!("{}/bin/python3", venv)
    };
    
    let status = Command::new(&python)
        .args(&["-c", script])
        .status()?;
    if !status.success() {
        anyhow::bail!("Script failed in venv {} using {}", venv, python);
    }
    Ok(())
}
