use block2::RcBlock;
use objc2::{rc::Retained, runtime::AnyObject};
use objc2_app_kit::{NSEvent, NSEventMask, NSEventModifierFlags as Flags, NSView, NSWindow};
use std::ptr::NonNull;
use tauri::{App, Webview, WebviewWindow};

struct Monitor(Retained<AnyObject>);

impl Drop for Monitor {
    fn drop(&mut self) {
        unsafe { NSEvent::removeMonitor(&self.0) };
    }
}

pub fn run(app: App, content: WebviewWindow, drag: Option<Webview>) -> tauri::Result<i32> {
    let _monitor = drag.map(|drag| listen(&content, drag)).transpose()?;
    if app.config().app.windows[0].visible {
        content.show()?;
    }
    Ok(app.run_return(|_, _| {}))
}

fn listen(content: &WebviewWindow, drag: Webview) -> tauri::Result<Monitor> {
    let number = unsafe { &*content.ns_window()?.cast::<NSWindow>() }.windowNumber();
    let handler = RcBlock::new(move |event: NonNull<NSEvent>| {
        let key = unsafe { event.as_ref() };
        let modifiers =
            key.modifierFlags() & (Flags::Option | Flags::Control | Flags::Shift | Flags::Command);
        if key.windowNumber() != number
            || modifiers != Flags::Option
            || !key
                .charactersIgnoringModifiers()
                .is_some_and(|text| text.to_string().eq_ignore_ascii_case("x"))
        {
            return event.as_ptr();
        }

        if !key.isARepeat() {
            if let Err(error) = drag.with_webview(|native| {
                let view = unsafe { &*native.inner().cast::<NSView>() };
                view.setHidden(!view.isHidden());
            }) {
                eprintln!("Cannot toggle drag area: {error}");
            }
        }
        std::ptr::null_mut()
    });
    let monitor = unsafe {
        NSEvent::addLocalMonitorForEventsMatchingMask_handler(NSEventMask::KeyDown, &handler)
    }
    .ok_or_else(|| tauri::Error::Io(std::io::Error::other("Cannot install Option+X shortcut")))?;
    Ok(Monitor(monitor))
}
