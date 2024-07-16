use std::process::Command;

fn install_homebrew() -> std::io::Result<()> {
    let output = Command::new("sh")
        .arg("-c")
        .arg("$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)")
        .output()?;

    if output.status.success() {
        println!("Installation successful");
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    } else {
        eprintln!("Installation failed");
        eprintln!("Error: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(())
}
