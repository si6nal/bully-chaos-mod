use log::info;
use windows::Win32::Foundation::{HANDLE, HWND};
use crate::memory::game_offsets;
use crate::windows::{processes, window};

pub struct GameData {
    pub process_id: u32,
    pub window_handle: HWND,
    pub handle: HANDLE,
    pub player_offset: usize,
    pub player_coordinates_offset: usize,
}

impl GameData {
    pub async fn get() -> GameData {
        // get pid of bully
        let bully_pid: u32;
        loop {
            match processes::get_process_id("Bully.exe") {
                Some(pid) => {
                    bully_pid = pid;
                    break;
                },
                None => {
                    info!("waiting for bully process.");
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        }

        info!("bully pid: {}", bully_pid);

        // get window handle for bully
        let window_handle: HWND;
        loop {
            match window::get_window_handle("Bully") {
                Some(bully_hwnd) => {
                    window_handle = bully_hwnd;
                    break;
                },
                None => {
                    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
                    info!("failed to get window handle, sleeping...");
                },
            }
        }

        info!("window handle: {:?}", window_handle);

        // get module address of the executable
        let base_addr = processes::get_module_address(bully_pid, "Bully.exe").expect("failed to get base address of game.");
        info!("base addr: 0x{:08x}", base_addr);

        // open handle to game
        let handle = processes::open_process(bully_pid).expect("failed to get handle to game.");

        // get offset of player
        let player_offset = game_offsets::get_player_offset(base_addr);
        info!("player offset: 0x{:08x}", player_offset);
        
        // get offset of player coordinates
        let player_coordinates_offset = game_offsets::get_player_coordinates_offset(base_addr);
        info!("player coordinates offset: 0x{:08x}", player_coordinates_offset);
        
        // create & return game data
        GameData {
            process_id: bully_pid,
            window_handle,
            handle,
            player_offset,
            player_coordinates_offset,
        }
    }
}