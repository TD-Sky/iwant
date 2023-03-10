use package_manager_derive::PackageManager;
use std::process::Command;

#[derive(PackageManager)]
pub struct Paru(Command);

impl Paru {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("paru");
        cmd.args(["-S", "--needed"]);
        Self(cmd)
    }
}
