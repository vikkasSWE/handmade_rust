use std::ffi::c_void;

use crate::types::*;

#[link(name = "Kernel32")]
extern "system" {
    pub fn GetModuleHandleW(lpModuleName: LPCWSTR) -> HMODULE;

    pub fn OutputDebugStringW(lpOutputString: LPCWSTR) -> c_void;

    pub fn GetLastError() -> DWORD;

    pub fn VirtualAlloc(
        lpAddress: LPVOID,
        dwSize: SIZE_T,
        flAllocationType: DWORD,
        flProtect: DWORD,
    ) -> LPVOID;

    pub fn VirtualFree(lpAddress: LPVOID, dwSize: SIZE_T, dwFreeType: DWORD) -> BOOL;
}
