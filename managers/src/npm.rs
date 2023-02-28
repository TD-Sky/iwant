use derive_more::AsMut;
use std::process::Command;

#[derive(AsMut)]
pub struct Npm(Command);

impl Npm {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("npm");
        cmd.arg("install");
        Self(cmd)
    }
}
