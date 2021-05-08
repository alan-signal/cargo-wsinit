use clap::{App, Arg, ArgMatches};
use colored::*;

use cargo_wsinit::{Error, FileExistsBehaviour, Options, Workspace};

fn main() {
    let matches = App::new("Cargo Workspace Init")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .usage("cargo wsinit [FLAGS] [OPTIONS]")
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
        )
        .get_matches();

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

fn get_file_exists_behaviour(matches: &ArgMatches) -> FileExistsBehaviour {
    if matches.is_present("update") {
        FileExistsBehaviour::Update
    } else if matches.is_present("overwrite") {
        FileExistsBehaviour::Overwrite
    } else {
        FileExistsBehaviour::Halt
    }
}
