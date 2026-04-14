use windows::Win32::Foundation::HANDLE;
use crate::memory::memory;

pub fn get_player_offset(base_addr: usize) -> usize {
    base_addr + PLAYER_POINTER_OFFSET
}

pub fn get_player_coordinates_offset(base_addr: usize) -> usize {
    base_addr + PLAYER_COORDINATES_POINTER_OFFSET
}

pub fn get_offset(game: HANDLE, base_offset: usize, data_offset: usize) -> Option<usize> {
    match memory::read::<usize>(game, base_offset) {
        Some(addr) => Some(addr + data_offset),
        None => None
    }
}

const PLAYER_POINTER_OFFSET: usize = 0x1CC438C;
const PLAYER_COORDINATES_POINTER_OFFSET: usize = 0x082AA68;

pub const PLAYER_COORDINATES_OFFSET: usize = 0x30;

pub const PLAYER_MONEY_OFFSET: usize = 0x1CA0;
pub const PLAYER_HEALTH_OFFSET: usize = 0x1CB8;
pub const PLAYER_MAX_HEALTH_OFFSET: usize = 0x1CA4;
pub const PLAYER_WANTED_LEVEL_OFFSET: usize = 0x1D40;

pub const PLAYER_EGG_AMMO_OFFSET: usize = 0x378F8;
pub const PLAYER_STINK_BOMBS_AMMO_OFFSET: usize = 0x37A10;
pub const PLAYER_SPUD_GUN_AMMO_OFFSET: usize = 0x379C0;
pub const PLAYER_ROCKET_LAUNCHER_AMMO_OFFSET: usize = 0x37970;
pub const PLAYER_FIRECRACKER_AMMO_OFFSET: usize = 0x37A8;
