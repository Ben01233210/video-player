use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{
    io::{BufRead, BufReader, Write},
    process::Command,
    time::Duration,
};
use tauri::{Emitter, Manager, AppHandle, State};

// Platform-specific imports for the IPC socket
#[cfg(windows)]
use std::fs::OpenOptions;
#[cfg(unix)]
use std::os::unix::net::UnixStream;

// --- Constants and Static State ---

#[cfg(windows)]
pub const IPC_PATH: &str = r"\\.\pipe\mpvsocket";
#[cfg(unix)]
pub const IPC_PATH: &str = "/tmp/mpvsocket";

// A global, thread-safe variable to hold the unique path to our IPC socket.
pub static IPC_PATH_ONCELOCK: std::sync::OnceLock<String> = std::sync::OnceLock::new();

// The properties we want mpv to notify us about when they change.
pub const OBSERVED_PROPERTIES: [&str; 7] = [
    "playlist", "filename", "pause", "eof-reached", "time-pos", "duration", "volume",
];


// --- Structs for Events ---

// This struct defines the shape of the events we'll send to the frontend.
#[derive(Clone, serde::Serialize)]
pub struct MpvEvent {
    event_type: String,
    name: Option<String>,
    data: Option<serde_json::Value>,
}

// --- Internal (Private) Functions ---

/// Internal helper to send a JSON command to the mpv socket.
/// This is the core communication function used by all our public commands.
fn _send_command(command_json: &str) -> Result<String, String> {
    let ipc_path = IPC_PATH_ONCELOCK.get().ok_or_else(|| "IPC path not set".to_string())?;

    #[cfg(unix)]
    let mut stream = UnixStream::connect(ipc_path)
        .map_err(|e| format!("Failed to connect to Unix socket: {}", e))?;
    
    #[cfg(windows)]
    let mut stream = OpenOptions::new().read(true).write(true).open(ipc_path)
        .map_err(|e| format!("Failed to open named pipe: {}", e))?;

    stream.write_all(command_json.as_bytes())
        .map_err(|e| format!("Failed to write command: {}", e))?;
    stream.write_all(b"\n")
        .map_err(|e| format!("Failed to write newline: {}", e))?;

    let mut reader = BufReader::new(stream);
    let mut response = String::new();
    reader.read_line(&mut response)
        .map_err(|e| format!("Failed to read response: {}", e))?;

    Ok(response)
}

/// The main event listener loop. Runs in a background thread.
fn mpv_event_listener(app_handle: AppHandle) {
    // Wait a moment for mpv to initialize the socket
    std::thread::sleep(Duration::from_secs(2));

    if let Some(ipc_path) = IPC_PATH_ONCELOCK.get() {
        // Platform-agnostic connection logic would be more complex,
        // for now we use cfg flags.
        #[cfg(unix)]
        let stream_result = UnixStream::connect(ipc_path);
        #[cfg(windows)]
        let stream_result = OpenOptions::new().read(true).write(true).open(ipc_path);

        if let Ok(mut stream) = stream_result {
            // Subscribe to property changes
            for (id, &property) in OBSERVED_PROPERTIES.iter().enumerate() {
                let cmd = format!(r#"{{"command": ["observe_property", {}, "{}"]}}"#, id + 1, property);
                let _ = stream.write_all(cmd.as_bytes());
                let _ = stream.write_all(b"\n");
            }

            // Read events line by line
            let reader = BufReader::new(stream);
            for line in reader.lines().filter_map(Result::ok) {
                if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(&line) {
                    if let Some(event_name) = json_value.get("event").and_then(|v| v.as_str()) {
                        let payload = MpvEvent {
                            event_type: event_name.to_string(),
                            name: json_value.get("name").and_then(|v| v.as_str()).map(String::from),
                            data: json_value.get("data").cloned(),
                        };
                        // Emit the structured event to the frontend
                        app_handle.emit_to("main", "mpv-event", &payload).unwrap();
                    }
                }
            }
        }
    }
}


// --- Public Tauri Commands ---

#[tauri::command]
pub fn play(path: String) -> Result<(), String> {
    // This sequence is more robust: pause, load, then unpause.
    _send_command(r#"{"command":["set_property","pause",true]}"#)?;
    let load_command = format!(r#"{{"command":["loadfile","{}"]}}"#, path.replace('\\', "/"));
    _send_command(&load_command)?;
    _send_command(r#"{"command":["set_property","pause",false]}"#)?;
    Ok(())
}

#[tauri::command]
pub fn pause() -> Result<(), String> {
    _send_command(r#"{"command":["set_property","pause",true]}"#).map(|_|())
}

#[tauri::command]
pub fn resume() -> Result<(), String> {
    _send_command(r#"{"command":["set_property","pause",false]}"#).map(|_|())
}

#[tauri::command]
pub fn seek(time_in_seconds: f64) -> Result<(), String> {
    let command = format!(r#"{{"command":["seek",{},"absolute"]}}"#, time_in_seconds);
    _send_command(&command).map(|_|())
}

// --- Setup Function ---

/// This is the main setup function called from `main.rs`.
pub fn setup_mpv(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let webview_window = app.get_webview_window("main").unwrap();
    let app_handle = app.handle().clone();
    let handle = webview_window.window_handle()?.as_raw();
    
    let window_handle = match handle {
        RawWindowHandle::Win32(handle) => handle.hwnd.get() as i64,
        RawWindowHandle::Xlib(handle) => handle.window as i64,
        RawWindowHandle::Wayland(handle) => {
            let i = unsafe {handle.surface.read()};
            i as i64
        }
        // Add other platforms as needed
        _ => return Err("Unsupported window handle type".into()),
    };

    let ipc_path = format!("{}_{}", IPC_PATH, window_handle);
    IPC_PATH_ONCELOCK.set(ipc_path.clone()).unwrap();

    // Launch mpv with the new high-quality profile
    Command::new("mpv")
        .args([
            &format!("--wid={}", window_handle),
            &format!("--input-ipc-server={}", ipc_path),
            "--idle=yes",
            "--no-osc",
            "--no-border",
            // --- THE FIX IS HERE ---
            // This flag enables a suite of high-quality rendering options.
            "--profile=gpu-hq",
        ])
        .spawn()?;

    // The rest of the function remains the same...
    std::thread::spawn(move || {
        mpv_event_listener(app_handle);
    });

    Ok(())
}

// (This file is largely the same as the one from the previous prompt,
// but with new commands for volume and fullscreen. I will only show the new parts
// and how they integrate for brevity. Assume the rest of the mpv.rs code is there.)

// --- Add "volume" to your OBSERVED_PROPERTIES constant ---

// --- Add these new Tauri Commands to the file ---

#[tauri::command]
pub fn set_volume(volume: f64) -> Result<(), String> {
    // mpv volume is 0-100. We'll ensure it's within bounds.
    let clamped_volume = volume.max(0.0).min(100.0);
    let command = format!(r#"{{"command":["set_property","volume",{}]}}"#, clamped_volume);
    _send_command(&command).map(|_|()) // .map(|_|()) discards the "ok" response string
}

#[tauri::command]
pub fn toggle_fullscreen(window: tauri::Window) -> Result<(), String> {
    let is_fullscreen = window.is_fullscreen().map_err(|e| e.to_string())?;
    window.set_fullscreen(!is_fullscreen).map_err(|e| e.to_string())
}

// Add this function alongside your play, pause, seek, etc. commands

#[tauri::command]
pub fn stop() -> Result<(), String> {
    // The "stop" command in mpv unloads the current file and stops playback.
    _send_command(r#"{"command":["stop"]}"#).map(|_|())
}
