use anyhow::Result;
use std::process::Command;

/// Install a wheel file into the current venv
pub fn install_wheel(path: &str) -> Result<()> {
    let status = Command::new("python3")
        .args(&["-m", "pip", "install", "--no-deps", path])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to install wheel {}", path);
    }
    println!("✅ Installed {}", path);
    Ok(())
}

/// Uninstall a package from the current venv
pub fn uninstall_package(pkg: &str) -> Result<()> {
    let status = Command::new("python3")
        .args(&["-m", "pip", "uninstall", "-y", pkg])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to uninstall {}", pkg);
    }
    println!("✅ Uninstalled {}", pkg);
    Ok(())
}

/// Create virtual environment
pub fn create_venv(name: &str) -> Result<()> {
    let status = Command::new("python3")
        .args(&["-m", "venv", name])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to create venv {}", name);
    }
    println!("✅ Created venv {}", name);
    Ok(())
}

/// Run script in venv
pub fn run_in_venv(venv: &str, script: &str) -> Result<()> {
    let python_path = format!("{}/bin/python3", venv);
    let status = Command::new(python_path)
        .args(&["-c", script])
        .status()?;

    if !status.success() {
        anyhow::bail!("Failed to run script in venv {}", venv);
    }
    Ok(())
}
