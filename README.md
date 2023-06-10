# password_generator
A password generator that uses hashes so that you don't lose your passwords.

```
# pg facebook
Enter your passphrase:
This is your password: |c1iZUJxc (obviously not my password lol)
```

```
# pg --help
Usage: run [OPTIONS] [args]...

Arguments:
  [args]...

Options:
  -q, --quiet                   Do not print cargo log messages
      --bin [<NAME>]            Name of the bin target to run
      --example [<NAME>]        Name of the example target to run
  -p, --package [<SPEC>]        Package with the target to run
  -v, --verbose...              Use verbose output (-vv very verbose/build.rs output)
  -j, --jobs <N>                Number of parallel jobs, defaults to # of CPUs
      --color <WHEN>            Coloring: auto, always, never
      --keep-going              Do not abort the build as soon as there is an error (unstable)
      --frozen                  Require Cargo.lock and cache are up to date
  -r, --release                 Build artifacts in release mode, with optimizations
      --locked                  Require Cargo.lock is up to date
      --profile <PROFILE-NAME>  Build artifacts with the specified profile
  -F, --features <FEATURES>     Space or comma separated list of features to activate
      --all-features            Activate all available features
      --config <KEY=VALUE>      Override a configuration value
      --no-default-features     Do not activate the `default` feature
  -Z <FLAG>                     Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
      --target <TRIPLE>         Build for the target triple
      --target-dir <DIRECTORY>  Directory for all generated artifacts
      --manifest-path <PATH>    Path to Cargo.toml
      --message-format <FMT>    Error format
      --unit-graph              Output build graph in JSON (unstable)
      --ignore-rust-version     Ignore `rust-version` specification in packages
      --timings[=<FMTS>]        Timing output formats (unstable) (comma separated): html, json
  -h, --help                    Print help information
```
