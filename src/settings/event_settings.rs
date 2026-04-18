use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct EventSettings {
    pub meta_no_chaos_roo: u32, // no chaos rate of occurrence
    pub meta_extra_chaos_roo: u32, // extra chaos rate of occurrence
    // todo: combo meta event (applies an extra random event)
}

impl EventSettings {
    pub fn get() -> EventSettings {
        if fs::exists("./event_settings.toml").expect("failed to check if event_settings.toml exists.") {
            Self::load()
        } else {
            Self::save()
        }
    }

    fn save() -> EventSettings {
        let default_cfg = EventSettings {
            meta_no_chaos_roo: 30,
            meta_extra_chaos_roo: 50,
        };
        
        // serialize default config & write to disk
        let cfg_str = toml::to_string_pretty(&default_cfg).expect("failed to serialize default event settings");
        fs::write("./event_settings.toml", cfg_str).expect("failed to write default event settings");
        default_cfg
    }

    fn load() -> EventSettings {
        let cfg_contents = fs::read_to_string("./event_settings.toml").expect("failed to read event settings");
        toml::from_str(&cfg_contents).expect("failed to deserialize event settings")
    }
}