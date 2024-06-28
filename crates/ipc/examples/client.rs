use anyhow::{Ok, Result};
use ipc::subscribe::subscribe;
use windows::Win32::{
    Foundation::{COLORREF, HWND},
    UI::WindowsAndMessaging::{
        GetWindowLongA, SetLayeredWindowAttributes, SetWindowLongA, GWL_EXSTYLE, LWA_ALPHA,
        WS_EX_LAYERED,
    },
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    subscribe("focus_changed", |payload| {
        let mut handle = HWND::default();
        handle.0 = isize::try_from(payload.data.focused_container.handle).unwrap();

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
            let _ = SetLayeredWindowAttributes(handle, COLORREF::default(), 125, LWA_ALPHA);
        }

        println!("Received message: {:?}", payload);
        false
    })
    .await?;
    Ok(())
}
