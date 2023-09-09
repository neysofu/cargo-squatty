# `cargo-squatty` ![GitHub](https://img.shields.io/github/license/neysofu/cargo-squatty) ![Crates.io](https://img.shields.io/crates/v/cargo-squatty) ![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/neysofu/cargo-squatty/ci.yml)

`cargo-squatty` helps you reserve names for your crates on crates.io.

## Installation

```text
$ cargo install cargo-squatty
```

## Usage

```text
$ cargo squatty --help
Helper tool to reserve names for your crates on https://crates.io

Usage: cargo squatty [OPTIONS] --crate-name <CRATE_NAME>

Options:
      --crate-name <CRATE_NAME>
          Name of the crate to publish
      --crate-description <CRATE_DESCRIPTION>
          Description of the crate to publish. Defaults to the crate name
  -y, --yes
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

**I strongly discourage the use of this tool to squat on crate names that you do not intend to use**. Please use this tool responsibly.

## Legal

`cargo-squatty` is licensed under the [MIT license](LICENSE).
