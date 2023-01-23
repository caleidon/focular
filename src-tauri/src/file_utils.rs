use crate::foc_error::Result;
use audio_video_metadata::{get_format_from_file, Metadata};

pub fn get_image_dimensions(image_path: &str) -> Option<(i32, i32)> {
    match image::image_dimensions(image_path) {
        Ok(dimensions) => Some((dimensions.0 as i32, dimensions.1 as i32)),
        Err(_) => None,
    }
}

pub fn get_video_dimensions(video_path: &str) -> Option<(i32, i32)> {
    match get_format_from_file(video_path) {
        Ok(Metadata::Video(m)) => Some((m.dimensions.width as i32, m.dimensions.height as i32)),
        _ => None,
    }
}

pub fn get_duration(media_path: &str) -> Option<i32> {
    match get_format_from_file(media_path) {
        Ok(Metadata::Audio(m)) => Some(m.duration.unwrap_or_default().as_secs() as i32),
        _ => None,
    }
}

#[tauri::command]
pub fn get_file_size(file_path: &str) -> Result<u64> {
    let file_size = std::fs::metadata(file_path)?.len();
    Ok(file_size)
}
