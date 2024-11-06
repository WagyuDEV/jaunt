use std::io::Write;
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
                    Ok(mut new_toml) => {
                        // TODO: Write boiler plate file contents
                        // TODO: write data to file
                        /*
                        [settings]
                        regex = true
                        regex_global = false # this stands for the g flag in regex. can cause issues

                        [tracking]
                        ignore = [] # list of files and or patterns that should be ignored by git
                        include = [] # list of files and or patterns that should be tracked
                         */
                        let file_content = format!("\
[settings]
regex = true
                        
[tracking]
ignore = []
ignore_dirs = []
include = ['.']
include_dirs = ['.']");

                        match new_toml.write(file_content.as_bytes()) {
                            Ok(_) => (),
                            Err(e) => {
                                eprintln!("Error {}", e);
                                exit(1);
                            }
                        }

                        
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
