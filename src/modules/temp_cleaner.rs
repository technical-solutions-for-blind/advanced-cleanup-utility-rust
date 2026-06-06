use anyhow::Result;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;
use log::{info, warn};

pub struct TempCleaner;

impl TempCleaner {
    pub fn clean_windows_temp() -> Result<(u32, u64)> {
        let path = "C:\\Windows\\Temp";
        Self::clean_path(path)
    }

    pub fn clean_user_temp() -> Result<(u32, u64)> {
        let path = std::env::var("TEMP")?;
        Self::clean_path(&path)
    }

    fn clean_path(path: &str) -> Result<(u32, u64)> {
        let mut file_count = 0u32;
        let mut bytes_freed = 0u64;

        info!("Starting cleanup of: {}", path);

        if !Path::new(path).exists() {
            return Ok((0, 0));
        }

        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.path().is_file() {
                if let Ok(metadata) = fs::metadata(entry.path()) {
                    bytes_freed += metadata.len();
                    match fs::remove_file(entry.path()) {
                        Ok(_) => file_count += 1,
                        Err(e) => warn!("Failed to delete: {}", e),
                    }
                }
            }
        }

        info!("Cleanup: {} files, {} MB", file_count, bytes_freed / 1024 / 1024);
        Ok((file_count, bytes_freed))
    }
}
