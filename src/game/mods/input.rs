/*use std::time::{Duration, Instant};
use log::debug;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_SPACE;
use crate::game::bully::GameData;
use crate::windows::input;

pub async fn phoon(data: &GameData) {
    let start_time = Instant::now();
    loop {
        // check if 30 seconds has passed
        if start_time.elapsed().as_secs() >= 30 {
            break;
        }

        // send space key press
        input::send_keystroke(data.window_handle, VK_SPACE).await;
        debug!("test");

        // sleep for cpu usage
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}*/