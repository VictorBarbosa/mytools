
use std::process::Command;
pub fn command(_command_line: String) {
    let _child = Command::new("sh")
        .arg("-c")
        .arg(_command_line)
        .spawn()
        .expect("failed");
}