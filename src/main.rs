use jauntings::jaunt_init;

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
                .about("Choose the files that jaunt tracks")
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("untrack")
            .about("Stop tracking a file")
            .arg_required_else_help(true)
        )
}

fn main() {
    let matches = commandler().get_matches();

    match matches.subcommand() {
        Some(("init", _)) => {
            jaunt_init();
        },
        Some(("track", _subs)) => {

        }
        Some(("untrack", _subs)) => {
            
        }
        _ => {}
    }
}
