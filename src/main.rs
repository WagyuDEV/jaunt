use jauntings::{jaunt_init, jaunt_track, jaunt_untrack};

use clap::{arg, Arg, ArgAction, Command};

fn commandler() -> Command {
    Command::new("jaunt")
        .about("Track that jaunt")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
            .about("Initialize a new jaunt repo")
        )
        .subcommand(
            Command::new("track")
                .about("Add files or dirs to their respective include arrays in jaunt.toml, calling this functions calls sync by default")
                .arg_required_else_help(true)
                // .arg(
                //     Arg::new("dir")
                //     .long("dir")
                //     .short('d')
                //     .help("Specify that a pattern is meant to apply to directories")
                // )
                .arg(
                    Arg::new("no-sync")
                    .long("no-sync")
                    .short('S')
                    .help("Do not run sync after editing jaunt.toml via command")
                )
                .arg(
                    Arg::new("remove")
                    .long("remove")
                    .short('R')
                    .help("Remove provided pattern from the include section of jaunt.toml")
                )
                .arg(
                    Arg::new("patterns")
                    .required(true)
                    .num_args(1..)
                )
        )
        .subcommand(
            Command::new("untrack")
            .about("Add files or dirs to their respective ignore arrays in jaunt.toml, calling this functions calls sync by default")
            .arg_required_else_help(true)
            // .arg(
            //     Arg::new("dir")
            //     .long("dir")
            //     .short('d')
            //     .help("Specify that a pattern is meant to apply to directories")
            // )
            .arg(
                Arg::new("no-sync")
                .long("no-sync")
                .short('S')
                .help("Do not run sync after editing jaunt.toml via command")
            )
            .arg(
                Arg::new("patterns")
                .required(true)
                .num_args(1..)
            )
        )
        .subcommand(
            Command::new("sync")
            .about("Sync files tracked in git with files in jaunt.toml")
        )
}

fn main() {
    let matches = commandler().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            jaunt_init();
        },
        Some(("track", subs)) => {
            jaunt_track(subs);
        }
        Some(("untrack", subs)) => {
            jaunt_untrack(subs);
        }
        _ => {}
    }
}
