use std::env::current_dir;

use clap::ArgMatches;


pub fn jaunt_untrack(subs: &ArgMatches){
    match current_dir(){
        Ok(dir) => {
            let jaunt_dot_toml = dir.join("jaunt.toml");

            // execute command if there is an existing jaunt.toml else die
            if jaunt_dot_toml.exists(){
                
            }else{

            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}