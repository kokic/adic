use tauri::{
    webview::WebviewBuilder, LogicalPosition, LogicalSize, Webview, WebviewUrl, WebviewWindow,
};
#[cfg(not(target_os = "linux"))]
use tauri::{Rect, WindowEvent};

pub const WIDTH: f64 = 160.0;
pub const HEIGHT: f64 = 48.0;

pub fn create(content: &WebviewWindow) -> tauri::Result<Webview> {
    let view = content.as_ref().window().add_child(
        WebviewBuilder::new("drag", WebviewUrl::App("index.html".into())).focused(false),
        LogicalPosition::new(0.0, 0.0),
        LogicalSize::new(WIDTH, HEIGHT),
    )?;
    view.hide()?;

    #[cfg(not(target_os = "linux"))]
    {
        center(&view)?;
        let view = view.clone();
        content.on_window_event(move |event| {
            if matches!(
                event,
                WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. }
            ) {
                if let Err(error) = center(&view) {
                    eprintln!("Cannot center drag area: {error}");
                }
            }
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
