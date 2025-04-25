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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_find_ts_files() {
        // Create a temporary directory
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let dir_path = temp_dir.path();

        // Create some test files
        let ts_file = dir_path.join("test1.ts");
        let tsx_file = dir_path.join("test2.tsx");
        let js_file = dir_path.join("test3.js");
        let sub_dir = dir_path.join("sub");
        let nested_ts_file = sub_dir.join("nested.ts");

        fs::write(&ts_file, "// TypeScript file").unwrap();
        fs::write(&tsx_file, "// TSX file").unwrap();
        fs::write(&js_file, "// JavaScript file").unwrap();
        fs::create_dir(&sub_dir).unwrap();
        fs::write(&nested_ts_file, "// Nested TypeScript file").unwrap();

        // Call the function
        let mut found_files = find_ts_files(dir_path);

        // Normalize for easy testing (sort results)
        found_files.sort();

        let mut expected_files = vec![
            ts_file,
            tsx_file,
            nested_ts_file,
        ];
        expected_files.sort();

        println!("found_files: {:?}", found_files);
        println!("expected_files: {:?}", expected_files);

        assert_eq!(found_files, expected_files);
    }
}
