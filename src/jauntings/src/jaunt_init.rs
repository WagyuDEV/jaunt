use std::io::Write;
use std::process::{exit};
use std::env::{current_dir};
use std::fs::{create_dir, File};

use crate::find_jaunt::find_jaunt;

pub fn jaunt_init(){
    match current_dir() {
        Ok(dir) => {
            // let jaunt_dot_toml = dir.join("jaunt.toml");

            if find_jaunt().is_ok(){
                eprintln!("A jaunt.toml already exist in this directory");
                exit(0);
            }else{
                match File::create_new(dir.join("jaunt.toml").as_path()) {
                    Ok(mut new_toml) => {
                        // TODO: Write boiler plate file contents
                        // TODO: write data to file
                        /*
                        [settings]
                        regex = true
                        regex// the purpose of this file is to provide a function that can be used to find the jaunt.toml of a project.
// it will start in the current directory and step back until it either reaches the root directory or finds jaun.toml
// if it reaches the root it will throw an Err("No jaunt.toml") else it will return Ok("/path/to/jaun.toml")
pub fn find_jaunt() -> Result<std::path::PathBuf, &'static str>{
    let mut found = false;

    let mut dir = std::env::current_dir();


    loop {
        match dir {
            Ok(p) => {
                let temp = p.join("jount.toml");
                if dir.expect("This error should never happen").parent().is_none() {
                    return Err("No jaunt.toml could be found in this project");
                }
                if temp.exists(){
                    return Ok(temp);
                }else{
                    dir.expect("This error should never happen").pop();
                }
            },
            Err(e) => {
                eprintln!("Error {}", e);
                std::process::exit(1);
            }
        }
    }
    // Ok(_)
}_global = false # this stands for the g flag in regex. can cause issues

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
                                eprintln!("Error: {}", e);
                                exit(1);
                            }
                        }

                        
                    },
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        exit(1);
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}
