import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useState } from "react";
import "./App.css";

function App() {
  const [alarmTime, setAlarmTime] = useState("");
  const [status, setStatus] = useState("");
  const [filePath, setFilePath] = useState("/System/Library/Sounds/Glass.aiff");
  const [isNightMode, setIsNightMode] = useState(false);

  async function scheduleAlarm() {
    try {
      const date = new Date(alarmTime);
      await invoke("schedule_alarm", { isoTime: date.toISOString() });
      setStatus(`Alarm scheduled for ${date.toLocaleString()}`);
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  }

  async function cancelAlarm() {
    try {
      await invoke("cancel_alarm");
      setStatus("Alarm cancelled");
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  }

  async function selectFile() {
    const file = await open({
      multiple: false,
      directory: false,
      filters: [{ name: "Audio", extensions: ["mp3", "wav", "aiff", "m4a"] }],
    });
    if (file) {
      setFilePath(file as string);
    }
  }

  async function playTest() {
    try {
      await invoke("play_alarm", { filePath });
      setStatus("Playing alarm...");
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  }

  async function stopAlarm() {
    try {
      await invoke("stop_alarm");
      setStatus("Alarm stopped");
    } catch (error) {
      setStatus(`Error: ${error}`);
    }
  }

  if (isNightMode) {
    return (
      <div
        className="night-mode-overlay"
        onDoubleClick={() => setIsNightMode(false)}
      >
        <div className="night-mode-content">
          <h1>IronRise Active</h1>
          <p>Double-click to wake</p>
        </div>
      </div>
    );
  }

  return (
    <main className="container">
      <h1>IronRise Alarm</h1>

      <div className="card">
        <h2>1. Set Alarm Time</h2>
        <input
          type="datetime-local"
          value={alarmTime}
          onChange={(e) => setAlarmTime(e.target.value)}
        />
        <div className="row">
          <button onClick={scheduleAlarm}>Schedule Wake</button>
          <button onClick={cancelAlarm}>Cancel Schedule</button>
        </div>
      </div>

      <div className="card">
        <h2>2. Audio Settings</h2>
        <div className="row">
          <input
            type="text"
            value={filePath}
            readOnly
            placeholder="No file selected"
          />
          <button onClick={selectFile}>Browse...</button>
        </div>
        <div className="row">
          <button onClick={playTest}>Test Play (Max Vol)</button>
          <button onClick={stopAlarm}>Stop Audio</button>
        </div>
      </div>

      <div className="card">
        <h2>3. Night Mode</h2>
        <p>Turn screen black to prevent burn-in while keeping app active.</p>
        <button onClick={() => setIsNightMode(true)}>Enter Night Mode</button>
      </div>

      <div className="status-bar">
        <p>{status}</p>
      </div>

      <div className="warning-box">
        <h3>⚠️ Nightstand Protocol</h3>
        <ul>
          <li><strong>Plug it in:</strong> Do not rely on battery power.</li>
          <li><strong>Leave Lid OPEN:</strong> The system will NOT wake if the lid is closed.</li>
          <li><strong>Screen:</strong> Use Night Mode, do not sleep the machine manually.</li>
        </ul>
      </div>
    </main>
  );
}

export default App;
