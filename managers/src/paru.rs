use derive_more::AsMut;
use std::process::Command;

#[derive(AsMut)]
pub struct Paru(Command);

impl Paru {
    #[inline]
    pub fn install() -> Self {
        let mut cmd = Command::new("paru");
        cmd.args(["-S", "--needed"]);
        Self(cmd)
    }
}
