use derive_more::AsMut;
use std::process::Command;

#[derive(AsMut)]
pub struct Pacman(Command);

impl Pacman {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("sudo");
        cmd.args(["pacman", "-S", "--needed"]);
        Self(cmd)
    }
}
