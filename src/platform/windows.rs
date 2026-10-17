use tauri::{App, Webview, WebviewWindow};
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

pub fn run(app: App, content: WebviewWindow, drag: Option<Webview>) -> tauri::Result<i32> {
    if let Some(drag) = drag {
        for source in [content.as_ref(), &drag] {
            listen(source, &drag)?;
        }
    }

    if app.config().app.windows[0].visible {
        content.show()?;
    }
    Ok(app.run_return(|_, _| {}))
}

fn listen(source: &Webview, drag: &Webview) -> tauri::Result<()> {
    let target = drag.clone();
    source.with_webview(move |native| unsafe {
        let handler = AcceleratorKeyPressedEventHandler::create(Box::new(move |_, args| {
            let Some(args) = args else {
                return Ok(());
            };
            let mut key = 0;
            args.VirtualKey(&mut key)?;
            let down = |key| GetKeyState(i32::from(key)) < 0;
            if key != u32::from(VK_X.0)
                || !down(VK_MENU.0)
                || [VK_CONTROL, VK_SHIFT, VK_LWIN, VK_RWIN]
                    .iter()
                    .any(|key| down(key.0))
            {
                return Ok(());
            }

            args.SetHandled(true)?;
            let mut kind = Default::default();
            let mut status = PhysicalKeyStatus::default();
            args.KeyEventKind(&mut kind)?;
            args.PhysicalKeyStatus(&mut status)?;
            if matches!(kind, KEY_DOWN | SYSTEM_KEY_DOWN) && !status.WasKeyDown.as_bool() {
                toggle(&target);
            }
            Ok(())
        }));
        if let Err(error) = native
            .controller()
            .add_AcceleratorKeyPressed(&handler, &mut 0)
        {
            eprintln!("Cannot install Alt+X shortcut: {error}");
        }
    })
}

fn toggle(drag: &Webview) {
    let target = drag.clone();
    if let Err(error) = drag.with_webview(move |native| {
        let mut visible = Default::default();
        if let Err(error) = unsafe { native.controller().IsVisible(&mut visible) } {
            eprintln!("Cannot read drag area visibility: {error}");
            return;
        }

        let result = if visible.as_bool() {
            target.hide()
        } else {
            target.show()
        };
        if let Err(error) = result {
            eprintln!("Cannot toggle drag area: {error}");
        }
    }) {
        eprintln!("Cannot access drag area: {error}");
    }
}
