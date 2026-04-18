use log::warn;
use crate::game::bully::GameData;
use crate::memory::{game_offsets, memory};

pub fn remove_money(data: &GameData) {
    update_money(&data, 0);
}

pub fn check_bounced(data: &GameData) {
    update_money(&data, 2500); // $25
}

pub fn spare_change(data: &GameData) {
    // get current money
    match get_money(&data) {
        Some(current_money) => {
            // get random amount to add ($00.01-$10.00)
            let change = rand::random_range(0001..1000);

            // add change & update money
            update_money(&data, current_money + change);
        },
        None => check_bounced(&data)
    }
}

pub fn give_max_money(data: &GameData) {
    update_money(&data, 100000000); // $1m
}

fn get_money(data: &GameData) -> Option<i32> {
    match game_offsets::get_offset(data.handle, data.player_offset, game_offsets::PLAYER_MONEY_OFFSET) {
        Some(money_offset) => {
            match memory::read::<i32>(data.handle, money_offset) {
                Some(money) => Some(money),
                None => {
                    warn!("failed to read money.");
                    None
                }
            }
        },
        None => {
            warn!("failed to get money offset.");
            None
        }
    }
}

fn update_money(data: &GameData, money: i32) {
    match game_offsets::get_offset(data.handle, data.player_offset, game_offsets::PLAYER_MONEY_OFFSET) {
        Some(money_offset) => {
            if !memory::write::<i32>(data.handle, money_offset, money) {
                warn!("failed to update money.");
            }
        },
        None => warn!("failed to get money offset.")
    }
}