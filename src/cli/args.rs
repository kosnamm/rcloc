use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Args {
    #[arg(short, long)]
    pub commit: Option<String>,

    pub file_path: PathBuf,
}

pub fn parse_args() -> Args {
    Args::parse()
}
