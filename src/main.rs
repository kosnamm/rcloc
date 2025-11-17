use Rcloc::cli::args::parse_args;
use Rcloc::cli::command::Command;

fn main() {
    let args = parse_args();

    let cmd = Command::from(args);
    // let command = Command::from(commit, root);
    println!("commit id {:?}", cmd.commit);
    println!("file path{:?}", cmd.root);
}
