#![allow(non_camel_case_types)]

use std::ffi::c_void;

pub type ATOM = WORD;
pub type BOOL = INT;
pub type BYTE = u8;
pub type DWORD = u32;
pub type HANDLE = PVOID;
pub type HBITMAP = HANDLE;
pub type HBRUSH = HANDLE;
pub type HCURSOR = HICON;
pub type HDC = HANDLE;
pub type HGDIOBJ = HANDLE;
pub type HICON = HANDLE;
pub type HINSTANCE = HANDLE;
pub type HMENU = HANDLE;
pub type HMODULE = HINSTANCE;
pub type HWND = HANDLE;
pub type INT = i32;
pub type INT_PTR = isize;
pub type LONG = i32;
pub type LONG_PTR = isize;
pub type LPARAM = LONG_PTR;
pub type LPCWSTR = *const WCHAR;
pub type LPVOID = *mut c_void;
pub type LRESULT = LONG_PTR;
pub type PVOID = *mut c_void;
pub type SIZE_T = ULONG_PTR;
pub type ULONG_PTR = usize;
pub type UINT = u32;
pub type UINT_PTR = usize;
pub type WCHAR = wchar_t;
pub type wchar_t = u16;
pub type WORD = u16;
pub type WPARAM = UINT_PTR;

pub type WNDPROC = Option<
    unsafe extern "system" fn(hwnd: HWND, uMsg: UINT, wParam: WPARAM, lParam: LPARAM) -> LRESULT,
>;
