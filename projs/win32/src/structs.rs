#![allow(non_snake_case)]

use crate::types::*;

#[repr(C)]
pub struct tagWNDCLASSW {
    pub style: UINT,
    pub lpfnWndProc: WNDPROC,
    pub cbClsExtra: INT,
    pub cbWndExtra: INT,
    pub hInstance: HINSTANCE,
    pub hIcon: HICON,
    pub hCursor: HCURSOR,
    pub hbrBackground: HBRUSH,
    pub lpszMenuName: LPCWSTR,
    pub lpszClassName: LPCWSTR,
}

pub type WNDCLASSW = tagWNDCLASSW;

impl Default for tagWNDCLASSW {
    #[inline]
    #[must_use]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
pub struct tagMSG {
    pub hwnd: HWND,
    pub message: UINT,
    pub wParam: WPARAM,
    pub lParam: LPARAM,
    pub time: DWORD,
    pub pt: POINT,
    pub lPrivate: DWORD,
}

pub type MSG = tagMSG;
pub type LPMSG = *mut MSG;

impl Default for tagMSG {
    #[inline]
    #[must_use]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
pub struct tagPOINT {
    pub x: LONG,
    pub y: LONG,
}

pub type POINT = tagPOINT;

#[repr(C)]
pub struct tagPAINTSTRUCT {
    pub hdc: HDC,
    pub fErase: BOOL,
    pub rcPaint: RECT,
    pub fRestore: BOOL,
    pub fIncUpdate: BOOL,
    pub rgbReserved: [BYTE; 32],
}

pub type PAINTSTRUCT = tagPAINTSTRUCT;
pub type LPPAINTSTRUCT = *mut PAINTSTRUCT;

impl Default for tagPAINTSTRUCT {
    #[inline]
    #[must_use]
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}

#[repr(C)]
pub struct tagRECT {
    pub left: LONG,
    pub top: LONG,
    pub right: LONG,
    pub bottom: LONG,
}

pub type RECT = tagRECT;
