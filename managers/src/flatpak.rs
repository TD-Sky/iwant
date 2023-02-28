use derive_more::AsMut;
use std::process::Command;

#[derive(AsMut)]
pub struct Flatpak(Command);

impl Flatpak {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("flatpak");
        cmd.args(["install", "flathub"]);
        Self(cmd)
    }
}
