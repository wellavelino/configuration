use std::process::Command;

pub fn sourceTmux() {
    Command::new("sh")
        .arg("tmux source ~/.tmux.conf")
        .output()
        .expect("")
}
