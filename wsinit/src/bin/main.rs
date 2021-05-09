use std::env;

use clap::{App, Arg, ArgMatches};
use colored::*;

use cargo_wsinit::{Error, FileExistsBehaviour, Options, Workspace};

macro_rules! wsinit {
    () => {
        "wsinit"
    };
}

fn main() {
    let (args, cargo) = get_command_line_args(wsinit!());
    let app = App::new("Cargo Workspace Init")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("overwrite")
                .short("o")
                .long("overwrite")
                .help("Overwrite the workspace toml file if it exists")
                .conflicts_with("update"),
        )
        .arg(
            Arg::with_name("update")
                .short("u")
                .long("update")
                .help("Update the workspace toml file. It must exist.")
                .conflicts_with("overwrite"),
        )
        .arg(
            Arg::with_name("path")
                .short("p")
                .default_value("")
                .help("Path to initialize workspace"),
        );

    let matches = (match cargo {
        CargoRun::LikelyRunFromCargo => {
            app.usage(concat!("cargo ", wsinit!(), " [FLAGS] [OPTIONS]"))
        }
        CargoRun::RunOutsideCargo => app,
    })
    .get_matches_from(args);

    let path = matches.value_of("path").expect("Has default");
    let file_exists_behaviour = get_file_exists_behaviour(&matches);

    let options = Options::new(path, file_exists_behaviour);
    let workspace = Workspace::new(options);
    match workspace.update_toml() {
        Ok(toml) => {
            println!(
                "{}",
                format!("Workspace file created/updated at {}", toml).green()
            );
        }
        Err(Error::FileAlreadyExists) => {
            eprintln!(
                "{}",
                format!(
                    "Could not create file {} for workspace!\nThe file already exists. Use --update to replace.",
                    workspace.toml()
                ).red()
            );
        }
        Err(Error::GenericCreationError(io)) => {
            eprintln!(
                "{}",
                format!(
                    "Could not create file {} for workspace!\n{:?}",
                    workspace.toml(),
                    io.kind()
                )
                .red()
            );
        }
        Err(Error::WriteError(io)) => {
            eprintln!(
                "{}",
                format!(
                    "Could not write file {} for workspace!\n{:?}",
                    workspace.toml(),
                    io
                )
                .red()
            );
        }
        Err(Error::ReadError(io)) => {
            eprintln!(
                "{}",
                format!("Could not read file {}!\n{:?}", workspace.toml(), io).red()
            );
        }
        Err(Error::ParseError) => {
            eprintln!(
                "{}",
                format!(
                    "Could not parse the existing toml file {}!",
                    workspace.toml()
                )
                .red()
            );
        }
    }
}

#[derive(PartialEq)]
enum CargoRun {
    LikelyRunFromCargo,
    RunOutsideCargo,
}

/// Removes sub_command, e.g. "wsinit" from the second index as this means it's likely running from cargo <sub_command>
fn get_command_line_args(cargo_sub_command: &str) -> (Vec<String>, CargoRun) {
    let mut args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(s) if s == cargo_sub_command => {
            args.remove(1);
            (args, CargoRun::LikelyRunFromCargo)
        }
        _ => (args, CargoRun::RunOutsideCargo),
    }
}

fn get_file_exists_behaviour(matches: &ArgMatches) -> FileExistsBehaviour {
    if matches.is_present("update") {
        FileExistsBehaviour::Update
    } else if matches.is_present("overwrite") {
        FileExistsBehaviour::Overwrite
    } else {
        FileExistsBehaviour::Halt
    }
}
