// =======================================================================
//  Copyleft City:Arts Project 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from external library *//
use std::fs::File;
use std::vec::Vec;
use std::io::prelude::*;
use std::io::Error;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub telegram: Option<TelegramConfig>,
    pub server: Option<ServerConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Commands {
    pub first_msg: Option<String>,
    pub commands: Option<Vec<Command>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Command {
    pub command: Option<String>,
    pub reply: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct TelegramConfig {
    pub bot_api: Option<String>,
    pub admins: Option<Vec<String>>,
    pub main_group_id: Option<String>,
    pub username: Option<String>
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub address: Option<String>,
    pub port: Option<String>,
    pub rcon_address: Option<String>,
    pub rcon_port: Option<String>,
    pub rcon_pass: Option<String>
}

impl Config {
    pub const fn default() -> Self { Self { server: None, telegram: None } }
}

impl Commands {
    pub const fn default() -> Self { Self { first_msg: None, commands: None } }
}

pub fn read_conf(filename: &str) -> Result<Config, Error> {
    let mut file = File::open(filename).unwrap();
    let mut toml_str = String::new();

    file.read_to_string(&mut toml_str)?; // Read config file data
    Ok(::toml::from_str(&toml_str).unwrap())
}

pub fn read_commands(filename: &str) -> Result<Commands, Error> {
    let mut file = File::open(filename).unwrap();
    let mut toml_str = String::new();

    file.read_to_string(&mut toml_str)?; // Read config file data
    Ok(::toml::from_str(&toml_str).unwrap())
}
