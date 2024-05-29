use std::borrow::Cow;
use std::cell::Cell;
use std::slice;
use std::str::FromStr;

use serde::Deserialize;
use smol_str::SmolStr;
use tabled::Tabled;

use crate::spec;

thread_local! {
    pub static CARGO_KIND: Cell<CargoKind> = const { Cell::new(CargoKind::Src) };
}

#[derive(Debug, Tabled)]
pub struct Item<'spec> {
    name: &'spec SmolStr,
    category: &'spec SmolStr,
    #[tabled(display_with("Self::display_packages", self))]
    packages: Option<&'spec [SmolStr]>,
    pub(super) manager: Manager,
    description: &'spec str,
}

impl<'spec> Item<'spec> {
    fn display_packages(&self) -> Cow<'spec, str> {
        match self.packages {
            Some(packages) => Cow::from(packages.join("\n")),
            None => Cow::from(self.name.as_str()),
        }
    }
}

impl<'spec> Item<'spec> {
    pub fn try_new(
        category: &'spec SmolStr,
        name: &'spec SmolStr,
        item: &'spec spec::Item,
    ) -> Result<Self, UnknownManager> {
        let item = match item {
            spec::Item::Desc(description) => Self {
                name,
                category,
                packages: None,
                manager: Manager::default(),
                description,
            },

            spec::Item::Info(info) => {
                let mut manager: Manager = info
                    .manager
                    .as_deref()
                    .map(|s| s.parse())
                    .transpose()?
                    .unwrap_or_default();

                // NOTE: We currently doesn't support '/' contained in `name` for cargo,
                //       so only check `packages`.
                if matches!(manager, Manager::Cargo(_))
                    && info
                        .packages
                        .as_ref()
                        .is_some_and(|ps| ps.iter().any(|p| p.contains('/')))
                {
                    manager = Manager::Cargo(CargoKind::Git);
                }

                Self {
                    category,
                    name,
                    packages: info.packages.as_deref(),
                    manager,
                    description: info.desc.as_deref().unwrap_or_default(),
                }
            }
        };

        Ok(item)
    }

    #[inline]
    pub fn packages(&self) -> &[SmolStr] {
        self.packages.unwrap_or(slice::from_ref(self.name))
    }
}

#[derive(Debug, Default, PartialEq, Eq, Deserialize)]
#[serde(try_from = "String")]
pub enum Manager {
    #[default]
    Pacman,
    Paru,
    Flatpak,
    Npm,
    Cargo(CargoKind),
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CargoKind {
    #[default]
    Src,
    Bin,
    #[serde(skip_deserializing)]
    Git,
}

#[derive(Debug, thiserror::Error)]
#[error("unknown package manager `{0}`")]
pub struct UnknownManager(String);

impl FromStr for Manager {
    type Err = UnknownManager;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Manager::*;
        let manager = match s {
            "pacman" => Pacman,
            "paru" => Paru,
            "flatpak" => Flatpak,
            "npm" => Npm,
            "cargo" => Cargo(CARGO_KIND.get()),
            "cargo:src" => Cargo(CargoKind::Src),
            "cargo:bin" => Cargo(CargoKind::Bin),
            _ => return Err(UnknownManager(s.to_owned())),
        };

        Ok(manager)
    }
}

impl TryFrom<String> for Manager {
    type Error = <Self as FromStr>::Err;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        s.parse()
    }
}

impl std::fmt::Display for Manager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Manager::*;
        let s = match self {
            Pacman => "pacman",
            Paru => "paru",
            Flatpak => "flatpak",
            Npm => "npm",
            Cargo(CargoKind::Src | CargoKind::Git) => "cargo",
            Cargo(CargoKind::Bin) => "cargo-binstall",
        };
        f.write_str(s)
    }
}
