use std::fs;
use std::path::{Path, PathBuf};

pub fn glob_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if dir.is_dir() {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    files.extend(glob_files(&path));
                    files.push(path);
                } else {
                    files.push(path);
                }
            }
        }
    }
    files
}
