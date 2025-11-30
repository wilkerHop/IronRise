mod audio;
mod pmset;

use audio::AlarmPlayer;
use chrono::DateTime;
use std::sync::Mutex;

struct AppState {
    player: Mutex<AlarmPlayer>,
    scheduled_time: Mutex<Option<DateTime<chrono::Local>>>,
}

#[tauri::command]
fn schedule_alarm(state: tauri::State<AppState>, iso_time: String) -> Result<(), String> {
    let time = DateTime::parse_from_rfc3339(&iso_time)
        .map_err(|e| e.to_string())?
        .with_timezone(&chrono::Local);

    pmset::schedule_wake(time).map_err(|e| e.to_string())?;

    // Store the scheduled time so we can cancel it later
    let mut scheduled = state.scheduled_time.lock().unwrap();
    *scheduled = Some(time);

    Ok(())
}

#[tauri::command]
fn cancel_alarm(state: tauri::State<AppState>) -> Result<(), String> {
    let mut scheduled = state.scheduled_time.lock().unwrap();
    if let Some(time) = *scheduled {
        pmset::cancel_wake(time).map_err(|e| e.to_string())?;
        *scheduled = None;
    }
    Ok(())
}

#[tauri::command]
fn play_alarm(state: tauri::State<AppState>, file_path: String) -> Result<(), String> {
    state.player.lock().unwrap().play_alarm(&file_path);
    Ok(())
}

#[tauri::command]
fn stop_alarm(state: tauri::State<AppState>) {
    state.player.lock().unwrap().stop();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(AppState {
            player: Mutex::new(AlarmPlayer::new()),
            scheduled_time: Mutex::new(None),
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
