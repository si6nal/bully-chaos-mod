use std::ffi::CStr;
use std::time::Duration;
use log::{info, warn};
use windows::Win32::Foundation::{CloseHandle, HANDLE, INVALID_HANDLE_VALUE};
use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, Module32First, Module32Next, Process32First, Process32Next, Thread32First, Thread32Next, MODULEENTRY32, PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPMODULE32, TH32CS_SNAPPROCESS, TH32CS_SNAPTHREAD, THREADENTRY32};
use windows::Win32::System::Threading::{GetExitCodeProcess, OpenProcess, OpenThread, ResumeThread, SuspendThread, TerminateProcess, PROCESS_ALL_ACCESS, THREAD_ALL_ACCESS};

pub fn get_process_id(target_executable_name: &str) -> Option<u32> {
    unsafe {
        // create snapshot of current processes
        match CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) {
            Ok(snapshot) => {
                // check if the snapshot is valid
                if snapshot == INVALID_HANDLE_VALUE {
                    warn!("failed to create process snapshot (INVALID_HANDLE_VALUE).");
                    let _ = CloseHandle(snapshot);
                    return None;
                }

                // create process entry struct
                let mut proc_entry = PROCESSENTRY32::default();

                // set size of process entry struct
                proc_entry.dwSize = size_of::<PROCESSENTRY32>() as u32;

                // try to get first process
                if Process32First(snapshot, &mut proc_entry).is_err() {
                    warn!("failed to get process entry.");
                    let _ = CloseHandle(snapshot);
                    return None;
                }

                // iterate through processes
                loop {
                    // get executable name of process
                    let executable_name = CStr::from_ptr(proc_entry.szExeFile.as_ptr()).to_string_lossy().to_string();

                    // check if it is the process we're looking for
                    if executable_name.eq(target_executable_name) {
                        let _ = CloseHandle(snapshot);
                        return Some(proc_entry.th32ProcessID);
                    }

                    // try to get next process
                    if Process32Next(snapshot, &mut proc_entry).is_err() {
                        let _ = CloseHandle(snapshot);
                        break;
                    }
                }
            },
            Err(e) => warn!("failed to create process snapshot. error: {}", e)
        }
    }

    None
}

pub fn get_module_address(process_id: u32, target_module_name: &str) -> Option<usize> {
    unsafe {
        // create snapshot of loaded modules in process
        match CreateToolhelp32Snapshot(TH32CS_SNAPMODULE | TH32CS_SNAPMODULE32, process_id) {
            Ok(snapshot) => {
                // check if the snapshot is valid
                if snapshot == INVALID_HANDLE_VALUE {
                    warn!("failed to create module snapshot (INVALID_HANDLE_VALUE).");
                    let _ = CloseHandle(snapshot);
                    return None;
                }

                // create module entry struct
                let mut module_entry = MODULEENTRY32::default();

                // set size of process entry struct
                module_entry.dwSize = size_of::<MODULEENTRY32>() as u32;

                // try to get first module
                if Module32First(snapshot, &mut module_entry).is_err() {
                    warn!("failed to get module entry.");
                    let _ = CloseHandle(snapshot);
                    return None;
                }

                // iterate through modules
                loop {
                    // get module name
                    let module_name = CStr::from_ptr(module_entry.szModule.as_ptr()).to_string_lossy().to_string();

                    // check if it is the module we're looking for
                    if module_name.eq_ignore_ascii_case(target_module_name) {
                        let _ = CloseHandle(snapshot);
                        return Some(module_entry.modBaseAddr as usize);
                    }

                    // try to get next module
                    if Module32Next(snapshot, &mut module_entry).is_err() {
                        let _ = CloseHandle(snapshot);
                        break;
                    }
                }
            },
            Err(e) => warn!("failed to create module snapshot. error: {}", e)
        }
    }

    None
}

pub fn open_process(process_id: u32) -> Option<HANDLE> {
    unsafe {
        match OpenProcess(PROCESS_ALL_ACCESS, false, process_id) {
            Ok(handle) => {
                if handle == INVALID_HANDLE_VALUE {
                    warn!("failed to get handle to process (INVALID_HANDLE_VALUE).");
                    return None;
                }
                
                Some(handle)
            },
            Err(e) => {
                warn!("failed to get handle to process. error: {}", e);
                None
            }
        }
    }
}

pub async fn pause_process(process_id: u32, pause_ms: u64) {
    unsafe {
        // create snapshot of current threads
        match CreateToolhelp32Snapshot(TH32CS_SNAPTHREAD, 0) {
            Ok(snapshot) => {
                // check if the snapshot is valid
                if snapshot == INVALID_HANDLE_VALUE {
                    warn!("failed to create thread snapshot (INVALID_HANDLE_VALUE).");
                    let _ = CloseHandle(snapshot);
                    return;
                }

                // create thread entry struct
                let mut thread_entry = THREADENTRY32::default();

                // set size of thread entry struct
                thread_entry.dwSize = size_of::<THREADENTRY32>() as u32;

                // try to get first thread
                if Thread32First(snapshot, &mut thread_entry).is_err() {
                    warn!("failed to get thread entry.");
                    let _ = CloseHandle(snapshot);
                    return;
                }

                // iterate through threads
                loop {
                    // check if the thread's owner pid is the target process id
                    if thread_entry.th32OwnerProcessID == process_id {
                        // open handle to thread
                        if let Ok(thread_handle) = OpenThread(THREAD_ALL_ACCESS, false, thread_entry.th32ThreadID) {
                            // suspend thread, sleep, resume & close handle to thread
                            SuspendThread(thread_handle);
                            tokio::time::sleep(Duration::from_millis(pause_ms)).await;
                            ResumeThread(thread_handle);
                            let _ = CloseHandle(thread_handle);
                        }

                        // close snapshot handle
                        let _ = CloseHandle(snapshot);
                        break;
                    }

                    // try to get next thread
                    if Thread32Next(snapshot, &mut thread_entry).is_err() {
                        let _ = CloseHandle(snapshot);
                        break;
                    }
                }
            },
            Err(e) => warn!("failed to create thread snapshot. error: {}", e)
        }
    }
}

pub fn terminate_process(handle: HANDLE) {
    unsafe {
        match TerminateProcess(handle, 0) {
            Ok(_) => info!("successfully terminated process"),
            Err(e) => warn!("failed to terminate process. error: {}", e)
        }
    }
}

pub fn is_process_active(handle: HANDLE) -> bool {
    unsafe {
        let mut exit_code = 0u32;
        match GetExitCodeProcess(handle, &mut exit_code) {
            Ok(_) => exit_code == 0x103_u32, // STILL_ACTIVE
            Err(e) => {
                warn!("failed to check if process is still active. error: {}", e);
                false
            }
        }
    }
}