// =======================================================================
//  Copyleft City:Arts Project 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from external library *//
use futures::Future;
use tokio_core::reactor::{Handle};
use telegram_bot::{Api, Message, ParseMode, MessageKind};
use telegram_bot::prelude::*;

//* Use from local library *//
use conf::{Config, Commands, Command, TelegramConfig, ServerConfig};
use mcapi::{MCApi, Server, Players};
use rcon::Client;

pub struct TGBot;

impl TGBot {
    fn parse_reply(cmd: &str) -> String {
        let conf: Commands = unsafe { ::COMMANDS.clone() };
        let mut commands: Vec<Command> = conf.commands.unwrap();
        commands.retain(|x| x.clone().command.unwrap() == cmd);
        format!("{} {}", conf.first_msg.unwrap(), commands.into_iter().nth(0).unwrap().reply.unwrap())
    }

    fn check_admin(id: String) -> bool {
        let conf: Config = unsafe { ::CONF.clone() };
        let conf_telegram: TelegramConfig = conf.telegram.unwrap();
        let admins: Vec<String> = conf_telegram.admins.unwrap();

        for admin in admins {
            if id == admin { return true };
        }
        false
    }

    fn start(api: Api, message: Message, handle: &Handle, _parameter: String) {
        let html = api.send(message.text_reply(TGBot::parse_reply("start"))
            .parse_mode(ParseMode::Html)
        );
        
        handle.spawn({
            let future = html;
            future.map_err(|_| ()).map(|_| ())
        })
    }

    fn stop(api: Api, message: Message, handle: &Handle, _parameter: String) {
        let html = api.send(message.text_reply(TGBot::parse_reply("stop"))
            .parse_mode(ParseMode::Html)
        );
        
        handle.spawn({
            let future = html;
            future.map_err(|_| ()).map(|_| ())
        })
    }

    fn help(api: Api, message: Message, handle: &Handle, _parameter: String) {
        let html = api.send(message.text_reply(TGBot::parse_reply("help").replace("{VERSION}", env!("CARGO_PKG_VERSION")))
            .parse_mode(ParseMode::Html)
        );
        
        handle.spawn({
            let future = html;
            future.map_err(|_| ()).map(|_| ())
        })
    }

    fn status(api: Api, message: Message, handle: &Handle, _parameter: String) {
        let mut html = api.send(message.text_reply(TGBot::parse_reply("status_err"))
            .parse_mode(ParseMode::Html)
        );
        
        if let Ok(mcapi) = MCApi::new() {
            let server: Server = mcapi.server.unwrap();
            let players: Players = mcapi.players.unwrap();

            html = api.send(message.text_reply(TGBot::parse_reply("status")
                .replace("{SERVER_VERSION}", &server.name.unwrap())
                .replace("{SERVER_STATUS}", if mcapi.online.unwrap() { "✅" } else { "❎" })
                .replace("{SERVER_USERS}", &format!("{} / {}", players.now.unwrap(), players.max.unwrap())))
                .parse_mode(ParseMode::Html)
            );
        }
        
        handle.spawn({
            let future = html;
            future.map_err(|_| ()).map(|_| ())
        })
    }

    fn add(api: Api, message: Message, handle: &Handle, parameter: String) {
        let conf: Config = unsafe { ::CONF.clone() };
        let server_conf: ServerConfig = conf.server.unwrap();
        let msg = message.clone();
        let mut html = api.send(message.text_reply(TGBot::parse_reply("rcon_err2"))
            .parse_mode(ParseMode::Html)
        );

        info!("{}", format!("'/add' command issued by {}({}). parameter is '{}'", msg.from.first_name, msg.from.id, parameter));

        if let Ok(mut client) = Client::new(server_conf.rcon_address.unwrap(), server_conf.rcon_port.unwrap(), server_conf.rcon_pass.unwrap()) {
            let _ = client.send_auth();
            html = api.send(message.text_reply(TGBot::parse_reply("add").replace("{OUTPUT}", &client.send_command(&format!("whitelist add {}", parameter))))
                .parse_mode(ParseMode::Html)
            );
        }
        
        handle.spawn({
            let future = html;
            future.map_err(|_| ()).map(|_| ())
        })
    }

    fn remove(api: Api, message: Message, handle: &Handle, parameter: String) {
        let conf: Config = unsafe { ::CONF.clone() };
        let server_conf: ServerConfig = conf.server.unwrap();
        let msg = message.clone();
        let mut html = api.send(message.text_reply(TGBot::parse_reply("rcon_err2"))
            .parse_mode(ParseMode::Html)
        );

        info!("{}", format!("'/add' command issued by {}({}). parameter is '{}'", msg.from.first_name, msg.from.id, parameter));

        if let Ok(mut client) = Client::new(server_conf.rcon_address.unwrap(), server_conf.rcon_port.unwrap(), server_conf.rcon_pass.unwrap()) {
            let _ = client.send_auth();
            html = api.send(message.text_reply(TGBot::parse_reply("remove").replace("{OUTPUT}", &client.send_command(&format!("whitelist remove {}", parameter))))
                .parse_mode(ParseMode::Html)
            );
        }
        
        handle.spawn({
            let future = html;
            future.map_err(|_| ()).map(|_| ())
        })
    }

