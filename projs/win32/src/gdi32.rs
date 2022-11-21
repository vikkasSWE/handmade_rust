use std::ffi::c_void;

use crate::{structs::*, types::*};

#[link(name = "Gdi32")]
extern "system" {
    pub fn PatBlt(hdc: HDC, x: INT, y: INT, w: INT, h: INT, rop: DWORD) -> BOOL;
    pub fn DeleteObject(ho: HGDIOBJ) -> BOOL;

    pub fn CreateCompatibleDC(hdc: HDC) -> HDC;

    pub fn StretchDIBits(
        hdc: HDC,
        xDest: INT,
        yDest: INT,
        DestWidth: INT,
        DestHeight: INT,
        xSrc: INT,
        ySrc: INT,
        SrcWidth: INT,
        SrcHeight: INT,
        lpBits: *const c_void,
        lpbmi: *const BITMAPINFO,
        iUsage: UINT,
        rop: DWORD,
    ) -> INT;

    // TODO double pointer
    pub fn CreateDIBSection(
        hdc: HDC,
        pbmi: *const BITMAPINFO,
        usage: UINT,
        ppvBits: *mut *mut c_void,
        hSection: HANDLE,
        offset: DWORD,
    ) -> HBITMAP;
}
