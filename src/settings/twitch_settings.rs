use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TwitchSettings {
    pub username: String,
    pub oauth_token: String,
    pub voting_time: u64,
}

impl TwitchSettings {
    pub fn get() -> Option<TwitchSettings> {
        if fs::exists("./twitch_settings.toml").expect("failed to check if twitch_settings.toml exists.") {
            Some(Self::load())
        } else {
            Self::save();
            None
        }
    }

    fn save() {
        let default_cfg = TwitchSettings {
            username: String::from("USERNAME OF THE ACCOUNT MUST MATCH THE USERNAME ATTACHED TO THE OAUTH TOKEN"),
            oauth_token: String::from("OAUTH TOKEN, GENERATE: https://twitchtokengenerator.com/"),
            voting_time: 45,
        };

        // serialize default config & write to disk
        let cfg_str = toml::to_string_pretty(&default_cfg).expect("failed to serialize default twitch settings");
        fs::write("./twitch_settings.toml", cfg_str).expect("failed to write default twitch settings");
    }

    fn load() -> TwitchSettings {
        let cfg_contents = fs::read_to_string("./twitch_settings.toml").expect("failed to read twitch settings");
        let cfg: TwitchSettings = toml::from_str(&cfg_contents).expect("failed to deserialize twitch settings");
        cfg
    }
}