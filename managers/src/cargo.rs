use derive_more::AsMut;
use std::process::Command;

#[derive(AsMut)]
pub struct Cargo(Command);

impl Cargo {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("cargo");
        cmd.arg("install");
        Self(cmd)
    }
}
