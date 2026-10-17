use gtk::{gdk, prelude::*};
use tauri::Webview;

pub fn listen(drag: &Webview) -> tauri::Result<()> {
    drag.with_webview(|native| {
        let tile = native.inner();
        let window = tile.toplevel().unwrap().downcast::<gtk::Window>().unwrap();

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
    })
}
