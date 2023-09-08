# Cargo Squatty

Squatty helps you reserve names for your crates on <crates.io>.

## Installation

```bash
$ cargo install cargo-squatty
```

## Usage

```bash
$ cargo squatty --help
Easily reserve a crate name on <crates.io>

Usage: squatty [OPTIONS] --crate-name <CRATE_NAME>

Options:
  -c, --crate-name <CRATE_NAME>
          Name of the crate to publish
  -s, --skip-confirmation
          Skip the confirmation prompt
  -o, --cargo-publish-opts <CARGO_PUBLISH_OPTS>
          Options to pass along to `cargo publish` e.g. `--dry-run`
  -h, --help
          Print help
  -V, --version
          Print version

$ cargo squatty --crate-name test-empty-crate-foobar
Do you want to publish the crate? [y/N] y
    Updating crates.io index
warning: manifest has no documentation, homepage or repository.
See https://doc.rust-lang.org/cargo/reference/manifest.html#package-metadata for more info.
   Packaging test-empty-crate-foobar v0.0.0-reserved (/tmp/.tmpzULWEW)
   Verifying test-empty-crate-foobar v0.0.0-reserved (/tmp/.tmpzULWEW)
   Compiling test-empty-crate-foobar v0.0.0-reserved (/tmp/.tmpzULWEW/target/package/test-empty-crate-foobar-0.0.0-reserved)
    Finished dev [unoptimized + debuginfo] target(s) in 0.17s
    Packaged 4 files, 783.0B (614.0B compressed)
   Uploading test-empty-crate-foobar v0.0.0-reserved (/tmp/.tmpzULWEW)
```

## Abuse

We strongly discourage the use of this tool to squat on crate names that you do not intend to use. Please use this tool responsibly.

## Legal

Squatty is licensed under the [MIT license](LICENSE).
