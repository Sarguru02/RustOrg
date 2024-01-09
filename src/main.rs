use std::env;
use std::fs;
use std::path::PathBuf;
mod config;

use config::parse_config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        3 => {
            let directory_to_organize = &args[1];
            let rule_path= &args[2];
            logic(Some(rule_path), Some(directory_to_organize));
        },
        2 => {
            let directory_to_organize = &args[1];
            logic(None, Some(directory_to_organize));
        }
        1 => {
            logic(None, None);
        },
        _ => {
            println!("Usage: rorg <directory_to_organize> <rule_path>");
        },
    }
}

fn logic(rule_path: Option<&str>, directory_to_organize: Option<&str>) { 
    // $HOME variable got
    let home =env::var("HOME").unwrap();

    //defaults if no arguments are passed
    let default_rules = format!("{}/.config/rorg/rules.toml", &home);
    let default_directory_to_organize = format!("{}/Downloads", &home);

    //setting default values if no arguments are passed
    let rule_path = rule_path.unwrap_or_else(|| default_rules.as_str());
    let directory_to_organize = directory_to_organize.unwrap_or_else(|| default_directory_to_organize.as_str());

    let config = parse_config(rule_path);
    println!("Logic function"); 
    println!("Rule path {}", rule_path);
    println!("Directory to organize {}", directory_to_organize);

    let paths = get_files(directory_to_organize).expect("Error getting files");

    for i in config.rules {
        println!("Rule name: {}", i.name);

        create_directory(directory_to_organize, i.target_directory.as_str()).expect("Error creating directory");
        for j in i.extensions {
            let target_directory = format!("{}/{}", directory_to_organize, i.target_directory);
            organize(&paths, j.as_str(), &target_directory).expect("Error printing files");
        }
    }
    let paths = get_files(directory_to_organize).expect("Error getting files");
    let target_directory = format!("{}/{}", directory_to_organize, "Others");
    let _ = create_directory(directory_to_organize, "Others");
    organize(&paths, "*", &target_directory).expect("Error printing files");

}

fn create_directory(base_path: &str, target_directory: &str) -> Result<(), std::io::Error>{
    println!("Creating directory: {}/{}", base_path, target_directory);
    let path = format!("{}/{}", base_path, target_directory);
    fs::create_dir_all(&path)?;

    Ok(())

}

fn get_files(directory: &str) -> Result<Vec<PathBuf>, std::io::Error> {

        let dir_entries = fs::read_dir(directory)?;

        let mut files = Vec::new();

        for entry in dir_entries.filter_map(|entry| entry.ok()) {
            if entry.file_type()?.is_file() {
                files.push(entry.path());
            }
        }

        Ok(files)

}

fn organize(paths: &Vec<PathBuf>, extension: &str, target_directory: &str) -> Result<(), std::io::Error>{
    
    if extension == "*" {
        for path in paths {
            let path_str = path.to_str().expect("Invalid file name");
            move_file(path_str, target_directory)?;
            println!("File: {:?}", path);
        }
        return Ok(())
    }
    for path in paths {
        let path_str = path.to_str().expect("Invalid file name");
        if let Some(file_extension) = path.extension().and_then(|e| e.to_str()) {
            if file_extension == extension {
                move_file(path_str, target_directory)?;
                println!("File: {:?}", path);
            }
        }
    }

    Ok(())
}


fn move_file(file_path: &str, target_directory: &str) -> Result<(), std::io::Error> {
    let file_path = PathBuf::from(file_path);
    let file_name = file_path.file_name().expect("Invalid file name");
    let destination = format!("{}/{}", target_directory, file_name.to_str().unwrap());
    fs::rename(file_path, destination)?;
    Ok(())
}
