use std::fs;

use clap::ArgMatches;

use toml::Value;

// use regex::

use crate::find_jaunt::find_jaunt;

pub fn jaunt_track(subs: &ArgMatches){
    let jaunt_dot_toml = find_jaunt();

    // flag that decides whether or not the provided pattern(s) is being added or removed (T): remove (F): add
    let _aor = subs.contains_id("remove");

    match jaunt_dot_toml{
        Ok(dir) => {
            let content = fs::read_to_string(&dir)
                .expect("Failed to read jaunt.toml")
                .parse::<Value>()
                .expect("Failed to parse jaunt.toml");

            let tracking = content.get("tracking")

            // subs.
            if let Some(patterns) = subs.get_many::<String>("patterns"){
                for pattern in patterns{
                    // this code was written by chatgpt. needs correction. change to reference include not ignore
                    // Add the file to the ignore list if it's not already present
                    // if !ignore_list.iter().any(|v| v.as_str() == Some(file)) {
                    //     ignore_list.push(Value::String(file.clone()));
                    // }


                }
            }
        },
        Err(e) =>{
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }

    if subs.contains_id("no-sync"){
        // do not run jaunt_sync
    }else{
        // run jaunt_sync
    }
}