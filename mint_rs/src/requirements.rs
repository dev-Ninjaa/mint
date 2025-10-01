use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tracing::info;

/// Parse requirements.txt file
pub fn parse_requirements(path: &PathBuf) -> Result<Vec<String>> {
    if !path.exists() {
        anyhow::bail!("Requirements file not found: {:?}", path);
    }

    let content = fs::read_to_string(path)?;
    let mut packages = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        
        // Skip comments and empty lines
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Handle -r includes
        if line.starts_with("-r ") {
            let include_path = line[3..].trim();
            let include_file = path.parent()
                .map(|p| p.join(include_path))
                .unwrap_or_else(|| PathBuf::from(include_path));
            
            let included = parse_requirements(&include_file)?;
            packages.extend(included);
            continue;
        }

        // Handle -e editable installs
        if line.starts_with("-e ") {
            packages.push(line.to_string());
            continue;
        }

        packages.push(line.to_string());
    }

    info!("Parsed {} packages from requirements file", packages.len());
    Ok(packages)
}

/// Generate requirements.txt from installed packages
pub fn generate_requirements(venv_path: Option<&str>) -> Result<String> {
    let python = if let Some(v) = venv_path {
        if cfg!(target_os = "windows") {
            format!("{}/Scripts/python.exe", v)
        } else {
            format!("{}/bin/python3", v)
        }
    } else {
        if cfg!(target_os = "windows") {
            "python.exe".to_string()
        } else {
            "python3".to_string()
        }
    };

    let output = std::process::Command::new(&python)
        .args(&["-m", "pip", "freeze"])
        .output()?;

    if !output.status.success() {
        anyhow::bail!("Failed to generate requirements");
    }

    let requirements = String::from_utf8_lossy(&output.stdout);
    info!("Generated requirements for {} packages", requirements.lines().count());
    Ok(requirements.to_string())
}

/// Save requirements to file
#[allow(dead_code)]
pub fn save_requirements(packages: &[String], path: &PathBuf) -> Result<()> {
    let content = packages.join("\n");
    fs::write(path, content)?;
    info!("Saved requirements to {:?}", path);
    Ok(())
}
