use std::{
    env,
    ffi::OsString,
    fs,
    io::{self, Write},
    path::Path,
    process::{Command, ExitStatus},
};

use clap::{Parser, Subcommand};
use tempfile::TempDir;

/// This binary should be invoked as `cargo squatty` (in which case
/// this message will not be seen), not `cargo-squatty`.
#[derive(Debug, Parser)]
#[command(version, bin_name = "cargo")]
pub struct CargoSquattyApp {
    #[clap(subcommand)]
    subcommand: SquattySubcommand,
}

#[derive(Debug, Subcommand)]
enum SquattySubcommand {
    #[clap(name = "squatty")]
    Squatty(Args),
}

/// Easily reserve a crate name on <crates.io>
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the crate to publish
    #[arg(short, long)]
    crate_name: String,

    /// Skip the confirmation prompt
    #[arg(short, long, default_value = "false")]
    skip_confirmation: bool,

    /// Options to pass along to `cargo publish` e.g. `--dry-run`
    #[arg(short = 'o', long, allow_hyphen_values = true)]
    cargo_publish_opts: Option<String>,
}

fn main() {
    let args = match CargoSquattyApp::parse().subcommand {
        SquattySubcommand::Squatty(args) => args,
    };

    let confirmation = if args.skip_confirmation {
        Confirmation::Yes
    } else {
        ask_for_confirmation().expect("Failed to read user input. Aborting.")
    };

    match confirmation {
        Confirmation::Yes => (),
        Confirmation::No => panic!("User selected 'no'. Aborting"),
        Confirmation::UnrecognizedInput => panic!("Invalid input. Aborting"),
    }

    let source_dir =
        create_tmpdir_with_crate_sources(&args.crate_name).expect("Failed to create tmpdir");

    let cargo_publish_opts = args
        .cargo_publish_opts
        .as_deref()
        .map(|s| s.split(' ').collect::<Vec<_>>())
        .unwrap_or_default();
    let exit_status = run_cargo_publish(source_dir.path(), &cargo_publish_opts);
    if !exit_status.success() {
        panic!("Failed to publish crate");
    }
}

fn run_cargo_publish(path: &Path, cargo_publish_opts: &[&str]) -> ExitStatus {
    env::set_current_dir(path).expect("Failed to change directory");

    let mut args = vec!["publish"];
    args.extend_from_slice(cargo_publish_opts);

    Command::new(cargo_path())
        .args(args)
        .status()
        .expect("Failed to run cargo publish")
}

fn create_tmpdir_with_crate_sources(crate_name: &str) -> io::Result<TempDir> {
    let dir = tempfile::tempdir()?;

    fn generate_cargo_toml_contents(crate_name: &str, description: &str) -> String {
        const TEMPLATE: &str = include_str!("resources/Cargo.toml.template");
        TEMPLATE
            .replace("{{PACKAGE_NAME}}", crate_name)
            .replace("{{DESCRIPTION}}", description)
    }

    fs::write(
        dir.path().join("Cargo.toml"),
        generate_cargo_toml_contents(crate_name, crate_name),
    )?;
    fs::write(dir.path().join("lib.rs"), "")?;
    fs::write(dir.path().join("LICENSE"), "")?;

    Ok(dir)
}

enum Confirmation {
    Yes,
    No,
    UnrecognizedInput,
}

fn ask_for_confirmation() -> io::Result<Confirmation> {
    print!("Do you want to publish the crate? [y/N] ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => Ok(Confirmation::Yes),
        "n" | "no" => Ok(Confirmation::No),
        _ => Ok(Confirmation::UnrecognizedInput),
    }
}

fn cargo_path() -> OsString {
    std::env::var_os("CARGO").unwrap_or_else(|| OsString::from("cargo"))
}
