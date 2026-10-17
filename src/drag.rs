#[cfg(target_os = "linux")]
use gtk::prelude::*;
use tauri::{
    webview::WebviewBuilder, LogicalPosition, LogicalSize, Webview, WebviewUrl, WebviewWindow,
};
#[cfg(not(target_os = "linux"))]
use tauri::{Rect, WindowEvent};

const WIDTH: f64 = 160.0;
const HEIGHT: f64 = 48.0;

pub fn create(content: &WebviewWindow) -> tauri::Result<Webview> {
    let view = content.as_ref().window().add_child(
        WebviewBuilder::new("drag", WebviewUrl::App("index.html".into())).focused(false),
        LogicalPosition::new(0.0, 0.0),
        LogicalSize::new(WIDTH, HEIGHT),
    )?;
    view.hide()?;

    #[cfg(target_os = "linux")]
    view.with_webview(|native| {
        let tile = native.inner();
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
    })?;

    #[cfg(not(target_os = "linux"))]
    {
        center(&view)?;
        let view = view.clone();
        content.on_window_event(move |event| {
            matches!(
                event,
                WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. }
            )
            .then(|| {
                center(&view).unwrap_or_else(|error| {
                    eprintln!("Cannot center drag area: {error}");
                });
            });
        });
    }

    Ok(view)
}

#[cfg(not(target_os = "linux"))]
fn center(view: &Webview) -> tauri::Result<()> {
    let window = view.window();
    let size = window
        .inner_size()?
        .to_logical::<f64>(window.scale_factor()?);
    view.set_bounds(Rect {
        position: LogicalPosition::new(
            ((size.width - WIDTH) / 2.0).max(0.0),
            ((size.height - HEIGHT) / 2.0).max(0.0),
        )
        .into(),
        size: LogicalSize::new(WIDTH, HEIGHT).into(),
    })
}
