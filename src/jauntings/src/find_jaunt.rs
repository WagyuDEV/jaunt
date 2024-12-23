// the purpose of this file is to provide a function that can be used to find the jaunt.toml of a project.
// it will start in the current directory and step back until it either reaches the root directory or finds jaun.toml
// if it reaches the root it will throw an Err("No jaunt.toml") else it will return Ok("/path/to/jaun.toml")
pub fn find_jaunt() -> Result<std::path::PathBuf, &'static str>{
    // let mut found = false;

    let mut dir = match std::env::current_dir(){
        Ok(p) =>  p,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    loop {
        let temp = dir.join("jaunt.toml");

        if temp.exists(){
            return Ok(temp);
        }
        if !dir.pop(){
            return Err("No jaunt.toml could be found in this project");
        }
    }
    // Ok(_)
}