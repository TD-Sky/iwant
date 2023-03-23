<div align="center">

# iwant

**Install applications what I WANT.**

</div>



## About

**iwant** is a tool to install applications from provided manifest(.toml) using the supported package managers.



## Installation

Install from source code:

```bash
$ cargo install --path .
```

Install from **AUR**:

```bash
$ paru -S iwant
```



## Supported Package Managers

- [x] pacman
- [x] paru
- [x] flatpak (name allowed only: `flathub`)
- [x] cargo
- [x] npm

Because I'm a Arch Linux user, so the default manager is hard coded with `pacman` ;) .



## Manifest

The manifest of **iwant** is a toml file. It has the following structure:

```toml
[category0]
itemA = { packages = [], manager = "", description = "" }

[category1]
# Or only explicitly states the
# *name* (here is "itemB") and
# *description* (here is "itemB description").
itemB = "itemB description"

# ...more categories
```

The `item` later would be translated into

|             | name | category |  packages   | manager | description
|:-----------:|:----:|:--------:|:-----------:|:-------:|:-----------:
| **Default** |  --  |    --    | `item.name` |  pacman |     ""

When you don't state the keys explicitly, they would have default values as above.



## Usage

Please `iwant -h` and then read the help.

You can also refer to [my manifest](https://github.com/TD-Sky/dotfiles/blob/main/apps.toml).



## LICENSE

The MIT License ([MIT](https://opensource.org/licenses/MIT))
