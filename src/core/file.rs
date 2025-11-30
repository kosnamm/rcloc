use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, BufReader},
    path::{Path, PathBuf},
};

pub struct FileInfo {
    pub file_path: PathBuf,
    pub files: Vec<PathBuf>,
}

#[derive(Debug, Default)]
pub struct LangStat {
    pub name: String,
    pub files: u32,
    pub code: u64,
    pub comments: u64,
    pub blanks: u64,
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

    // pub fn cal_with_commit_id(&self, commit_id: String) {}
    pub fn cal_without_commit_id(&self) -> io::Result<()> {
        let mut stats: HashMap<String, LangStat> = HashMap::new();

        for path in &self.files {
            if !path.is_file() {
                continue;
            }
            let ext = path
                .extension()
                .and_then(|s| s.to_str())
                .unwrap_or("unknown")
                .to_string();
            let entry = stats.entry(ext.clone()).or_insert_with(|| LangStat {
                name: ext.clone(),
                ..Default::default()
            });
            entry.files += 1;
            let file = File::open(path)?;
            let reader = BufReader::new(file);

            let comment_prefixes = comment_prefixes_for_ext(&ext);

            for line in reader.lines() {
                let line = line?;
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    entry.blanks += 1;
                } else if is_comment_line(trimmed, &comment_prefixes) {
                    entry.comments += 1;
                } else {
                    entry.code += 1;
                }
            }
        }
        println!(
            "{:<12} {:>6} {:>8} {:>8} {:>8}",
            "Ext", "Files", "Code", "Comment", "Blank"
        );
        println!("{}", "-".repeat(50));
        let mut vals: Vec<_> = stats.values().collect();
        vals.sort_by_key(|s| &s.name);

        for s in vals {
            println!(
                "{:<12} {:>6} {:>8} {:>8} {:>8}",
                s.name, s.files, s.code, s.comments, s.blanks
            );
        }
        Ok(())
    }
}

// common functions
fn comment_prefixes_for_ext(ext: &str) -> Vec<&'static str> {
    match ext {
        "c" | "h" | "cpp" | "hpp" | "cc" | "cxx" | "rs" | "java" | "kt" | "swift" | "go" => {
            vec!["//"]
        }
        "py" | "sh" | "bash" | "zsh" | "rb" | "pl" | "r" | "toml" | "yaml" | "yml" | "nim" => {
            vec!["#"]
        }
        "html" | "xml" | "xsd" | "xsl" | "xslt" => vec!["<!--"],
        _ => Vec::new(),
    }
}

fn is_comment_line(line: &str, prefixes: &[&str]) -> bool {
    for p in prefixes {
        if line.starts_with(p) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_file_info() {
        let file_info = FileInfo::new("src/core/");
        assert_eq!(file_info.file_path, PathBuf::from("src/core/"));
        println!("Files: {:?}", file_info.files);
        assert_eq!(file_info.files.len(), 2);
    }

    #[test]
    fn test_cal_without_commit() {
        let file_info = FileInfo::new("src/");
        let _ = file_info.cal_without_commit_id();
    }
}
