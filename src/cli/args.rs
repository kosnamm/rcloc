use crate::FileInfo;
use clap::{Parser, builder::Str};
use std::{f64::consts::E, path::PathBuf};

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub commit: Option<String>,

    pub file_path: PathBuf,
}

pub fn parse_args() -> Args {
    Args::parse()
}

impl Args {
    pub fn validate(&self) -> Result<(), String> {
        if !self.file_path.exists() {
            return Err(format!("The path does not exist {:?}", self.file_path));
        }

        if let Some(c) = &self.commit {
            if c.trim().is_empty() {
                return Err("提交 ID 不能为空".to_string());
            }
        }
        Ok(())
    }

    pub fn to_file_info(&self) -> Result<FileInfo, String> {
        self.validate()?; // 必须先校验
        Ok(FileInfo::new(self.file_path.clone()))
    }
}
// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        let args = parse_args();
        assert!(args.commit.is_none());
        assert_eq!(args.file_path, PathBuf::from("src/cli/args.rs"));
    }
}
