use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use log::warn;
use windows::core::PCWSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::FindWindowW;

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