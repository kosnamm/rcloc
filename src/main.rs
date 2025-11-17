use Rcloc::FileInfo;

fn main() {
    let file_info = FileInfo::new("../src");
    println!("File name: {:?}", file_info.file_path);
    println!("File size: {:?}", file_info.files);
}
