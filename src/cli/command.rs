use super::args::Args;
use std::path::PathBuf;
// pub struct Command {
//     pub file_path: String,
//     pub commit_id: String,
// }
// // impl from command

// impl Command {
//     pub fn new(commit_id: String, file_path: String) -> Self {
//         Command {
//             file_path,
//             commit_id,
//         }
//     }
// }

#[derive(Debug)]
pub struct Command {
    pub commit: Option<String>,
    pub root: PathBuf,
}

impl From<Args> for Command {
    fn from(args: Args) -> Self {
        Self {
            commit: args.commit,
            root: args.file_path,
        }
    }
}
