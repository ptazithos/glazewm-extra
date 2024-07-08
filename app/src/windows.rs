use image::{ImageBuffer, Rgba};
use std::ffi::c_void;

use windows::Win32::{
    Foundation::{COLORREF, HWND, RECT},
    Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC,
        GetDIBits, ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, CAPTUREBLT,
        DIB_RGB_COLORS, RGBQUAD, SRCCOPY,
    },
    UI::WindowsAndMessaging::{
        GetClientRect, GetWindowLongPtrW, GetWindowRect, SetLayeredWindowAttributes,
        SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, GWL_STYLE, HWND_DESKTOP, HWND_TOP, LWA_ALPHA,
        SWP_FRAMECHANGED, SWP_NOMOVE, WS_CAPTION, WS_EX_LAYERED,
    },
};

#[tauri::command]
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

#[tauri::command]
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

#[tauri::command]
pub fn capture_window(raw_handle: isize, path: String) {
    let handle = HWND(raw_handle as *mut c_void);

    unsafe {
        let hdc = GetDC(HWND_DESKTOP);
        let hdc_mem = CreateCompatibleDC(hdc);

        let mut rect = RECT::default();
        GetClientRect(handle, &mut rect).unwrap();

        let width = rect.right - rect.left;
        let height = rect.bottom - rect.top;

        let virtual_screen = CreateCompatibleBitmap(hdc, width, height);
        SelectObject(hdc_mem, virtual_screen);

        BitBlt(
            hdc_mem,
            0,
            0,
            width,
            height,
            hdc,
            rect.left,
            rect.top,
            CAPTUREBLT | SRCCOPY,
        )
        .unwrap();

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width,
                biHeight: -height, // top-down DIB
                biPlanes: 1,
                biBitCount: 32,
                biCompression: 0,
                biSizeImage: (width * height * 4) as u32,
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

        let mut buffer: Vec<u8> = vec![0; (width * height * 4) as usize];
        GetDIBits(
            hdc,
            virtual_screen,
            0,
            height as u32,
            Option::Some(buffer.as_mut_ptr() as *mut c_void),
            &mut bmi,
            DIB_RGB_COLORS,
        );

        let img =
            ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, buffer).unwrap();
        img.save(path).unwrap();

        let _ = DeleteObject(virtual_screen);
        let _ = DeleteDC(hdc_mem);
        ReleaseDC(handle, hdc);
    }
}
