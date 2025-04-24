
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

/// Recursively find all .ts and .tsx files in a directory
pub fn find_ts_files(entry: &Path) -> Vec<PathBuf> {
    WalkDir::new(entry)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            let path = e.path();
            matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("ts" | "tsx")
            )
        })
        .map(|e| e.into_path())
        .collect()
}
