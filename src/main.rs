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
use managers::*;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut manifest = String::new();
    let mut fd = File::open(&cli.manifest)?;
    fd.read_to_string(&mut manifest)?;
    let mut manifest: Manifest = toml::from_str(&manifest)?;

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
    if !packages.is_empty() {
        Pacman::install().args(&packages).execute()?;
    }

    if cli.extra_managers.contains(&ExtraManager::Paru) {
        let packages = select_packages(&items, Manager::Paru);
        Paru::install().args(&packages).execute()?;
    }

    if cli.extra_managers.contains(&ExtraManager::Flatpak) {
        let packages = select_packages(&items, Manager::Flatpak);
        Flatpak::install().args(&packages).execute()?;
    }

    if cli.extra_managers.contains(&ExtraManager::Npm) {
        let packages = select_packages(&items, Manager::Npm);
        Npm::install().args(&packages).execute()?;
    }

    Ok(())
}

fn retain_categories(manifest: &mut Manifest, includeds: &[String], excludeds: &[String]) {
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

fn select_packages<'spec>(items: &'spec [Item<'spec>], manager: Manager) -> Vec<&'spec String> {
    items
        .iter()
        .filter(|&item| item.manager == manager)
        .fold(Vec::new(), |mut packages, item| {
            if let Some(pkgs) = item.packages {
                packages.extend(pkgs.iter());
            } else {
                packages.push(item.name);
            }

            packages
        })
}
