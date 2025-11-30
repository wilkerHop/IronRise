use chrono::{DateTime, Local};
use std::process::Command;

#[derive(Debug, thiserror::Error)]
pub enum PmsetError {
    #[error("Failed to execute pmset: {0}")]
    ExecutionFailed(std::io::Error),
    #[error("pmset returned non-zero exit code: {0}")]
    CommandFailed(String),
}

/// Trait for executing commands.
/// Allows mocking the actual system calls in tests.
pub trait CommandExecutor {
    fn execute(&self, command: &str, args: &[String]) -> Result<(), PmsetError>;
}

pub struct RealExecutor;

impl CommandExecutor for RealExecutor {
    fn execute(&self, command: &str, args: &[String]) -> Result<(), PmsetError> {
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
            return Err(PmsetError::CommandFailed(stderr.to_string()));
        }

        Ok(())
    }
}

/// Helper to build the pmset command arguments.
fn build_wake_args(time: &DateTime<Local>) -> Vec<String> {
    let time_str = time.format("%m/%d/%Y %H:%M:%S").to_string();
    vec!["repeat".to_string(), "wakeorpoweron".to_string(), time_str]
}

/// Schedules a system wake event using the provided executor.
pub fn schedule_wake_with<E: CommandExecutor>(
    executor: &E,
    time: DateTime<Local>,
) -> Result<(), PmsetError> {
    let args = build_wake_args(&time);
    executor.execute("pmset", &args)
}

/// Schedules a system wake event using the default RealExecutor.
pub fn schedule_wake(time: DateTime<Local>) -> Result<(), PmsetError> {
    schedule_wake_with(&RealExecutor, time)
}

/// Clears any existing repeat schedule using the provided executor.
pub fn clear_schedule_with<E: CommandExecutor>(executor: &E) -> Result<(), PmsetError> {
    let args = vec!["repeat".to_string(), "cancel".to_string()];
    executor.execute("pmset", &args)
}

/// Clears any existing repeat schedule using the default RealExecutor.
pub fn clear_schedule() -> Result<(), PmsetError> {
    clear_schedule_with(&RealExecutor)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::cell::RefCell;

    // Mock Executor that captures commands
    struct MockExecutor {
        executed_commands: RefCell<Vec<(String, Vec<String>)>>,
    }

    impl MockExecutor {
        fn new() -> Self {
            Self {
                executed_commands: RefCell::new(Vec::new()),
            }
        }
    }

    impl CommandExecutor for MockExecutor {
        fn execute(&self, command: &str, args: &[String]) -> Result<(), PmsetError> {
            self.executed_commands
                .borrow_mut()
                .push((command.to_string(), args.to_vec()));
            Ok(())
        }
    }

    #[test]
    fn test_build_wake_args() {
        let time = Local.with_ymd_and_hms(2023, 10, 27, 10, 0, 0).unwrap();
        let args = build_wake_args(&time);

        assert_eq!(args.len(), 3);
        assert_eq!(args[0], "repeat");
        assert_eq!(args[1], "wakeorpoweron");
        assert_eq!(args[2], "10/27/2023 10:00:00");
    }

    #[test]
    fn test_schedule_wake_use_case() {
        let mock = MockExecutor::new();
        let time = Local.with_ymd_and_hms(2024, 1, 1, 8, 30, 0).unwrap();

        // Execute the use case
        schedule_wake_with(&mock, time).unwrap();

        // Verify the interaction
        let commands = mock.executed_commands.borrow();
        assert_eq!(commands.len(), 1);

        let (cmd, args) = &commands[0];
        assert_eq!(cmd, "pmset");
        assert_eq!(args[0], "repeat");
        assert_eq!(args[1], "wakeorpoweron");
        assert_eq!(args[2], "01/01/2024 08:30:00");
    }

    #[test]
    fn test_clear_schedule_use_case() {
        let mock = MockExecutor::new();

        // Execute the use case
        clear_schedule_with(&mock).unwrap();

        // Verify the interaction
        let commands = mock.executed_commands.borrow();
        assert_eq!(commands.len(), 1);

        let (cmd, args) = &commands[0];
        assert_eq!(cmd, "pmset");
        assert_eq!(args[0], "repeat");
        assert_eq!(args[1], "cancel");
    }
}
