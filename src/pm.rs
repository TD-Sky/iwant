use std::process::Command;

pub fn pacman() -> Command {
    let mut cmd = Command::new("pacman");
    cmd.args(["-S", "--needed"]);
    cmd
}

pub fn paru() -> Command {
    let mut cmd = Command::new("paru");
    cmd.args(["-S", "--needed"]);
    cmd
}

pub fn npm() -> Command {
    let mut cmd = Command::new("npm");
    cmd.args(["install", "-g"]);
    cmd
}

pub fn cargo() -> Command {
    let mut cmd = Command::new("cargo");
    cmd.arg("install");
    cmd
}

pub fn flatpak() -> Command {
    let mut cmd = Command::new("flatpak");
    cmd.args(["install", "flathub"]);
    cmd
}
