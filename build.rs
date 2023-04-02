#[path = "src/cli.rs"]
mod cli;

use self::cli::Cli;
use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::{Bash, Fish, Zsh};
use clap_complete_nushell::Nushell;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let cmd = &mut Cli::command();
    let name = "iwant";
    let dir = "completions";

    fs::create_dir_all(dir)?;

    generate_to(Bash, cmd, name, dir)?;
    generate_to(Fish, cmd, name, dir)?;
    generate_to(Zsh, cmd, name, dir)?;
    generate_to(Nushell, cmd, name, dir)?;

    Ok(())
}
