use windows::Win32::{
    Foundation::{COLORREF, HWND},
    UI::WindowsAndMessaging::{
        GetWindowLongA, SetLayeredWindowAttributes, SetWindowLongA, GWL_EXSTYLE, LWA_ALPHA,
        WS_EX_LAYERED,
    },
};

pub fn set_window_alpha(raw_handle: isize, alpha: u8) {
    let handle = HWND(raw_handle);

    unsafe {
        let ex_style = GetWindowLongA(handle, GWL_EXSTYLE);
        if ex_style & i32::try_from(WS_EX_LAYERED.0).unwrap() == 0 {
            println!("Setting WS_EX_LAYERED");
            SetWindowLongA(
                handle,
                GWL_EXSTYLE,
                ex_style | i32::try_from(WS_EX_LAYERED.0).unwrap(),
            );
        }
        let _ = SetLayeredWindowAttributes(handle, COLORREF::default(), alpha, LWA_ALPHA);
    }
}
