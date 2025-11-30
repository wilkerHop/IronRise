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

/// Schedules a system wake event using the provided executor.
pub fn schedule_wake_with<E: CommandExecutor>(
    executor: &E,
    time: DateTime<Local>,
) -> Result<(), PmsetError> {
    // "schedule wakeorpoweron ..."
    // To cancel: pmset schedule cancel wakeorpoweron "..."

    // Let's adjust build_wake_args to handle the optional "cancel"
    let mut args = vec!["schedule".to_string()];
    args.push("wakeorpoweron".to_string());
    args.push(time.format("%m/%d/%Y %H:%M:%S").to_string());

    executor.execute("pmset", &args)
}

/// Schedules a system wake event using the default RealExecutor.
pub fn schedule_wake(time: DateTime<Local>) -> Result<(), PmsetError> {
    schedule_wake_with(&RealExecutor, time)
}

/// Cancels a specific scheduled wake event using the provided executor.
pub fn cancel_wake_with<E: CommandExecutor>(
    executor: &E,
    time: DateTime<Local>,
) -> Result<(), PmsetError> {
    let time_str = time.format("%m/%d/%Y %H:%M:%S").to_string();
    let args = vec![
        "schedule".to_string(),
        "cancel".to_string(),
        "wakeorpoweron".to_string(),
        time_str,
    ];
    executor.execute("pmset", &args)
}

/// Cancels a specific scheduled wake event using the default RealExecutor.
pub fn cancel_wake(time: DateTime<Local>) -> Result<(), PmsetError> {
    cancel_wake_with(&RealExecutor, time)
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
        assert_eq!(args[0], "schedule");
        assert_eq!(args[1], "wakeorpoweron");
        assert_eq!(args[2], "01/01/2024 08:30:00");
    }

    #[test]
    fn test_cancel_wake_use_case() {
        let mock = MockExecutor::new();
        let time = Local.with_ymd_and_hms(2024, 1, 1, 8, 30, 0).unwrap();

        // Execute the use case
        cancel_wake_with(&mock, time).unwrap();

        // Verify the interaction
        let commands = mock.executed_commands.borrow();
        assert_eq!(commands.len(), 1);

        let (cmd, args) = &commands[0];
        assert_eq!(cmd, "pmset");
        assert_eq!(args[0], "schedule");
        assert_eq!(args[1], "cancel");
        assert_eq!(args[2], "wakeorpoweron");
        assert_eq!(args[3], "01/01/2024 08:30:00");
    }
}
