// Utility functions for Mint package manager
use std::process::Command;

/// Check if a command exists in PATH
pub fn command_exists(cmd: &str) -> bool {
    if cfg!(target_os = "windows") {
        Command::new("where")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    } else {
        Command::new("which")
            .arg(cmd)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}

/// Get the appropriate Python executable name for the current OS
#[allow(dead_code)]
pub fn get_python_executable() -> &'static str {
    if cfg!(target_os = "windows") {
        "python.exe"
    } else {
        "python3"
    }
}

/// Get the appropriate pip executable name for the current OS
#[allow(dead_code)]
pub fn get_pip_executable() -> &'static str {
    if cfg!(target_os = "windows") {
        "pip.exe"
    } else {
        "pip3"
    }
}

/// Format bytes into human readable format
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Calculate download speed in MB/s
#[allow(dead_code)]
pub fn calculate_speed(bytes_per_second: f64) -> String {
    format_bytes((bytes_per_second * 1024.0 * 1024.0) as u64)
}

/// Validate package name according to PEP 508
#[allow(dead_code)]
pub fn is_valid_package_name(name: &str) -> bool {
    if name.is_empty() || name.len() > 214 {
        return false;
    }

    // Check for invalid characters
    for c in name.chars() {
        if !c.is_ascii_alphanumeric() && c != '-' && c != '_' && c != '.' {
            return false;
        }
    }

    // Can't start or end with special characters
    if name.starts_with('-') || name.starts_with('_') || name.starts_with('.') ||
       name.ends_with('-') || name.ends_with('_') || name.ends_with('.') {
        return false;
    }

    true
}
