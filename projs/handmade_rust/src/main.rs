use std::ptr::{null, null_mut};

use win32::*;

static mut COLOR: DWORD = WHITENESS;

pub unsafe extern "system" fn main_window_callback(
    window: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let mut result = 0;

    match message {
        WM_SIZE => {
            println!("WM_SIZE");
            OutputDebugStringW(wide_null("WM_SIZE\n").as_ptr());
        }
        WM_CLOSE => {
            println!("WM_CLOSE");
            OutputDebugStringW(wide_null("WM_CLOSE\n").as_ptr());
        }
        WM_ACTIVATEAPP => {
            println!("WM_ACTIVATEAPP");
            OutputDebugStringW(wide_null("WM_ACTIVATEAPP\n").as_ptr());
        }
        WM_DESTROY => {
            println!("WM_DESTROY");
            OutputDebugStringW(wide_null("WM_DESTROY\n").as_ptr());
        }
        WM_PAINT => {
            println!("WM_PAINT");
            let mut paint = PAINTSTRUCT::default();
            let device_context = BeginPaint(window, &mut paint);
            let x = paint.rcPaint.left;
            let y = paint.rcPaint.top;
            let width = paint.rcPaint.right - paint.rcPaint.left;
            let height = paint.rcPaint.bottom - paint.rcPaint.top;
            let operation = COLOR;
            PatBlt(device_context, x, y, width, height, operation);
            if operation == WHITENESS {
                COLOR = BLACKNESS;
            } else {
                COLOR = WHITENESS;
            }
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
        lpfnWndProc: Some(main_window_callback),
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
            loop {
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
            }
        } else {
            // TODO: Logging
        }
    } else {
        // TODO: Logging
    }
}
