use std::process::Command;

pub fn list_formulae() -> Result<Vec<String>, String> {
    let output = Command::new("brew")
        .args(["list", "--formula", "-1"])
        .output()
        .map_err(|e| format!("Failed to run brew: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!(
            "brew list --formula failed ({}): {}",
            output.status,
            stderr.trim()
        ))
    }
}

pub fn list_casks() -> Result<Vec<String>, String> {
    let output = Command::new("brew")
        .args(["list", "--cask", "-1"])
        .output()
        .map_err(|e| format!("Failed to run brew: {e}"))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(|l| !l.is_empty())
            .map(String::from)
            .collect())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!(
            "brew list --cask failed ({}): {}",
            output.status,
            stderr.trim()
        ))
    }
}
