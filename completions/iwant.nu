module completions {

  def "nu-complete iwant extra_managers" [] {
    [ "paru" "flatpak" "npm" "cargo" ]
  }

  # Install applications what I WANT.
  export extern iwant [
    manifest: string          # The manifest of applications
    --categories(-C): string  # The specified categories [delimiter: ,] [default: all]
    --exclude(-E): string     # The excluded categories [delimiter: ,]
    --silent(-s)              # Don't display manifest
    --preview(-p)             # View the manifest without downloading
    --extra-managers(-m): string@"nu-complete iwant extra_managers" # Enable extra managers [delimiter: ,]
    --help(-h)                # Print help
    --version(-V)             # Print version
  ]

}

export use completions *
