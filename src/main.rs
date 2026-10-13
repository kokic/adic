use tauri::WebviewUrl;

fn usage() -> ! {
    eprintln!("Usage: adic <url> [--decorations true|false]");
    std::process::exit(2);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let url = args.next().unwrap_or_else(|| usage());
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
    let url = WebviewUrl::External(url.parse()?);

    tauri::Builder::default()
        .setup(move |app| {
            let mut config = app.config().app.windows[0].clone();
            if let Some(decorations) = decorations {
                config.decorations = decorations;
            }
            config.url = url;
            tauri::WebviewWindowBuilder::from_config(app, &config)?.build()?;
            Ok(())
        })
        .run(tauri::generate_context!())?;

    Ok(())
}
