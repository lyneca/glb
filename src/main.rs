extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs;
use std::process;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    values: Vec<KeyValue>
}

fn read_config() -> String {
    let config_file_path = match env::var("CONFIG_FILE") {
        Ok(file) => file,
        Err(_) => format!("{}/.config/config.yaml", env::var("HOME").unwrap())
    };
    fs::read_to_string(&config_file_path)
        .expect(&format!("Couldn't find the file \"{}\"", config_file_path))
    
}

fn get(config: Config, args: Vec<String>) {
    let mut found = false;
    for mut kv in config.values {
        if kv.key == args[0] {
            println!("{}", kv.value);
            found = true;
        }
    };
    if !found {
        println!("Key not found.");
        process::exit(1);
    }
}

fn set(mut config: Config, args: Vec<String>) {
    let mut found = false;
    for mut kv in &mut config.values {
        if kv.key == args[0] {
            kv.value = args[1].clone();
            found = true;
        }
    }
    if !found {
        config.values.push(KeyValue { key: args[0].clone(), value: args[1].clone() });
    }
    write_config(config)
}

fn help() {
    println!("Usage: config <command> [OPTIONS...]");
    println!("where <command> is one of:");
    println!("    get, set\n");
    println!("config get [key]          -- prints the value of [key]");
    println!("config set [key] [value]  -- set the value of [key] to [value]\n");
    println!("Values are stored in $HOME/.config/config.yaml, or in $CONFIG_FILE.");
}

fn write_config(config: Config) {
    let config_file_path: String = match env::var("CONFIG_FILE") {
        Ok(file) => file,
        Err(_) => format!("{}/.config/config.yaml", env::var("HOME").unwrap())
    };
    let mut config_file = File::create(&config_file_path)
        .expect(&format!("Couldn't find the file \"{}\"", config_file_path));
    config_file.write_all(serde_yaml::to_string(&config).unwrap().as_bytes())
        .expect("Could not write to file.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        0 => panic!("Args is empty?"),
        1 => {
            help();
            return
        },
        _ => {}
    }
    let yaml: Config = serde_yaml::from_str(&read_config())
        .expect("Invalid configuration file");
    match args[1].as_str() {
        "get" => get(yaml, args[2..].to_vec()),
        "set" => set(yaml, args[2..].to_vec()),
        "help" => help(),
        _ => {}
    }
}
