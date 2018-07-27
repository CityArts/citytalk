// =======================================================================
//  Copyleft City:Arts Project 2018-∞.
//  Distributed under the terms of the 3-Clause BSD License.
//  (See accompanying file LICENSE or copy at
//   https://opensource.org/licenses/BSD-3-Clause)
// =======================================================================

//* Use from local library *//
use conf::{Config, ServerConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MCApi {
    pub status: Option<String>,
    pub online: Option<bool>,
    pub motd: Option<String>,
    pub favicon: Option<String>,
    pub error: Option<String>,
    pub players: Option<Players>,
    pub server: Option<Server>,
    pub last_online: Option<String>,
    pub last_updated: Option<String>,
    pub duration: Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Players {
    pub max: Option<u32>,
    pub now: Option<u32>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub name: Option<String>,
    pub protocol: Option<u32>
}

impl MCApi {
    pub fn new() -> Self {
        let conf: Config = unsafe { ::CONF.clone() };
        let conf_server: ServerConfig = conf.server.unwrap();

        let res: Self = reqwest::get(&format!("https://mcapi.us/server/status?ip={}", conf_server.address.unwrap())).unwrap().json().unwrap();
        res
    }
}