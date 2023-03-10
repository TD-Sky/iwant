use package_manager_derive::PackageManager;
use std::process::Command;

#[derive(PackageManager)]
pub struct Npm(Command);

impl Npm {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("npm");
        cmd.arg("install");
        Self(cmd)
    }
}
