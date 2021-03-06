use serde::{Deserialize, Serialize};
use std::{env, process, fs};
use std::fs::File;
use std::io::Write;
use lazy_static::lazy_static;
use std::path::PathBuf;
use std::collections::HashSet;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Config {
    pub token: String,
    pub name: String,
    pub group: i64,
    pub silences: HashSet<i64>,
    pub locks: HashSet<i64>,
    pub notes: Vec<i64>,
    pub answers: Vec<(i64, Vec<String>)>,
}

lazy_static! {
    static ref CONFIG_PATH: PathBuf = env::current_exe().unwrap().parent().unwrap().join("config");
}

pub fn load() -> Config {
    if !CONFIG_PATH.exists() {
        let mut config_file = File::create(&*CONFIG_PATH).unwrap();
        config_file.write_all(
            serde_json::to_string_pretty(&Config { ..Default::default() }).unwrap().as_bytes()
        ).unwrap();
        println!("config created");
        process::exit(0);
    } else {
        let config_content = fs::read_to_string(&*CONFIG_PATH).unwrap();
        let config: Config = serde_json::from_str(&config_content).unwrap();
        return config;
    }
}

impl Config {
    pub fn save(&self) {
        let mut config_file = File::create(&*CONFIG_PATH).unwrap();
        config_file.write_all(
            serde_json::to_string_pretty(self).unwrap().as_bytes()
        ).unwrap();
    }
}