use std::{env, fs};
use log::{info, warn};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Diagnostics::Debug::{ReadProcessMemory, WriteProcessMemory, IMAGE_DIRECTORY_ENTRY_EXPORT, IMAGE_NT_HEADERS32};
use windows::Win32::System::Memory::{VirtualAllocEx, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE};
use windows::Win32::System::SystemServices::{IMAGE_DOS_HEADER, IMAGE_EXPORT_DIRECTORY};
use windows::Win32::System::Threading::{CreateRemoteThread, GetExitCodeThread, WaitForSingleObject, INFINITE};
use crate::windows::processes;

pub fn get_full_dll_path(dll_name: &str) -> String {
    // get current working directory
    env::current_dir()
        .expect("failed to get current directory")
        .join(dll_name)
        .to_str()
        .expect("failed to convert full dll path to a string")
        .to_string()
}

pub fn load_library(handle: HANDLE, process_id: u32, dll_path: String) -> bool {
    // verify the dll exists
    match fs::exists(dll_path.clone()) {
        Ok(dll_exists) => {
            if !dll_exists {
                warn!("dll does not exist: {}", dll_path);
                return false;
            }
        },
        Err(e) => {
            warn!("failed to check if dll exists. error: {}", e);
            return false;
        }
    }

    // get module base for kernel32
    match processes::get_module_address(process_id, "kernel32.dll") {
        Some(kernel32_base_addr) => {
            info!("kernel32 address: 0x{:x}", kernel32_base_addr);
            unsafe {
                // convert dll path string to bytes
                let dll_path_bytes = dll_path.as_bytes();

                // allocate memory for dll path in process
                let p_dll_path = VirtualAllocEx(handle, None, dll_path_bytes.len(), MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
                if p_dll_path.is_null() {
                    warn!("failed to allocate memory for dll path in process.");
                    return false;
                }

                // write dll to the memory we just allocated
                if WriteProcessMemory(handle, p_dll_path, dll_path_bytes.as_ptr() as _, dll_path_bytes.len(), None).is_err() {
                    warn!("failed to write dll path to memory in process.");
                    return false;
                }

                // get address of LoadLibraryA in remote process
                match get_proc_address(handle, kernel32_base_addr, "LoadLibraryA") {
                    Some(load_library_addr) => {
                        info!("LoadLibraryA address: 0x{:x}", load_library_addr);

                        // create remote thread calling LoadLibraryA
                        match CreateRemoteThread(handle, None, 0, std::mem::transmute(load_library_addr as *const ()), Some(p_dll_path as _), 0, None) {
                            Ok(thread_handle) => {
                                // wait for thread to finish executing
                                WaitForSingleObject(thread_handle, INFINITE);

                                // get thread exit code
                                /*let mut thread_exit_code: u32 = 0;
                                let mut thread_success_exit = true; // assume the thread will exit successfully
                                if let Ok(_) = GetExitCodeThread(thread_handle, &mut thread_exit_code) {
                                    // check if the thread exit code is 0, if it is then the thread failed to execute
                                    if thread_exit_code == 0 {
                                        thread_success_exit = false;
                                    }
                                }*/

                                // close thread handle
                                let _ = CloseHandle(thread_handle);

                                // assume the library was loaded
                                info!("successfully loaded dll: {}", dll_path);
                                true//thread_success_exit
                            },
                            Err(e) => {
                                warn!("failed to create remote thread. error: {}", e);
                                false
                            }
                        }
                    },
                    None => {
                        warn!("failed to get address of LoadLibraryA");
                        false
                    }
                }
            }
        },
        None => {
            warn!("failed to get base address of kernel32.dll");
            false
        }
    }
}

fn get_proc_address(handle: HANDLE, module_base_addr: usize, fn_name: &str) -> Option<usize> {
    unsafe {
        // create dos header struct & read from process
        let mut dos_header = IMAGE_DOS_HEADER::default();
        if ReadProcessMemory(handle, module_base_addr as *const _, &mut dos_header as *mut IMAGE_DOS_HEADER as *mut _, size_of::<IMAGE_DOS_HEADER>(), None).is_err() {
            warn!("failed to read dos header from process.");
            return None;
        }

        // create nt headers struct & read from process
        let mut nt_headers = IMAGE_NT_HEADERS32::default();
        if ReadProcessMemory(handle, (module_base_addr + dos_header.e_lfanew as usize) as *const _, &mut nt_headers as *mut IMAGE_NT_HEADERS32 as *mut _, size_of::<IMAGE_NT_HEADERS32>(), None).is_err() {
            warn!("failed to read nt headers from process.");
            return None;
        }

        // get address of exports table
        let export_table_addr = nt_headers.OptionalHeader.DataDirectory[IMAGE_DIRECTORY_ENTRY_EXPORT.0 as usize].VirtualAddress;

        // check if the module has exports
        if export_table_addr == 0 {
            warn!("module has no exports.");
            return None;
        }

        // create export directory struct & read from process
        let mut export_directory = IMAGE_EXPORT_DIRECTORY::default();
        if ReadProcessMemory(handle, (module_base_addr + export_table_addr as usize) as *const _, &mut export_directory as *mut IMAGE_EXPORT_DIRECTORY as *mut _, size_of::<IMAGE_EXPORT_DIRECTORY>(), None).is_err() {
            warn!("failed to read export directory from process.");
            return None;
        }

        // create arrays for export data
        let mut name_addrs: Vec<u32> = vec![0; export_directory.NumberOfNames as usize];
        let mut ordinals: Vec<u16> = vec![0; export_directory.NumberOfNames as usize];
        let mut functions: Vec<u32> = vec![0; export_directory.NumberOfFunctions as usize];

        // read name addresses
        let _ = ReadProcessMemory(handle, (module_base_addr + export_directory.AddressOfNames as usize) as *const _, name_addrs.as_mut_ptr() as _, export_directory.NumberOfNames as usize * size_of::<u32>(), None);

        // read ordinals
        let _ = ReadProcessMemory(handle, (module_base_addr + export_directory.AddressOfNameOrdinals as usize) as *const _, ordinals.as_mut_ptr() as _, export_directory.NumberOfNames as usize * size_of::<u16>(), None);

        // read function addresses
        let _ = ReadProcessMemory(handle, (module_base_addr + export_directory.AddressOfFunctions as usize) as *const _, functions.as_mut_ptr() as _, export_directory.NumberOfFunctions as usize * size_of::<u32>(), None);

        // create buffer for function names
        let mut fn_name_buf: Vec<u8> = vec![0u8; 256];
        let mut buf_bytes_read = 0;

        // find target function
        for idx in 0..export_directory.NumberOfNames as usize {
            // read function name
            if ReadProcessMemory(handle, (module_base_addr + *name_addrs.get(idx).unwrap() as usize) as *const _, fn_name_buf.as_mut_ptr() as _, fn_name_buf.len(), Some(&mut buf_bytes_read)).is_err() {
                continue;
            }

            // convert c-style string to rust string
            let cur_fn_name_slice = &fn_name_buf[..buf_bytes_read];
            let cur_fn_name_null_pos = cur_fn_name_slice.iter().position(|&c| c == 0).unwrap_or(cur_fn_name_slice.len());
            let cur_fn_name = String::from_utf8_lossy(&cur_fn_name_slice[..cur_fn_name_null_pos]);

            // check if the current function is the target function
            if cur_fn_name.eq_ignore_ascii_case(fn_name) {
                // get ordinal & function offset
                let ordinal = ordinals.get(idx).unwrap();
                let fn_offset = functions.get(*ordinal as usize).unwrap();
                info!("{} offset: 0x{:x}", fn_name, fn_offset);

                // return function address
                return Some(module_base_addr + *fn_offset as usize);
            }
        }
    }

    warn!("failed to get address of {}", fn_name);
    None
}