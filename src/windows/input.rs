use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VIRTUAL_KEY, VK_A, VK_D, VK_S, VK_SPACE, VK_W};

pub fn is_moving() -> bool {
    is_key_down(VK_W) || is_key_down(VK_A) || is_key_down(VK_S) || is_key_down(VK_D)
}

pub fn has_jumped() -> bool {
    is_key_pressed(VK_SPACE)
}

fn is_key_down(key: VIRTUAL_KEY) -> bool {
    unsafe {
        (GetAsyncKeyState(key.0 as i32) as u16 & 0x8000) != 0
    }
}

fn is_key_pressed(key: VIRTUAL_KEY) -> bool {
    unsafe {
        (GetAsyncKeyState(key.0 as i32) & 0x0001) != 0
    }
}