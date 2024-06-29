use windows::Win32::{
    Foundation::{COLORREF, HWND},
    UI::WindowsAndMessaging::{
        GetWindowLongPtrW, SetLayeredWindowAttributes, SetWindowLongPtrW, GWL_EXSTYLE, LWA_ALPHA,
        WS_EX_LAYERED,
    },
};

pub fn set_window_alpha(raw_handle: isize, alpha: u8) {
    let handle = HWND(raw_handle);

    unsafe {
        let ex_style = GetWindowLongPtrW(handle, GWL_EXSTYLE);
        if ex_style & isize::try_from(WS_EX_LAYERED.0).unwrap() == 0 {
            println!("Setting WS_EX_LAYERED");
            SetWindowLongPtrW(
                handle,
                GWL_EXSTYLE,
                ex_style | isize::try_from(WS_EX_LAYERED.0).unwrap(),
            );
        }
        let _ = SetLayeredWindowAttributes(handle, COLORREF::default(), alpha, LWA_ALPHA);
    }
}
