use crate::types::*;

#[link(name = "User32")]
extern "system" {
    pub fn MessageBoxW(hwnd: *mut HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> INT;
}
