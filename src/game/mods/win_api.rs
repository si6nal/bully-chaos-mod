use std::time::{Duration, Instant};
use crate::game::bully::GameData;
use crate::windows::{processes, window};

pub async fn fake_crash(data: &GameData) {
    // suspend process for 4 seconds
    processes::pause_process(data.process_id, 4000).await;
}

pub async fn real_crash(data: &GameData) {
    // pause process (fake crash)
    processes::pause_process(data.process_id, 4000).await;

    // "crash" (close) process
    processes::terminate_process(data.handle);
}

pub async fn minimize_game(data: &GameData) {
    window::minimize_window(data.window_handle);
}

pub async fn lag(data: &GameData) {
    let start_time = Instant::now();
    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }

        // suspend game & sleep
        processes::pause_process(data.process_id, 25).await;
        tokio::time::sleep(Duration::from_millis(45)).await;
    }
}

pub async fn lag_stutter(data: &GameData) {
    let start_time = Instant::now();
    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }

        // suspend game & sleep
        processes::pause_process(data.process_id, 25).await;
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}