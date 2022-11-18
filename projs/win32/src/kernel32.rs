use std::ffi::c_void;

use crate::types::*;

#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    pub fn OutputDebugStringW(lpOutputString: LPCWSTR) -> c_void;

    pub fn GetLastError() -> DWORD;
}
