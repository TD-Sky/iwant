mod cli;
mod item;
mod iter;
mod pm;
mod render;
mod spec;

use std::fs::File;
use std::io::Read;

use anyhow::Result;
use clap::Parser;
use smol_str::SmolStr;

use self::{
    cli::{Cli, ExtraManager},
    item::{CargoKind, Item, Manager, CARGO_KIND},
    iter::PeekExt,
    pm::*,
    spec::Manifest,
};

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut manifest = String::new();
    let mut fd = File::open(&cli.manifest)?;
    fd.read_to_string(&mut manifest)?;
    let mut manifest: Manifest = basic_toml::from_str(&manifest)?;
    CARGO_KIND.set(manifest.options.cargo);

    retain_categories(&mut manifest, &cli.categories, &cli.exclude);

    let mut items = Vec::new();
    for (cname, categroy) in &manifest.categories {
        for (name, item) in categroy {
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

    fn unit<T>(_: T) {}

    select_packages(&items, Manager::Pacman)
        .on_some(|packages| pacman().args(packages).status().map(unit))?;

    if cli.extra_managers.contains(&ExtraManager::Paru) {
        select_packages(&items, Manager::Paru)
            .on_some(|packages| paru().args(packages).status().map(unit))?;
    }

    if cli.extra_managers.contains(&ExtraManager::Npm) {
        select_packages(&items, Manager::Npm)
            .on_some(|packages| npm().args(packages).status().map(unit))?;
    }

    if cli.extra_managers.contains(&ExtraManager::Cargo) {
        select_packages(&items, Manager::Cargo(CargoKind::Bin))
            .on_some(|packages| Cargo::new().binstall().args(packages).status().map(unit))?;

        select_packages(&items, Manager::Cargo(CargoKind::Src))
            .on_some(|packages| Cargo::new().install().args(packages).status().map(unit))?;

        for p in select_packages(&items, Manager::Cargo(CargoKind::Git)) {
            Cargo::new().git(p).status()?;
        }
    }

    if cli.extra_managers.contains(&ExtraManager::Flatpak) {
        select_packages(&items, Manager::Flatpak)
            .on_some(|packages| flatpak().args(packages).status().map(unit))?;
    }

    Ok(())
}

fn retain_categories(manifest: &mut Manifest, includeds: &[SmolStr], excludeds: &[SmolStr]) {
    let including = !includeds.is_empty();
    let excluding = !excludeds.is_empty();

    if including && excluding {
        manifest
            .categories
            .retain(|category, _| includeds.contains(category) && !excludeds.contains(category));
    } else if including {
        manifest
            .categories
            .retain(|category, _| includeds.contains(category));
    } else if excluding {
        manifest
            .categories
            .retain(|category, _| !excludeds.contains(category));
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
        .peekable()
}
