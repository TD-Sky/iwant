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

pub fn flatpak() -> Command {
    let mut cmd = Command::new("flatpak");
    cmd.args(["install", "flathub"]);
    cmd
}

#[derive(Debug)]
pub struct Cargo(Command);

impl Cargo {
    pub fn new() -> Self {
        Self(Command::new("cargo"))
    }

    pub fn install(&mut self) -> &mut Command {
        self.0.arg("install")
    }

    pub fn binstall(&mut self) -> &mut Command {
        self.0.arg("binstall")
    }

    pub fn git(&mut self, package: &str) -> &mut Command {
        let url = format!("https://github.com/{package}");
        self.0.args(["install", "--git", &url])
    }
}
