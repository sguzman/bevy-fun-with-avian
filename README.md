# bevy-fun-with-avian

---

## Installation

With Nix (recommended, using the flake):

```bash
# Build the package
nix build .#bevy-fun-with-avian

# Run the CLI
nix run .#bevy-fun-with-avian -- --help
```

Or enter the devshell:

```bash
nix develop
cargo run -- --help
```

Without Nix:

```bash
cargo build --release
./target/release/bevy-fun-with-avian --help
```

---

* `--verbosity <0|1|2>`: log level

  * `0`: warnings only
  * `1`: info (default)
  * `2`: debug

---

## Logging

Logs are timestamped and color-coded.
Verbosity is controlled with `--verbosity`:

* **0** → warnings only
* **1** → info (default, shows batch progress)
* **2** → debug (adds insert statements and SurrealDB responses)

---

## Development

### Devshell (Nix)

```bash
nix develop
```

Includes:

* Rust toolchain (`cargo`, `clippy`, `rustfmt`)
* `cargo-release` and `git-cliff` for semver and changelogs
* `mold` + `clang` for faster linking
* Nix tools (`alejandra`, `statix`, `deadnix`)

### Build and test

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

---

## [License](./LICENSE)

CC0-1.0 © 2025
