use crate::drag::{HEIGHT, WIDTH};
use gtk::{gdk, prelude::*};
use tauri::{App, Webview, WebviewWindow};

pub fn run(app: App, content: WebviewWindow, drag: Option<Webview>) -> tauri::Result<i32> {
    if let Some(drag) = drag {
        drag.with_webview(|native| {
            let tile = native.inner();
            let window = tile.toplevel().unwrap().downcast::<gtk::Window>().unwrap();
            let container = tile.parent().unwrap().downcast::<gtk::Box>().unwrap();
            let page = container
                .children()
                .into_iter()
                .find(|child| child != tile.upcast_ref::<gtk::Widget>())
                .expect("content WebView");

            let overlay = gtk::Overlay::new();
            container.remove(&page);
            container.remove(&tile);
            overlay.add(&page);
            tile.set_no_show_all(true);
            tile.set_halign(gtk::Align::Center);
            tile.set_valign(gtk::Align::Center);
            tile.set_size_request(WIDTH as i32, HEIGHT as i32);
            overlay.add_overlay(&tile);
            container.pack_start(&overlay, true, true, 0);
            overlay.show();

            let shortcuts = gtk::AccelGroup::new();
            shortcuts.connect_accel_group(
                *gdk::keys::constants::x,
                gdk::ModifierType::MOD1_MASK,
                gtk::AccelFlags::empty(),
                move |_, _, _, _| {
                    tile.set_visible(!tile.is_visible());
                    true
                },
            );
            window.add_accel_group(&shortcuts);
        })?;
    }

    if app.config().app.windows[0].visible {
        content.show()?;
    }
    Ok(app.run_return(|_, _| {}))
}
