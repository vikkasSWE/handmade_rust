use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

use win32::*;

static mut RUNNING: bool = true;

static mut BITMAP_INFO: BITMAPINFO = BITMAPINFO {
    bmiHeader: BITMAPINFOHEADER {
        biSize: 0,
        biWidth: 0,
        biHeight: 0,
        biPlanes: 0,
        biBitCount: 0,
        biCompression: 0,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    },
    bmiColors: [RGBQUAD {
        rgbBlue: 0,
        rgbGreen: 0,
        rgbRed: 0,
        rgbReserved: 0,
    }; 1],
};
static mut BITMAP_MEMORY: *mut c_void = null_mut();
static mut BITMAP_HANDLE: HBITMAP = null_mut();
static mut BITMAP_DEVICE_CONTEXT: HDC = null_mut();

unsafe fn win32_resize_dib_section(width: i32, height: i32) {
    if !BITMAP_HANDLE.is_null() {
        DeleteObject(BITMAP_HANDLE);
    }

    if BITMAP_DEVICE_CONTEXT.is_null() {
        BITMAP_DEVICE_CONTEXT = CreateCompatibleDC(null_mut());
    }

    BITMAP_INFO.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    BITMAP_INFO.bmiHeader.biWidth = width;
    BITMAP_INFO.bmiHeader.biHeight = height;
    BITMAP_INFO.bmiHeader.biPlanes = 1;
    BITMAP_INFO.bmiHeader.biBitCount = 32;
    BITMAP_INFO.bmiHeader.biCompression = BI_RGB as u32;

    BITMAP_HANDLE = CreateDIBSection(
        BITMAP_DEVICE_CONTEXT,
        &BITMAP_INFO,
        DIB_RGB_COLORS,
        &mut BITMAP_MEMORY,
        null_mut(),
        0,
    );
}

unsafe fn win32_update_window(
    device_context: *mut c_void,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) {
    StretchDIBits(
        device_context,
        x,
        y,
        width,
        height,
        x,
        y,
        width,
        height,
        BITMAP_MEMORY,
        &BITMAP_INFO,
        DIB_RGB_COLORS,
        SRCCOPY,
    );
}

pub unsafe extern "system" fn win32_main_window_callback(
    window: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let mut result = 0;

    match message {
        WM_SIZE => {
            println!("WM_SIZE");
            let mut client_rect = RECT::default();
            GetClientRect(window, &mut client_rect);
            let width = client_rect.right - client_rect.left;
            let height = client_rect.bottom - client_rect.top;
            win32_resize_dib_section(width, height);
        }
        WM_CLOSE => {
            println!("WM_CLOSE");
            RUNNING = false;
        }
        WM_ACTIVATEAPP => {
            println!("WM_ACTIVATEAPP");
        }
        WM_DESTROY => {
            println!("WM_DESTROY");
            RUNNING = false;
        }
        WM_PAINT => {
            println!("WM_PAINT");
            let mut paint = PAINTSTRUCT::default();
            let device_context = BeginPaint(window, &mut paint);
            let x = paint.rcPaint.left;
            let y = paint.rcPaint.top;
            let width = paint.rcPaint.right - paint.rcPaint.left;
            let height = paint.rcPaint.bottom - paint.rcPaint.top;

            win32_update_window(device_context, x, y, width, height);
            EndPaint(window, &mut paint);
        }
        _ => {
            result = DefWindowProcW(window, message, wparam, lparam);
        }
    }

    result
}

fn main() {
    let h_instance = unsafe { GetModuleHandleW(null()) };
    let window_class_name_null = wide_null("HandmadeHeroWindowClass");

    let window_class = WNDCLASSW {
        lpfnWndProc: Some(win32_main_window_callback),
        hInstance: h_instance,
        lpszClassName: window_class_name_null.as_ptr(),
        // style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        ..Default::default()
    };

    let window_name_null = wide_null("Handmade Hero");

    if unsafe { RegisterClassW(&window_class) } != 0 {
        let lparam: *mut i32 = Box::leak(Box::new(5_i32));
        let window_handle = unsafe {
            CreateWindowExW(
                0,
                window_class.lpszClassName,
                window_name_null.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                null_mut(),
                null_mut(),
                h_instance,
                lparam.cast(),
            )
        };

        if !window_handle.is_null() {
            'main_loop: loop {
                let mut message = MSG::default();
                let message_result = unsafe { GetMessageW(&mut message, null_mut(), 0, 0) };
                if message_result > 0 {
                    unsafe {
                        TranslateMessage(&message);
                        DispatchMessageW(&message);
                    }
                } else {
                    break;
                }

                if unsafe { RUNNING } == false {
                    break 'main_loop;
                }
            }
        } else {
            // TODO: Logging
        }
    } else {
        // TODO: Logging
    }
}
