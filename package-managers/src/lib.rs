mod cargo;
mod flatpak;
mod npm;
mod pacman;
mod paru;

pub use self::cargo::Cargo;
pub use self::flatpak::Flatpak;
pub use self::npm::Npm;
pub use self::pacman::Pacman;
pub use self::paru::Paru;

use std::ffi::OsStr;
use std::io;
use std::process::ExitStatus;

pub trait PackageManager {
    fn arg(&mut self, s: impl AsRef<OsStr>) -> &mut Self;
    fn args<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(&mut self, args: I) -> &mut Self;
    fn execute(&mut self) -> io::Result<ExitStatus>;
}
