use std::time::{Duration, Instant};
use log::warn;
use crate::game::bully::GameData;
use crate::game::mods::health;
use crate::memory::{game_offsets, memory};

pub async fn max_trouble(data: &GameData) {
    let start_time = Instant::now();
    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }
        
        // update trouble
        update_trouble(&data, 300);
        
        // sleep for cpu usage
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

pub async fn trouble_health(data: &GameData) {
    let start_time = Instant::now();
    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }

        // get current health
        match health::get_health(&data) {
            Some(current_health) => {
                // end event if the player is dead
                if current_health <= 0f32 {
                    break;
                }

                // update trouble
                update_trouble(&data, current_health as i32);

                // sleep for cpu usage
                tokio::time::sleep(Duration::from_millis(100)).await;
            },
            None => {
                warn!("failed to get current health.");
                tokio::time::sleep(Duration::from_millis(25)).await;
            }
        }
    }
}

pub fn remove_trouble(data: &GameData) {
    update_trouble(&data, 0);
}

fn update_trouble(data: &GameData, wanted_level: i32) {
    match game_offsets::get_offset(data.handle, data.player_offset, game_offsets::PLAYER_WANTED_LEVEL_OFFSET) {
        Some(wanted_level_offset) => {
            if !memory::write::<i32>(data.handle, wanted_level_offset, wanted_level) {
                warn!("failed to update wanted level.");
            }
        },
        None => warn!("failed to get wanted level offset.")
    }
}