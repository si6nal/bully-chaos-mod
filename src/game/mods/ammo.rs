use log::warn;
use crate::game::bully::GameData;
use crate::memory::{game_offsets, memory};

pub fn remove_all_ammo(data: &GameData) {
    update_ammo(&data, 0f32, game_offsets::PLAYER_EGG_AMMO_OFFSET);
    update_ammo(&data, 0f32, game_offsets::PLAYER_STINK_BOMBS_AMMO_OFFSET);
    update_ammo(&data, 0f32, game_offsets::PLAYER_SPUD_GUN_AMMO_OFFSET);
    update_ammo(&data, 0f32, game_offsets::PLAYER_ROCKET_LAUNCHER_AMMO_OFFSET);
    update_ammo(&data, 0f32, game_offsets::PLAYER_FIRECRACKER_AMMO_OFFSET);
}

pub fn give_all_ammo(data: &GameData) {
    update_ammo(&data, 999f32, game_offsets::PLAYER_EGG_AMMO_OFFSET);
    update_ammo(&data, 999f32, game_offsets::PLAYER_STINK_BOMBS_AMMO_OFFSET);
    update_ammo(&data, 999f32, game_offsets::PLAYER_SPUD_GUN_AMMO_OFFSET);
    update_ammo(&data, 999f32, game_offsets::PLAYER_ROCKET_LAUNCHER_AMMO_OFFSET);
    update_ammo(&data, 999f32, game_offsets::PLAYER_FIRECRACKER_AMMO_OFFSET);
}

fn update_ammo(data: &GameData, ammo: f32, ammo_addr: usize) {
    match game_offsets::get_offset(data.handle, data.player_offset, ammo_addr) {
        Some(ammo_offset) => {
            if !memory::write::<f32>(data.handle, ammo_offset, ammo) {
                warn!("failed to update ammo. offset: {}", ammo_offset);
            }
        },
        None => warn!("failed to get ammo offset at {}", ammo_addr)
    }
}
