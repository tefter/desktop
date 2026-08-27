// tefter.io in a desktop window. The predecessor was a 2019 nativefier
// build whose bundled Chromium 66 aged out from under it; Tauri renders in
// the system webview, which ages with the OS instead of with this app.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{Url, WebviewUrl, WebviewWindowBuilder};

const TEFTER: &str = "https://tefter.io";

// The same user-agent idea as the old build's "tefter-electron": unknown to
// the server's allow_browser check (which only blocks browsers it recognizes
// as outdated), and recognizable in the production logs.
const USER_AGENT: &str = concat!("tefter-desktop/", env!("CARGO_PKG_VERSION"));

// tefter.io and its subdomains (guides.) stay in the window; anything else
// belongs to the person's own browser.
fn internal(url: &Url) -> bool {
    matches!(url.host_str(), Some(host) if host == "tefter.io" || host.ends_with(".tefter.io"))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(TEFTER.parse().unwrap()))
                .title("Tefter")
                .inner_size(1280.0, 840.0)
                .user_agent(USER_AGENT)
                .on_navigation(|url| {
                    if internal(url) {
                        return true;
                    }
                    let _ = open::that(url.as_str());
                    false
                })
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tefter");
}
