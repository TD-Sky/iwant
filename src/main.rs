mod cli;
mod item;
mod render;
mod spec;

use self::cli::Cli;
use self::cli::ExtraManager;
use self::item::Item;
use self::spec::Manifest;
use anyhow::Result;
use clap::Parser;
use item::Manager;
use package_managers::*;
use smol_str::SmolStr;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut manifest = String::new();
    let mut fd = File::open(&cli.manifest)?;
    fd.read_to_string(&mut manifest)?;
    let mut manifest: Manifest = basic_toml::from_str(&manifest)?;

    retain_categories(&mut manifest, &cli.categories, &cli.exclude);

    let mut items = Vec::new();
    for (cname, categroy) in manifest.iter() {
        for (name, item) in categroy.iter() {
            let item = Item::try_new(cname, name, item)?;
            items.push(item);
        }
    }

    if !cli.silent {
        let table = render::to_table(&items);
        println!("{table}");
    }

    if cli.preview {
        return Ok(());
    }

    let packages = select_packages(&items, Manager::Pacman);
    Pacman::install().args(packages).execute()?;

    if cli.extra_managers.contains(&ExtraManager::Paru) {
        let packages = select_packages(&items, Manager::Paru);
        Paru::install().args(packages).execute()?;
    }

    if cli.extra_managers.contains(&ExtraManager::Npm) {
        let packages = select_packages(&items, Manager::Npm);
        Npm::install().args(packages).execute()?;
    }

    if cli.extra_managers.contains(&ExtraManager::Cargo) {
        let packages = select_packages(&items, Manager::Cargo);
        Cargo::install().args(packages).execute()?;
    }

    if cli.extra_managers.contains(&ExtraManager::Flatpak) {
        let packages = select_packages(&items, Manager::Flatpak);
        Flatpak::install().args(packages).execute()?;
    }

    Ok(())
}

fn retain_categories(manifest: &mut Manifest, includeds: &[SmolStr], excludeds: &[SmolStr]) {
    let including = !includeds.is_empty();
    let excluding = !excludeds.is_empty();

    if including && excluding {
        manifest
            .retain(|category, _| includeds.contains(category) && !excludeds.contains(category));
    } else if including {
        manifest.retain(|category, _| includeds.contains(category));
    } else if excluding {
        manifest.retain(|category, _| !excludeds.contains(category));
    }
}

fn select_packages<'spec>(
    items: &'spec [Item<'spec>],
    manager: Manager,
) -> impl Iterator<Item = &'spec str> {
    items
        .iter()
        .filter(move |&item| item.manager == manager)
        .flat_map(|item| item.packages())
        .map(|p| p.as_str())
}
