use walkdir::WalkDir;

const VIDEO_EXTENSIONS: [&str; 6] = ["mp4", "mkv", "avi", "mov", "webm", "flv"];

#[tauri::command]
pub fn get_videos() -> Vec<String> {
    let mut videos = Vec::new();
    if let Some(home_dir) = dirs::home_dir() {
        for entry in WalkDir::new(home_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            if let Some(ext) = entry.path().extension().and_then(|s| s.to_str()) {
                if VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()) {
                    videos.push(entry.path().to_string_lossy().into_owned());
                }
            }
        }
    }
    videos
}