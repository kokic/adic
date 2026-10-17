use tauri::Webview;
use webview2_com::{
    AcceleratorKeyPressedEventHandler,
    Microsoft::Web::WebView2::Win32::{
        COREWEBVIEW2_KEY_EVENT_KIND_KEY_DOWN as KEY_DOWN,
        COREWEBVIEW2_KEY_EVENT_KIND_SYSTEM_KEY_DOWN as SYSTEM_KEY_DOWN,
        COREWEBVIEW2_PHYSICAL_KEY_STATUS as PhysicalKeyStatus,
    },
};
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetKeyState, VK_CONTROL, VK_LWIN, VK_MENU, VK_RWIN, VK_SHIFT, VK_X,
};

pub fn listen(content: &Webview, drag: &Webview) -> tauri::Result<()> {
    [content, drag].into_iter().try_for_each(|source| {
        let target = drag.clone();
        source.with_webview(move |native| unsafe {
            let handler = AcceleratorKeyPressedEventHandler::create(Box::new(move |_, args| {
                args.map_or(Ok(()), |args| {
                    let mut key = 0;
                    args.VirtualKey(&mut key)?;
                    let down = |key| GetKeyState(i32::from(key)) < 0;
                    (key == u32::from(VK_X.0)
                        && down(VK_MENU.0)
                        && ![VK_CONTROL, VK_SHIFT, VK_LWIN, VK_RWIN]
                            .iter()
                            .any(|key| down(key.0)))
                    .then(|| {
                        args.SetHandled(true)?;
                        let mut kind = Default::default();
                        let mut status = PhysicalKeyStatus::default();
                        args.KeyEventKind(&mut kind)?;
                        args.PhysicalKeyStatus(&mut status)?;
                        (matches!(kind, KEY_DOWN | SYSTEM_KEY_DOWN)
                            && !status.WasKeyDown.as_bool())
                        .then(|| toggle(&target));
                        Ok(())
                    })
                    .unwrap_or(Ok(()))
                })
            }));
            native
                .controller()
                .add_AcceleratorKeyPressed(&handler, &mut 0)
                .unwrap_or_else(|error| eprintln!("Cannot install Alt+X shortcut: {error}"));
        })
    })
}

fn toggle(drag: &Webview) {
    let target = drag.clone();
    drag.with_webview(move |native| {
        let mut visible = Default::default();
        unsafe { native.controller().IsVisible(&mut visible) }.map_or_else(
            |error| eprintln!("Cannot read drag area visibility: {error}"),
            |()| {
                visible
                    .as_bool()
                    .then(|| target.hide())
                    .unwrap_or_else(|| target.show())
                    .unwrap_or_else(|error| eprintln!("Cannot toggle drag area: {error}"));
            },
        );
    })
    .unwrap_or_else(|error| eprintln!("Cannot access drag area: {error}"));
}
