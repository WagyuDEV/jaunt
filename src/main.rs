use clap::{arg, Command, Arg, ArgAction};

fn commandler() -> Command {
    Command::new("jaunt")
        .about("Track that jaunt")
        .subcommand_required(false)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("init")
            .about("Initialize a new jaunt repo")
        ).subcommand(
            Command::new("track")
            .about("Choose the files that jaunt tracks")
            .arg_required_else_help(true)
            .arg(
                    Arg::new("remove")
                    .help("Untrack files")
                    .short('R')
                    .long("remove")
                )
        )
        .subcommand(
            Command::new("commit")
            .about("Update what jaunt knows")
            .arg_required_else_help(true)
            .arg(arg!(<MESSAGE> "Commit Message"))
        )
        .subcommand(
            Command::new("shove")
            .about("Push changes remotely")
            .arg_required_else_help(true)
        )
}

fn main() {
    let matches = commandler().get_matches();

    match matches.subcommand() {
        Some(("init", cmd)) => {
            
        }
    }
}
