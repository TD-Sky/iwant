use package_manager_derive::PackageManager;
use std::process::Command;

#[derive(PackageManager)]
pub struct Pacman(Command);

impl Pacman {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("sudo");
        cmd.args(["pacman", "-S", "--needed"]);
        Self(cmd)
    }
}
