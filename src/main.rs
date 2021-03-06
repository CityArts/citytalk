// =======================================================================
//  Copyleft City:Arts Project 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

#![feature(const_fn)]
#![feature(extern_prelude)]
#![feature(label_break_value)]
#![feature(fixed_size_array)]

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate log;

extern crate telegram_bot;
extern crate tokio_core;
extern crate futures;
extern crate toml;
extern crate serde_json;
extern crate reqwest;
extern crate simple_logging;
extern crate chrono;
extern crate rcon_rs;

mod conf;
mod bot;
mod mcapi;

//* Use from external library *//
use std::fs;
use std::path::Path;
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::{Api, UpdateKind};
use log::LevelFilter;
use chrono::prelude::*;

//* Use from local library *//
use conf::{read_conf, read_commands};
use conf::{Config, Commands, TelegramConfig};
use bot::bot;

pub static mut CONF: Config = Config::default();
pub static mut COMMANDS: Commands = Commands::default();

fn main() {
    unsafe { CONF = read_conf("bot.toml").unwrap() };
    unsafe { COMMANDS = read_commands("commands.toml").unwrap() };
    let conf: Config = unsafe { CONF.clone() };
    let conf_telegram: TelegramConfig = conf.telegram.unwrap();
    let token = conf_telegram.bot_api.unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let api = Api::configure(token).build(core.handle()).unwrap();

    let future = api.stream().for_each(|update| {
        if let UpdateKind::Message(message) = update.kind {
            bot(api.clone(), message, &handle)
        }
        Ok(())
    });

    if !Path::new("log").exists() { fs::create_dir("log").unwrap() };

    simple_logging::log_to_file(&format!("log/bot-{}", Local::now().to_rfc3339()), LevelFilter::Info).unwrap();
    core.run(future).unwrap();
}
