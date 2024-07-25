use std::ffi::c_void;

use windows::Win32::{
    Foundation::{CloseHandle, COLORREF, HWND, RECT},
    Graphics::Dwm::{
        DwmSetWindowAttribute, DWMWA_WINDOW_CORNER_PREFERENCE, DWMWCP_DONOTROUND, DWMWCP_ROUND,
        DWM_WINDOW_CORNER_PREFERENCE,
    },
    System::{
        ProcessStatus::GetProcessImageFileNameW,
        Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
    UI::WindowsAndMessaging::{
        GetClassNameW, GetWindowLongPtrW, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
        GetWindowThreadProcessId, SetLayeredWindowAttributes, SetWindowLongPtrW, SetWindowPos,
        GWL_EXSTYLE, GWL_STYLE, HWND_TOP, LWA_ALPHA, SWP_FRAMECHANGED, SWP_NOMOVE, WS_CAPTION,
        WS_EX_LAYERED,
    },
};

pub fn set_window_alpha(raw_handle: isize, alpha: u8) {
    let handle = HWND(raw_handle as *mut c_void);

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

pub fn set_window_titlebar(raw_handle: isize, titlebar: bool) {
    let handle = HWND(raw_handle as *mut c_void);

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

pub fn set_window_rounded(raw_handle: isize, rounded: bool) {
    let handle = HWND(raw_handle as *mut c_void);

    unsafe {
        let preference: DWM_WINDOW_CORNER_PREFERENCE = if rounded {
            DWMWCP_ROUND
        } else {
            DWMWCP_DONOTROUND
        };

        let _ = DwmSetWindowAttribute(
            handle,
            DWMWA_WINDOW_CORNER_PREFERENCE,
            &preference as *const _ as *const _,
            std::mem::size_of::<DWM_WINDOW_CORNER_PREFERENCE>() as u32,
        );
    }
}

pub fn get_window_title(raw_handle: isize) -> Option<String> {
    let handle = HWND(raw_handle as *mut c_void);
    unsafe {
        let length = GetWindowTextLengthW(handle);
        if length == 0 {
            return None;
        }

        let mut buffer: Vec<u16> = vec![0; (length + 1) as usize];

        let result = GetWindowTextW(handle, &mut buffer);

        if result > 0 {
            Some(String::from_utf16_lossy(&buffer[..result as usize]))
        } else {
            None
        }
    }
}

pub fn get_window_class(raw_handle: isize) -> Option<String> {
    let handle = HWND(raw_handle as *mut c_void);
    unsafe {
        let mut buffer: Vec<u16> = vec![0; 256]; // Arbitrary buffer size

        let length = GetClassNameW(handle, &mut buffer);

        if length > 0 {
            Some(String::from_utf16_lossy(&buffer[..length as usize]))
        } else {
            None
        }
    }
}

pub fn get_window_process_name(raw_handle: isize) -> Option<String> {
    let handle = HWND(raw_handle as *mut c_void);
    unsafe {
        let mut process_id: u32 = 0;
        GetWindowThreadProcessId(handle, Some(&mut process_id));

        if process_id == 0 {
            return None;
        }

        let process_handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_READ,
            false,
            process_id,
        );

        if process_handle.is_err() {
            return None;
        }

        let mut buffer = vec![0u16; 260];

        let length = GetProcessImageFileNameW(process_handle.clone().unwrap(), &mut buffer);

        if length == 0 {
            return None;
        }

        let _ = CloseHandle(process_handle.unwrap());

        return Some(String::from_utf16_lossy(&buffer[..length as usize]));
    }
}

// pub fn get_visible_windows() -> Vec<isize> {
//     let mut hwnds: Vec<isize> = Vec::new();

//     unsafe {
//         let _ = EnumWindows(
//             Some(get_visible_windows_proc),
//             LPARAM(&mut hwnds as *mut _ as _),
//         );
//     };

//     hwnds
//         .iter()
//         .copied()
//         .filter(|hwnd| unsafe { IsWindowVisible(HWND(*hwnd as *mut c_void)) }.as_bool())
//         .collect()
// }

// extern "system" fn get_visible_windows_proc(handle: HWND, data: LPARAM) -> BOOL {
//     let hwnds = unsafe { (data.0 as *mut Vec<isize>).as_mut() };
//     if let Some(hwnds) = hwnds {
//         hwnds.push(handle.0 as isize);
//     }
//     true.into()
// }

pub struct WindowInfo {
    pub title: String,
    pub class: String,
    pub process_name: String,
}

pub fn get_window_info(raw_handle: isize) -> WindowInfo {
    let title = get_window_title(raw_handle).unwrap_or(String::from(".*"));

    let class = get_window_class(raw_handle).unwrap_or(String::from(""));

    let process_name = get_window_process_name(raw_handle).unwrap_or(String::from(""));

    WindowInfo {
        title,
        class,
        process_name,
    }
}
