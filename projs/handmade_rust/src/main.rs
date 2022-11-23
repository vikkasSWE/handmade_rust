use std::{
    ffi::c_void,
    ptr::{null, null_mut},
};

use win32::*;

static mut GLOBAL_RUNNING: bool = true;
static mut GLOBAL_BACKBUFFER: Win32OffscreenBuffer = {
    let info = BITMAPINFO {
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

    let memory = null_mut();
    let width = 0;
    let height = 0;
    let pitch = 0;

    Win32OffscreenBuffer {
        info,
        memory,
        width,
        height,
        pitch,
    }
};

struct Win32OffscreenBuffer {
    info: BITMAPINFO,
    memory: *mut c_void,
    width: i32,
    height: i32,
    pitch: i32,
}

struct Win32WindowDimensions {
    width: i32,
    height: i32,
}

fn win32_get_window_dimension(window: HWND) -> Win32WindowDimensions {
    let mut client_rect = RECT::default();

    unsafe { GetClientRect(window, &mut client_rect) };

    Win32WindowDimensions {
        width: client_rect.right - client_rect.left,
        height: client_rect.bottom - client_rect.top,
    }
}

unsafe fn render_weird_gradient(
    buffer: &Win32OffscreenBuffer,
    blue_offset: i32,
    green_offset: i32,
) {
    let mut row = buffer.memory as *mut u8;
    for y in 0..buffer.height {
        let mut pixel = row as *mut u32;
        for x in 0..buffer.width {
            let blue = x + blue_offset;
            let green = y + green_offset;

            *pixel = (green << 8 | blue) as u32;
            pixel = pixel.wrapping_add(1);
        }
        row = row.wrapping_add(buffer.pitch as usize);
    }
}

unsafe fn win32_resize_dib_section(buffer: &mut Win32OffscreenBuffer, width: i32, height: i32) {
    if !buffer.memory.is_null() {
        VirtualFree(buffer.memory, 0, MEM_RELEASE);
    }

    buffer.width = width;
    buffer.height = height;

    let bytes_per_pixel = 4;

    buffer.info.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    buffer.info.bmiHeader.biWidth = buffer.width;
    buffer.info.bmiHeader.biHeight = -buffer.height;
    buffer.info.bmiHeader.biPlanes = 1;
    buffer.info.bmiHeader.biBitCount = 32;
    buffer.info.bmiHeader.biCompression = BI_RGB as u32;

    let bitmap_memory_size = (buffer.width * buffer.height) * bytes_per_pixel;
    buffer.memory = VirtualAlloc(
        null_mut(),
        bitmap_memory_size as usize,
        MEM_COMMIT,
        PAGE_READWRITE,
    );
    buffer.pitch = width * bytes_per_pixel;
}

unsafe fn win32_display_buffer_in_window(
    device_context: *mut c_void,
    window_width: i32,
    window_height: i32,
    buffer: &Win32OffscreenBuffer,
) {
    StretchDIBits(
        device_context,
        0,
        0,
        window_width,
        window_height,
        0,
        0,
        buffer.width,
        buffer.height,
        buffer.memory,
        &buffer.info,
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
        WM_CLOSE => {
            GLOBAL_RUNNING = false;
        }
        WM_ACTIVATEAPP => {
            println!("WM_ACTIVATEAPP");
        }
        WM_DESTROY => {
            GLOBAL_RUNNING = false;
        }
        WM_PAINT => {
            let mut paint = PAINTSTRUCT::default();
            let device_context = BeginPaint(window, &mut paint);

            let dimension = win32_get_window_dimension(window);
            win32_display_buffer_in_window(
                device_context,
                dimension.width,
                dimension.height,
                &GLOBAL_BACKBUFFER,
            );
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
    unsafe { win32_resize_dib_section(&mut GLOBAL_BACKBUFFER, 1280, 720) };

    let window_class_name_null = wide_null("HandmadeHeroWindowClass");
    let window_class = WNDCLASSW {
        lpfnWndProc: Some(win32_main_window_callback),
        hInstance: h_instance,
        lpszClassName: window_class_name_null.as_ptr(),
        style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
        ..Default::default()
    };

    if unsafe { RegisterClassW(&window_class) } != 0 {
        let window_name_null = wide_null("Handmade Hero");
        let window = unsafe {
            CreateWindowExW(
                0,
                window_class.lpszClassName,
                window_name_null.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                1480,
                920,
                null_mut(),
                null_mut(),
                h_instance,
                null_mut(),
            )
        };

        if !window.is_null() {
            let device_context = unsafe { GetDC(window) };

            let mut x_offset = 0;
            let mut y_offset = 0;

            'main_loop: loop {
                let mut message = MSG::default();

                while unsafe { PeekMessageW(&mut message, null_mut(), 0, 0, PM_REMOVE) } > 0 {
                    unsafe {
                        if message.message == WM_QUIT {
                            GLOBAL_RUNNING = false;
                        }

                        TranslateMessage(&message);
                        DispatchMessageW(&message);
                    }
                }

                unsafe { render_weird_gradient(&GLOBAL_BACKBUFFER, x_offset, y_offset) };

                let dimension = win32_get_window_dimension(window);
                unsafe {
                    win32_display_buffer_in_window(
                        device_context,
                        dimension.width,
                        dimension.height,
                        &GLOBAL_BACKBUFFER,
                    )
                };

                x_offset += 1;
                y_offset += 2;

                if unsafe { GLOBAL_RUNNING } == false {
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
