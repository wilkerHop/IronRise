import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [alarmTime, setAlarmTime] = useState("");
  const [status, setStatus] = useState("");
  const [filePath, setFilePath] = useState("/System/Library/Sounds/Glass.aiff");

  async function scheduleAlarm() {
    try {
      // Convert local time to ISO string for the backend
      // We just take the input value (e.g. "2023-10-27T10:00") and append seconds/timezone if needed
      // But input type="datetime-local" gives "YYYY-MM-DDTHH:mm"
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
        <input
          type="text"
          value={filePath}
          onChange={(e) => setFilePath(e.target.value)}
          placeholder="/path/to/sound.mp3"
        />
        <div className="row">
          <button onClick={playTest}>Test Play (Max Vol)</button>
          <button onClick={stopAlarm}>Stop Audio</button>
        </div>
      </div>

      <div className="status-bar">
        <p>{status}</p>
      </div>

      <div className="warning-box">
        <h3>⚠️ Nightstand Protocol</h3>
        <ul>
          <li><strong>Plug it in:</strong> Do not rely on battery power.</li>
          <li><strong>Leave Lid OPEN:</strong> The system will NOT wake if the lid is closed.</li>
          <li><strong>Screen:</strong> You may dim the screen, but do not sleep the machine manually.</li>
        </ul>
      </div>
    </main>
  );
}

export default App;
