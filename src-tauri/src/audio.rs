use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::BufReader;
use std::process::Command;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct AlarmPlayer {
    stop_signal: Arc<AtomicBool>,
}

impl AlarmPlayer {
    pub fn new() -> Self {
        Self {
            stop_signal: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Sets system volume to 100% using AppleScript
    fn maximize_volume() {
        let _ = Command::new("osascript")
            .arg("-e")
            .arg("set volume output volume 100")
            .output();
    }

    pub fn play_alarm(&self, file_path: &str) {
        // 1. Force Volume Max
        Self::maximize_volume();

        let stop_signal = self.stop_signal.clone();
        let file_path = file_path.to_string();

        // 2. Spawn audio thread
        std::thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            let file = File::open(file_path).unwrap();
            let source = Decoder::new(BufReader::new(file)).unwrap();

            // Loop the source indefinitely
            let source = source.repeat_infinite();

            sink.append(source);
            sink.play();

            // Block until stop signal
            while !stop_signal.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(100));
                // Aggressively keep volume up in case user tries to lower it
                Self::maximize_volume();
            }

            sink.stop();
        });
    }

    pub fn stop(&self) {
        self.stop_signal.store(true, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alarm_player_creation() {
        let player = AlarmPlayer::new();
        // Just verify we can create it and the signal is false
        assert!(!player.stop_signal.load(Ordering::Relaxed));
    }

    #[test]
    fn test_alarm_player_stop() {
        let player = AlarmPlayer::new();
        player.stop();
        assert!(player.stop_signal.load(Ordering::Relaxed));
    }
}
