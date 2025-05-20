use std::mem;
use winapi::shared::minwindef::{DWORD, LPVOID};
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
use winapi::um::memoryapi::VirtualQueryEx;
use winapi::um::processthreadsapi::{GetCurrentProcess, GetCurrentProcessId};
use winapi::um::sysinfoapi::{GetSystemInfo, SYSTEM_INFO};
use winapi::um::winnt::{HANDLE, MEMORY_BASIC_INFORMATION, PVOID, SIZE_T};

fn main() {
    let this_pid: DWORD;
    let this_proc: HANDLE;
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMORY_BASIC_INFORMATION;
    const MEMINFO_SIZE: usize = mem::size_of::<MEMORY_BASIC_INFORMATION>();
    
    unsafe {
        base_addr = mem::zeroed();
        proc_info = mem::zeroed();
        mem_info = mem::zeroed();
    }
    
    unsafe {
        this_pid = GetCurrentProcessId();
        this_proc = GetCurrentProcess();
        GetSystemInfo(&mut proc_info);
    };
    
    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;
    
    println!("{:?} @ {:?}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:?}, max: {:?}", min_addr, max_addr);
    
    loop {
        let rc: SIZE_T = unsafe {
            VirtualQueryEx(
                this_proc, 
                base_addr,
                &mut mem_info, 
                MEMINFO_SIZE as SIZE_T
            )
        };
        
        if rc == 0 {
            break
        }
        
        println!("{:#?}", mem_info);
        base_addr = ((base_addr as usize) + mem_info.RegionSize) as PVOID;
    }
}
