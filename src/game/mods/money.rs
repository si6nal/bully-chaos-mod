use log::warn;
use crate::game::bully::GameData;
use crate::memory::{game_offsets, memory};

pub fn remove_money(data: &GameData) {
    update_money(&data, 0);
}

pub fn give_max_money(data: &GameData) {
    update_money(&data, 100000000); // 1m
}

fn update_money(data: &GameData, money: i32) {
    match game_offsets::get_offset(data.handle, data.player_offset, game_offsets::PLAYER_MONEY_OFFSET) {
        Some(money_offset) => {
            if !memory::write_int(data.handle, money_offset, money) {
                warn!("failed to update money.");
            }
        },
        None => warn!("failed to get money offset.")
    }
}