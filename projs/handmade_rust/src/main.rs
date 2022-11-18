use std::ffi::c_void;
use win32::*;

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
