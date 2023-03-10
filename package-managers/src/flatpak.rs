use package_manager_derive::PackageManager;
use std::process::Command;

#[derive(PackageManager)]
pub struct Flatpak(Command);

impl Flatpak {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("flatpak");
        cmd.args(["install", "flathub"]);
        Self(cmd)
    }
}
