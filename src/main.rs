use std::env;
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
    println!("Config {:#?}", config);
}
