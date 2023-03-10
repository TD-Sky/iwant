use package_manager_derive::PackageManager;
use std::process::Command;

#[derive(PackageManager)]
pub struct Cargo(Command);

impl Cargo {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("cargo");
        cmd.arg("install");
        Self(cmd)
    }
}
