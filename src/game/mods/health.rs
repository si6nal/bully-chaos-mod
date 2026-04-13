use std::time::{Duration, Instant};
use log::warn;
use crate::game::bully::GameData;
use crate::memory::{game_offsets, memory};

const DEFAULT_MAX_HEALTH: f32 = 200f32;

pub async fn give_invincibility(data: &GameData) {
    // get current health (we will revert to it)
    let original_health = get_health(&data);

    // get max health
    let max_health = get_max_health(&data);

    // get current time for duration check
    let start_time = Instant::now();
    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            // revert to original health
            if let Some(original_health) = original_health {
                update_health(&data, original_health);
            }

            break;
        }

        // set health to max
        update_health(&data, max_health);

        // sleep for cpu usage
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

pub fn heal(data: &GameData) {
    // get max health
    let max_health = get_max_health(&data);

    // set health to max health
    update_health(&data, max_health);
}

pub fn no_health(data: &GameData) {
    update_health(&data, 1f32);
}

pub fn suicide(data: &GameData) {
    update_health(&data, 0f32);
}

pub fn get_health(data: &GameData) -> Option<f32> {
    match game_offsets::get_offset(data.handle, data.player_offset, game_offsets::PLAYER_HEALTH_OFFSET) {
        Some(health_offset) => memory::read_float(data.handle, health_offset),
        None => {
            warn!("failed to get health offset.");
            None
        }
    }
}

fn get_max_health(data: &GameData) -> f32 {
    match game_offsets::get_offset(data.handle, data.player_offset, game_offsets::PLAYER_MAX_HEALTH_OFFSET) {
        Some(max_health_offset) => {
            // read max health value
            match memory::read_float(data.handle, max_health_offset) {
                Some(max_health) => {
                    // check if the read failed
                    if max_health <= 0f32 {
                        DEFAULT_MAX_HEALTH
                    } else {
                        max_health
                    }
                },
                None => DEFAULT_MAX_HEALTH
            }
        },
        None => DEFAULT_MAX_HEALTH
    }
}

pub fn update_health(data: &GameData, health: f32) {
    match game_offsets::get_offset(data.handle, data.player_offset, game_offsets::PLAYER_HEALTH_OFFSET) {
        Some(health_offset) => {
            if !memory::write_float(data.handle, health_offset, health) {
                warn!("failed to update health.");
            }
        },
        None => warn!("failed to get health offset.")
    }
}