use std::mem::MaybeUninit;
use windows::Win32::Foundation::HANDLE;
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory};

pub fn read<T: Copy>(game: HANDLE, addr: usize) -> Option<T> {
    unsafe {
        let mut read_value = MaybeUninit::<T>::uninit();
        match ReadProcessMemory(game, addr as *const _, read_value.as_mut_ptr() as *mut _, size_of::<T>(), None) {
            Ok(_) => Some(read_value.assume_init()),
            Err(_) => None,
        }
    }
}

pub fn write<T: Copy>(game: HANDLE, addr: usize, new_value: T) -> bool {
    unsafe {
        WriteProcessMemory(game, addr as *mut _, &new_value as *const T as *const _, size_of::<T>(), None).is_ok()
    }
}