use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};

pub fn read_usize(game: HANDLE, addr:usize) -> Option<usize> {
    unsafe {
        let mut read_value: usize = 0;
        let mut bytes_read = 0;
        match ReadProcessMemory(game, addr as *const _, &mut read_value as *mut usize as *mut _, size_of::<i32>(), Some(&mut bytes_read)) {
            Ok(_) => Some(read_value),
            Err(_) => None,
        }
    }
}

/*pub fn read_int(game: HANDLE, addr: usize) -> Option<i32> {
    unsafe {
        let mut read_value: i32 = -1;
        let mut bytes_read = 0;
        match ReadProcessMemory(game, addr as *const _, &mut read_value as *mut i32 as *mut _, size_of::<i32>(), Some(&mut bytes_read)) {
            Ok(_) => Some(read_value),
            Err(_) => None,
        }
    }
}*/

pub fn write_int(game: HANDLE, addr: usize, new_value: i32) -> bool {
    unsafe {
        let mut bytes_written = 0;
        WriteProcessMemory(game, addr as *mut _, &new_value as *const i32 as *const _, size_of::<i32>(), Some(&mut bytes_written)).is_ok()
    }
}

pub fn read_float(game: HANDLE, addr: usize) -> Option<f32> {
    unsafe {
        let mut read_value: f32 = -1.0;
        let mut bytes_read = 0;
        match ReadProcessMemory(game, addr as *const _, &mut read_value as *mut f32 as *mut _, size_of::<f32>(), Some(&mut bytes_read)) {
            Ok(_) => Some(read_value),
            Err(_) => None,
        }
    }
}

pub fn write_float(game: HANDLE, addr: usize, new_value: f32) -> bool {
    unsafe {
        let mut bytes_written = 0;
        WriteProcessMemory(game, addr as *mut _, &new_value as *const f32 as *const _, size_of::<i32>(), Some(&mut bytes_written)).is_ok()
    }
}