extern crate serde;
extern crate serde_yaml;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate clap;

use clap::{App, AppSettings, ArgMatches};
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

fn get_config_path() -> String {
    match env::var("CONFIG_FILE") {
        Ok(file) => file,
        Err(_) => format!("{}/.config/glb.yaml", env::var("HOME").unwrap())
    }
}

fn read_config() -> String {
    let config_file_path = get_config_path();
    fs::read_to_string(&config_file_path)
        .expect(&format!("Couldn't find the file \"{}\"", config_file_path))
    
}

fn get(config: Config, args: &ArgMatches) {
    let mut found = false;
    for kv in config.values {
        if kv.key == args.value_of("key").unwrap() {
            println!("{}", kv.value);
            found = true;
        }
    };
    if !found {
        println!("Key not found.");
        process::exit(1);
    }
}

fn set(mut config: Config, args: &ArgMatches) {
    let mut found = false;
    for mut kv in &mut config.values {
        if kv.key == args.value_of("key").unwrap() {
            kv.value = String::from(args.value_of("value").unwrap());
            found = true;
        }
    }
    if !found {
        config.values.push(KeyValue {
            key: String::from(args.value_of("key").unwrap()), value: String::from(args.value_of("value").unwrap())
        });
    }
    write_config(config)
}

fn del(mut config: Config, args: &ArgMatches) {
    let mut found = false;
    let mut to_remove = 0;
    for (i, kv) in config.values.iter().enumerate() {
        if kv.key == args.value_of("key").unwrap() {
            to_remove = i;
            found = true;
        }
    }
    if found {
        config.values.remove(to_remove);
    } else {
        println!("Value \"{}\" not found.", args.value_of("key").unwrap());
    }
    write_config(config);
}

fn list(config: Config) {
    println!("Currently defined values:");
    for kv in config.values {
        println!(" - {}: {}", kv.key, kv.value);
    }
}

fn write_config(config: Config) {
    let config_file_path = get_config_path();
    let mut config_file = File::create(&config_file_path)
        .expect(&format!("Couldn't find the file \"{}\"", config_file_path));
    config_file.write_all(serde_yaml::to_string(&config).unwrap().as_bytes())
        .expect("Could not write to file.");
}

fn main() {
    let cli_yaml = load_yaml!("cli.yml");
    let yaml: Config = serde_yaml::from_str(&read_config())
        .expect("Invalid configuration file.");
    let app = App::from_yaml(cli_yaml)
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::ColorAlways);
    let matches = app.get_matches();
    match matches.subcommand() {
        ("get", Some(args)) => get(yaml, args),
        ("set", Some(args)) => set(yaml, args),
        ("del", Some(args)) => del(yaml, args),
        ("list", Some(_)) => list(yaml),
        _ => {}
    }
}
