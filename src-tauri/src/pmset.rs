use std::process::Command;
use chrono::{DateTime, Local};

#[derive(Debug, thiserror::Error)]
pub enum PmsetError {
    #[error("Failed to execute pmset: {0}")]
    ExecutionFailed(std::io::Error),
    #[error("pmset returned non-zero exit code: {0}")]
    CommandFailed(String),
}

/// Schedules a system wake event.
/// Note: This command requires root privileges.
pub fn schedule_wake(time: DateTime<Local>) -> Result<(), PmsetError> {
    // Format time as "MM/dd/yyyy HH:mm:ss" for pmset
    let time_str = time.format("%m/%d/%Y %H:%M:%S").to_string();

    // Command: sudo pmset repeat wakeorpoweron "MM/dd/yyyy HH:mm:ss"
    // In a real GUI app, you'd use a privileged helper or AppleScript to prompt for auth.
    // For this snippet, we assume the process has rights or we use a sudo prompt wrapper.
    let output = Command::new("sudo")
        .arg("pmset")
        .arg("repeat")
        .arg("wakeorpoweron")
        .arg(&time_str)
        .output()
        .map_err(PmsetError::ExecutionFailed)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(PmsetError::CommandFailed(stderr.to_string()));
    }

    Ok(())
}

/// Clears any existing repeat schedule.
pub fn clear_schedule() -> Result<(), PmsetError> {
    let output = Command::new("sudo")
        .arg("pmset")
        .arg("repeat")
        .arg("cancel")
        .output()
        .map_err(PmsetError::ExecutionFailed)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(PmsetError::CommandFailed(stderr.to_string()));
    }

    Ok(())
}
