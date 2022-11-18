use crate::types::*;

#[link(name = "Gdi32")]
extern "system" {
    pub fn PatBlt(hdc: HDC, x: INT, y: INT, w: INT, h: INT, rop: DWORD) -> BOOL;
}
