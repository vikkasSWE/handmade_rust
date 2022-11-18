use std::ffi::c_void;

type INT = i32;
type UINT = u32;
type LPCWSTR = *const WCHAR;
type HWND = HANDLE;
type HANDLE = PVOID;
type PVOID = c_void;
type WCHAR = wchar_t;
#[allow(non_camel_case_types)]
type wchar_t = u16;

const MB_OK: u32 = 0x00000000;
const MB_ICONINFORMATION: u32 = 0x00000040;

#[link(name = "User32")]
extern "system" {
    pub fn MessageBoxW(hwnd: *mut HWND, lpText: LPCWSTR, lpCaption: LPCWSTR, uType: UINT) -> INT;
}

/// Turns a Rust string slice into a null-terminated utf-16 vector.
pub fn wide_null(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

fn main() {
    let message = "This is Handmade Hero.";
    let message_null = wide_null(message);
    let title = "Handmade Hero";
    let title_null = wide_null(title);

    unsafe {
        MessageBoxW(
            0 as *mut c_void,
            message_null.as_ptr(),
            title_null.as_ptr(),
            MB_OK | MB_ICONINFORMATION,
        );
    }
}
