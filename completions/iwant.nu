module completions {

  def "nu-complete iwant extra_managers" [] {
    [ "paru" "flatpak" "npm" "cargo" ]
  }

  # Install applications what I WANT.
  export extern iwant [
    manifest: string          # The manifest (.toml) of applications
    --categories(-C): string  # The specified categories [default: all]
    --exclude(-E): string     # The excluded categories
    --silent(-s)              # Don't display manifest
    --preview(-p)             # View the manifest without downloading
    --extra-managers(-m): string@"nu-complete iwant extra_managers" # Enable extra managers
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

}

use completions *
