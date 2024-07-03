use windows::Win32::{
    Foundation::{COLORREF, HWND, RECT},
    UI::WindowsAndMessaging::{
        GetWindowLongPtrW, GetWindowRect, SetLayeredWindowAttributes, SetWindowLongPtrW,
        SetWindowPos, GWL_EXSTYLE, GWL_STYLE, HWND_TOP, LWA_ALPHA, SWP_FRAMECHANGED, SWP_NOMOVE,
        WS_CAPTION, WS_EX_LAYERED,
    },
};

#[tauri::command]
pub fn set_window_alpha(raw_handle: isize, alpha: u8) {
    let handle = HWND(raw_handle);

    unsafe {
        let ex_style = GetWindowLongPtrW(handle, GWL_EXSTYLE);
        if ex_style & isize::try_from(WS_EX_LAYERED.0).unwrap() == 0 {
            SetWindowLongPtrW(
                handle,
                GWL_EXSTYLE,
                ex_style | isize::try_from(WS_EX_LAYERED.0).unwrap(),
            );
        }
        let _ = SetLayeredWindowAttributes(handle, COLORREF::default(), alpha, LWA_ALPHA);
    }
}

#[tauri::command]
pub fn set_window_titlebar(raw_handle: isize, titlebar: bool) {
    let handle = HWND(raw_handle);

    unsafe {
        let ex_style = GetWindowLongPtrW(handle, GWL_STYLE);
        let style = if titlebar {
            ex_style | (isize::try_from(WS_CAPTION.0).unwrap())
        } else {
            ex_style & !(isize::try_from(WS_CAPTION.0).unwrap())
        };
        SetWindowLongPtrW(handle, GWL_STYLE, style);

        let mut rect = RECT::default();
        let _ = GetWindowRect(handle, &mut rect);
        let _ = SetWindowPos(
            handle,
            HWND_TOP,
            0,
            0,
            rect.right - rect.left,
            rect.bottom - rect.top,
            SWP_FRAMECHANGED | SWP_NOMOVE,
        );
    }
}
