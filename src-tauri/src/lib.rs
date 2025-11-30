mod pmset;
mod audio;

use audio::AlarmPlayer;
use chrono::DateTime;
use std::sync::Mutex;
use tauri::State;

struct AppState {
    player: Mutex<AlarmPlayer>,
}

#[tauri::command]
fn schedule_alarm(iso_time: String) -> Result<(), String> {
    let time = DateTime::parse_from_rfc3339(&iso_time)
        .map_err(|e| e.to_string())?
        .with_timezone(&chrono::Local);
    
    pmset::schedule_wake(time).map_err(|e| e.to_string())
}

#[tauri::command]
fn cancel_alarm() -> Result<(), String> {
    pmset::clear_schedule().map_err(|e| e.to_string())
}

#[tauri::command]
fn play_alarm(state: State<AppState>, file_path: String) -> Result<(), String> {
    let mut player = state.player.lock().unwrap();
    // Reset player state by creating a new instance
    *player = AlarmPlayer::new();
    player.play_alarm(&file_path);
    Ok(())
}

#[tauri::command]
fn stop_alarm(state: State<AppState>) -> Result<(), String> {
    let player = state.player.lock().unwrap();
    player.stop();
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            player: Mutex::new(AlarmPlayer::new()),
        })
        .invoke_handler(tauri::generate_handler![
            schedule_alarm,
            cancel_alarm,
            play_alarm,
            stop_alarm
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
