use std::path::{Path, PathBuf};

pub struct FileInfo {
    pub file_path: PathBuf,

    pub files: Vec<PathBuf>,
}

impl FileInfo {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        let file_path = file_path.into();
        let mut files = Vec::new();
        Self::walk(&file_path, &mut files);
        Self { file_path, files }
    }
    fn walk(path: &Path, files: &mut Vec<PathBuf>) {
        if path.is_file() {
            files.push(path.to_path_buf());
        }

        if path.is_dir() {
            if path.file_name().map(|n| n == ".git").unwrap_or(false) {
                return;
            }
            for entry in path.read_dir().unwrap() {
                if let Ok(e) = entry {
                    let p = e.path();
                    Self::walk(&p, files);
                }
            }
        }
    }
}
