use tauri::{WebviewUrl, WebviewWindowBuilder};

mod drag;

#[cfg_attr(windows, path = "platform/windows.rs")]
#[cfg_attr(target_os = "macos", path = "platform/macos.rs")]
#[cfg_attr(target_os = "linux", path = "platform/linux.rs")]
mod platform;

fn usage() -> ! {
    eprintln!("Usage: adic <url> [--decorations true|false]");
    std::process::exit(2);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let url = WebviewUrl::External(args.next().unwrap_or_else(|| usage()).parse()?);
    let mut decorations = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--decorations" => {
                decorations = Some(
                    args.next()
                        .and_then(|value| value.parse::<bool>().ok())
                        .unwrap_or_else(|| usage()),
                );
            }
            _ => usage(),
        }
    }

    let app = tauri::Builder::default().build(tauri::generate_context!())?;
    let mut config = app.config().app.windows[0].clone();
    config.url = url;
    if let Some(value) = decorations {
        config.decorations = value;
    }

    let content = WebviewWindowBuilder::from_config(&app, &config)?
        .visible(false)
        .build()?;
    let drag = if config.decorations {
        None
    } else {
        Some(drag::create(&content)?)
    };

    let exit_code = platform::run(app, content, drag)?;
    std::process::exit(exit_code);
}
