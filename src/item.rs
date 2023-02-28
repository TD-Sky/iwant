use crate::spec;
use parse_display::Display;
use serde::Deserialize;
use std::borrow::Cow;
use std::str::FromStr;
use tabled::Tabled;

#[derive(Debug, Tabled)]
pub struct Item<'spec> {
    pub name: &'spec String,
    pub category: &'spec String,
    #[tabled(display_with("Self::display_packages", args))]
    pub packages: Option<&'spec [String]>,
    pub manager: Manager,
    pub description: &'spec str,
}

#[derive(Debug, Default, PartialEq, Eq, Display, Deserialize)]
#[display(style = "lowercase")]
pub enum Manager {
    #[default]
    Pacman,
    Paru,
    Flatpak,
    Npm,
    Cargo,
}

impl<'spec> Item<'spec> {
    pub fn try_new(
        category: &'spec String,
        name: &'spec String,
        item: &'spec spec::Item,
    ) -> Result<Self, UnknownManager> {
        let item = match item {
            spec::Item::Description(description) => Self {
                name,
                category,
                packages: None,
                manager: Manager::default(),
                description,
            },

            spec::Item::Info(info) => Self {
                category,
                name,
                packages: info.packages.as_deref(),
                manager: info
                    .manager
                    .as_deref()
                    .map(|s| s.parse())
                    .transpose()?
                    .unwrap_or_default(),
                description: info.description.as_deref().unwrap_or_default(),
            },
        };

        Ok(item)
    }
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
            "cargo" => Cargo,
            _ => return Err(UnknownManager(s.to_owned())),
        };

        Ok(manager)
    }
}

impl<'spec> Item<'spec> {
    fn display_packages(&self) -> Cow<'spec, str> {
        match self.packages {
            Some(packages) => Cow::from(packages.join(" ")),
            None => Cow::from(self.name),
        }
    }
}
