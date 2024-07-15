mod apps;
use anyhow::Result;
use std::process::Command;

pub async fn install_apps(apps: Vec<String>) -> Result<()> {
    for app in apps {
        let package = ALL_BREW_PACKAGES.iter().find(|p| p.name == app);
        
        if let Some(package) = package {
            println!("Installing {}", package.name);
            let mut cmd = Command::new("brew");
            
            match package.package_type {
                BrewPackageType::Cask => {
                    cmd.args(&["install", "--cask"]);
                }
                BrewPackageType::Formula => {
                    cmd.arg("install");
                }
            }
            
            let output = cmd.arg(&package.name).output()?;

            if output.status.success() {
                println!("{} installed successfully", package.name);
            } else {
                println!("Failed to install {}", package.name);
            }
        } else {
            println!("Package {} not found in the predefined list", app);
        }
    }
    Ok(())
}

pub fn get_all_package_names() -> Vec<String> {
    ALL_BREW_PACKAGES.iter().map(|p| p.name.to_string()).collect()
}
