use crate::types::*;

pub const CS_HREDRAW: u32 = 0x0002;
pub const CS_OWNDC: u32 = 0x0020;
pub const CS_VREDRAW: u32 = 0x0001;

pub const CW_USEDEFAULT: INT = 0x80000000_u32 as INT;

pub const MB_ICONINFORMATION: u32 = 0x00000040;
pub const MB_OK: u32 = 0x00000000;

pub const WS_CAPTION: u32 = 0x00C00000;
pub const WS_MAXIMIZEBOX: u32 = 0x00010000;
pub const WS_MINIMIZEBOX: u32 = 0x00020000;
pub const WS_OVERLAPPED: u32 = 0x00000000;
pub const WS_OVERLAPPEDWINDOW: u32 =
    WS_OVERLAPPED | WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX;
pub const WS_SYSMENU: u32 = 0x00080000;
pub const WS_THICKFRAME: u32 = 0x00040000;
pub const WS_VISIBLE: u32 = 0x10000000;

pub const WM_ACTIVATEAPP: u32 = 0x001C;
pub const WM_CLOSE: u32 = 0x0010;
pub const WM_DESTROY: u32 = 0x0002;
pub const WM_PAINT: u32 = 0x000F;
pub const WM_SIZE: u32 = 0x0005;

pub const WM_KEYUP: u32 = 0x0101;
pub const WM_KEYDOWN: u32 = 0x0100;
pub const WM_SYSKEYUP: u32 = 0x0105;
pub const WM_SYSKEYDOWN: u32 = 0x0104;

pub const BLACKNESS: DWORD = 0x00000042;
pub const WHITENESS: DWORD = 0x00FF0062;

pub const DIB_RGB_COLORS: u32 = 0;
pub const SRCCOPY: DWORD = 0x00CC0020;
pub const BI_RGB: LONG = 0;

pub const WM_QUIT: u32 = 0x0012;
pub const PM_REMOVE: u32 = 0x0001;

pub const MEM_COMMIT: u32 = 0x00001000;
pub const PAGE_READWRITE: u32 = 0x04;
pub const MEM_RELEASE: u32 = 0x00008000;

pub const VK_UP: u32 = 0x26;
pub const VK_LEFT: u32 = 0x25;
pub const VK_DOWN: u32 = 0x28;
pub const VK_RIGHT: u32 = 0x27;
pub const VK_ESCAPE: u32 = 0x1B;
pub const VK_SPACE: u32 = 0x20;

pub const ERROR_SUCCESS: LONG = 0;
pub const ERROR_DEVICE_NOT_CONNECTED: LONG = 0x48F;

pub const XUSER_MAX_COUNT: u32 = 4;

pub const XINPUT_GAMEPAD_DPAD_UP: u32 = 0x0001;
pub const XINPUT_GAMEPAD_DPAD_DOWN: u32 = 0x0002;
pub const XINPUT_GAMEPAD_DPAD_LEFT: u32 = 0x0004;
pub const XINPUT_GAMEPAD_DPAD_RIGHT: u32 = 0x0008;
pub const XINPUT_GAMEPAD_START: u32 = 0x0010;
pub const XINPUT_GAMEPAD_BACK: u32 = 0x0020;
pub const XINPUT_GAMEPAD_LEFT_THUMB: u32 = 0x0040;
pub const XINPUT_GAMEPAD_RIGHT_THUMB: u32 = 0x0080;
pub const XINPUT_GAMEPAD_LEFT_SHOULDER: u32 = 0x0100;
pub const XINPUT_GAMEPAD_RIGHT_SHOULDER: u32 = 0x0200;
pub const XINPUT_GAMEPAD_A: u32 = 0x1000;
pub const XINPUT_GAMEPAD_B: u32 = 0x2000;
pub const XINPUT_GAMEPAD_X: u32 = 0x4000;
pub const XINPUT_GAMEPAD_Y: u32 = 0x8000;
