

use std::process::Command;
pub fn command(_command_line: String) {
    let _child = Command::new("cmd")
        .arg("/C")
        .arg(_command_line)
        .spawn()
        .expect("failed");
}
