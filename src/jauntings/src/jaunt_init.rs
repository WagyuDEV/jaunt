use std::process::{exit};
use std::env::{current_dir};
use std::fs::{create_dir, File};

pub fn jaunt_init(){
    match current_dir() {
        Ok(dir) => {
            let jaunt_dot_toml = dir.join("jaunt.toml");

            if jaunt_dot_toml.exists(){
                eprintln!("A jaunt repo or its side effects already exists in this directory");
                exit(0);
            }else{
                match File::create_new(dir.join("jaunt.toml").as_path()) {
                    Ok(new_toml) => {
                        // TODO: Write boiler plate file contents
                        // TODO: write data to file
                        /*
                        [project]
                        name = "dir_name"
                        
                         */
                        let file_content = format!("");

                        
                    },
                    Err(e) => {
                        eprintln!("Error {}", e);
                        exit(1);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error {}", e);
            exit(1);
        }
    }
}