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
- [x] cargo-binstall
- [x] npm

Because I'm an Arch Linux user, so the default manager is hard coded with `pacman` ;) .



## Manifest

The manifest of **iwant** is a toml file. It has the following structure:

```toml
[-]
cargo = "src"

[category0]
itemA = { packages = [], manager = "", desc = "" }

[category1]
# Or only explicitly states the
# *name* (here is "itemB") and
# *desc* (here is "itemB description").
itemB = "itemB description"

# ...more categories
```

### Global options

The `[-]` section is global options of *iwant* and it is optional. *iwant* will use default options when `[-]` isn't found.

| Key | Possible Values | Default | Description |
| :-----------: | :------------: | :------------: | :------------- |
| cargo | `src`, `bin` | `src` | Control the command used by manager `cargo` |

### Item

The `item` later would be translated into

|             | name | category |  packages   | manager | description
|:-----------:|:----:|:--------:|:-----------:|:-------:|:-----------:
| **Default** |  --  |    --    | `item.name` |  pacman |     ""

When you don't state the keys explicitly, they would have default values as above.

### Manager

Some managers have variants:

- cargo:

  | Value | Description |
  | -----------: | :------------- |
  | `cargo` | Executes the command specified by global key `cargo` |
  | `cargo:src` | Executes `cargo install` |
  | `cargo:bin` | Executes `cargo binstall` |

  In addition, `<owner>/<project>` is treated as github url path for `cargo`. For example, *iwant* will translate the following item

  ```toml
  iwant = { packages = ["TD-Sky/iwant"], manager = "cargo", desc = "Install applications what I WANT" }
  ```

  into `cargo install --git 'https://github.com/TD-Sky/iwant'`.
  > The command that installs from git  isn't affected by the global key `cargo`.



## Usage

Please `iwant -h` and then read the help.

You can also refer to [my manifest](https://github.com/TD-Sky/dotfiles/blob/main/Templates/apps.toml).



## LICENSE

The MIT License ([MIT](https://opensource.org/license/mit/))
