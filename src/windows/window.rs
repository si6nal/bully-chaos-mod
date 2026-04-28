use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use log::warn;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{FALSE, HWND};
use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, GetForegroundWindow, GetWindowThreadProcessId, ShowWindow, SW_MAXIMIZE, SW_MINIMIZE};

pub fn get_window_handle(title: &str) -> Option<HWND> {
    unsafe {
        // convert rust string to wide windows string
        let wide_title = OsStr::new(title)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect::<Vec<u16>>();

        // find window
        match FindWindowW(None, PCWSTR(wide_title.as_ptr())) {
            Ok(handle) => return Some(handle),
            Err(e) => warn!("failed to get window handle for window \"{}\". error: {}", title, e),
        }
    }

    None
}

pub fn minimize_window(window: HWND) {
    unsafe {
        if ShowWindow(window, SW_MINIMIZE) == FALSE {
            warn!("failed to minimize window.");
        }
    }
}

pub fn maximize_window(window: HWND) {
    unsafe {
        if ShowWindow(window, SW_MAXIMIZE) == FALSE {
            warn!("failed to minimize window.");
        }
    }
}

pub fn get_focused_window_process_id() -> u32 {
    unsafe {
        let focused_hwnd = GetForegroundWindow();
        let mut focused_pid: u32 = 0;
        GetWindowThreadProcessId(focused_hwnd, Some(&mut focused_pid));
        focused_pid
    }
}