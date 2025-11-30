use chrono::{DateTime, Local};
use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum PmsetError {
    #[error("Failed to execute pmset: {0}")]
    ExecutionFailed(std::io::Error),
    #[error("pmset returned non-zero exit code: {0}")]
    CommandFailed(String),
}

/// Helper to build the pmset command arguments.
/// Exposed for testing purposes.
fn build_wake_args(time: &DateTime<Local>) -> Vec<String> {
    let time_str = time.format("%m/%d/%Y %H:%M:%S").to_string();
    vec!["repeat".to_string(), "wakeorpoweron".to_string(), time_str]
}

/// Executes a command with administrator privileges using AppleScript.
fn execute_privileged(command: &str, args: &[String]) -> Result<(), PmsetError> {
    // Construct the full shell command string
    let full_command = format!("{} {}", command, args.join(" "));

    // AppleScript: do shell script "..." with administrator privileges
    let output = Command::new("osascript")
        .arg("-e")
        .arg(format!(
            "do shell script \"{}\" with administrator privileges",
            full_command
        ))
        .output()
        .map_err(PmsetError::ExecutionFailed)?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // User cancelled auth usually returns "User canceled."
        return Err(PmsetError::CommandFailed(stderr.to_string()));
    }

    Ok(())
}

/// Schedules a system wake event.
pub fn schedule_wake(time: DateTime<Local>) -> Result<(), PmsetError> {
    let args = build_wake_args(&time);
    execute_privileged("pmset", &args)
}

/// Clears any existing repeat schedule.
pub fn clear_schedule() -> Result<(), PmsetError> {
    let args = vec!["repeat".to_string(), "cancel".to_string()];
    execute_privileged("pmset", &args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_build_wake_args() {
        // Create a fixed time: Oct 27, 2023 10:00:00
        let time = Local.with_ymd_and_hms(2023, 10, 27, 10, 0, 0).unwrap();
        let args = build_wake_args(&time);

        assert_eq!(args.len(), 3);
        assert_eq!(args[0], "repeat");
        assert_eq!(args[1], "wakeorpoweron");
        assert_eq!(args[2], "10/27/2023 10:00:00");
    }
}