    fn rcon(api: Api, message: Message, handle: &Handle, parameter: String) {
        let conf: Config = unsafe { ::CONF.clone() };
        let server_conf: ServerConfig = conf.server.unwrap();
        let msg = message.clone();

        let mut html = api.send(message.text_reply(TGBot::parse_reply("rcon_err"))
            .parse_mode(ParseMode::Html)
        );

        if TGBot::check_admin(msg.from.id.to_string()) {
            info!("{}", format!("'/rcon' command issued by {}({}). parameter is '{}'", msg.from.first_name, msg.from.id, parameter));

            html = api.send(message.text_reply(TGBot::parse_reply("rcon_err2"))
                .parse_mode(ParseMode::Html)
            );

            if let Ok(mut client) = Client::new(server_conf.rcon_address.unwrap(), server_conf.rcon_port.unwrap(), server_conf.rcon_pass.unwrap()) {
                let _ = client.send_auth().unwrap();
                html = api.send(message.text_reply(TGBot::parse_reply("rcon").replace("{OUTPUT}", &client.send_command(&parameter)))
                    .parse_mode(ParseMode::Html)
                );
            }
        }
        
        handle.spawn({
            let future = html;
            future.map_err(|_| ()).map(|_| ())
        })
    }

    fn ch(api: Api, message: Message, handle: &Handle, parameter: String) {
        let conf: Config = unsafe { ::CONF.clone() };
        let server_conf: ServerConfig = conf.server.unwrap();
        let msg = message.clone();
        let mut html = api.send(message.text_reply(TGBot::parse_reply("rcon_err2"))
            .parse_mode(ParseMode::Html)
        );

        info!("{}", format!("'/ch' command issued by {}({}). parameter is '{}'", msg.from.first_name, msg.from.id, parameter));

        if let Ok(mut client) = Client::new(server_conf.rcon_address.unwrap(), server_conf.rcon_port.unwrap(), server_conf.rcon_pass.unwrap()) {
            let _ = client.send_auth();
            html = api.send(message.text_reply(TGBot::parse_reply("chat").replace("{OUTPUT}", &client.send_command(&format!("{}[§cT§6a§el§ak§f] {} : {}{}", r#"tellraw @a {"text":""#, msg.from.first_name, parameter, r#""}"#))))
                .parse_mode(ParseMode::Html)
            );
        }
        
        handle.spawn({
            let future = html;
            future.map_err(|_| ()).map(|_| ())
        })
    }
}

pub fn bot(api: Api, message: Message, handle: &Handle) {
    let conf: Config = unsafe { ::CONF.clone() };
    let conf_telegram: TelegramConfig = conf.telegram.unwrap();
    let username = conf_telegram.username.unwrap();
    let mut parameter = String::new();

    let function: fn(Api, Message, &Handle, String) = match message.kind {
        MessageKind::Text {ref data, ..} => {
            let matches: Vec<&str> = data.as_str().matches("/").collect();
            if matches.is_empty() {
                return
            } else {
                let matches: Vec<&str> = data.split_whitespace().nth(0).unwrap().matches("@").collect();
                let mut msg: Vec<&str> = data.split_whitespace().nth(0).unwrap().split(|c| c == '/' || c == '@').collect();
                let mut space_msg: Vec<&str> = data.split_whitespace().collect();
                space_msg.remove(0);
                msg.extend(space_msg);
                
                if !matches.is_empty() {
                    parameter = if msg.len() >= 4 { 
                        let mut res: String = String::new();
                        for i in &msg[3..] { res += &format!("{} ", i) };
                        res
                    } else { String::new() };
                    if msg[2] != username.as_str() {
                        return
                    }
                } else {
                    parameter = if msg.len() >= 3 { 
                        let mut res: String = String::new();
                        for i in &msg[2..] { res += &format!("{} ", i) };
                        res
                    } else { String::new() };
                }

                match msg[1] {
                    "start" => TGBot::start,
                    "stop" => TGBot::stop,
                    "help" => TGBot::help,
                    "status" => TGBot::status,
                    "add" => TGBot::add,
                    "remove" => TGBot::remove,
                    "rcon" => TGBot::rcon,
                    "ch" => TGBot::ch,
                    _ => return,
                }
            }
        }
        _ => return
    };

    function(api, message, handle, parameter)
}
