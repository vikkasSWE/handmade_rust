use std::ffi::c_void;

pub type INT = i32;
pub type UINT = u32;
pub type LPCWSTR = *const WCHAR;
pub type HWND = HANDLE;
pub type HANDLE = PVOID;
pub type PVOID = c_void;
pub type WCHAR = wchar_t;
#[allow(non_camel_case_types)]
pub type wchar_t = u16;
