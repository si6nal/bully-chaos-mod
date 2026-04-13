/*use std::time::Duration;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, SendInput, INPUT, INPUT_0, INPUT_KEYBOARD, KEYBDINPUT, KEYBD_EVENT_FLAGS, KEYEVENTF_KEYUP, VIRTUAL_KEY};
use windows::Win32::UI::WindowsAndMessaging::{PostMessageW, WM_KEYDOWN, WM_KEYUP};


pub async fn send_keystroke(window_handle: HWND, key: VIRTUAL_KEY) {
    unsafe {
        let _ = PostMessageW(Some(window_handle), WM_KEYDOWN, WPARAM(key.0 as usize), LPARAM(0));
        tokio::time::sleep(Duration::from_millis(1)).await;
        let _ = PostMessageW(Some(window_handle), WM_KEYUP, WPARAM(key.0 as usize), LPARAM(0));

        /*let key_states = [
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: key,
                        wScan: 0,
                        dwFlags: KEYBD_EVENT_FLAGS(0),
                        time: 0,
                        dwExtraInfo: 0,
                    }
                },
            },
            INPUT {
                r#type: INPUT_KEYBOARD,
                Anonymous: INPUT_0 {
                    ki: KEYBDINPUT {
                        wVk: key,
                        wScan: 0,
                        dwFlags: KEYEVENTF_KEYUP,
                        time: 0,
                        dwExtraInfo: 0,
                    }
                },
            },
        ];

        SendInput(&key_states, size_of::<INPUT>() as i32);*/
    }
}*/