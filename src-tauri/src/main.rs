// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use discord_rpc_client::Client as DiscordRPC;
use std::io;
use tauri::Manager;


#[tauri::command]
fn start_discord_rpc() {
    let mut drpc = DiscordRPC::new(1271991647772872706);

    drpc.on_ready(|_ctx| {
        println!("READY!");
    });

    drpc.on_error(|_ctx| {
        eprintln!("An error occured");
    });

    drpc.start();

    loop {
        let mut buf = String::new();

        io::stdin().read_line(&mut buf).unwrap();
        buf.pop();

        if buf.is_empty() {
            if let Err(why) = drpc.clear_activity() {
                println!("Failed to clear presence: {}", why);
            }
        } else {
            if let Err(why) = drpc.set_activity(|a| a
                .state(buf)
                .assets(|ass| ass
                    .large_image("ferris_wat")
                    .large_text("wat.")
                    .small_image("rusting")
                    .small_text("rusting...")))
            {
                println!("Failed to set presence: {}", why);
            }
        }
    };

}

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let window = app.get_window("main").unwrap();

        #[cfg(target_os = "macos")]
        {
            use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};
            if let Err(e) = apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None) {
                log::error!("Failed to apply vibrancy: {:?}", e);
            }
        }

        #[cfg(target_os = "windows")]
        {
            use window_vibrancy::{apply_acrylic, apply_blur, apply_rounded_corners};

            if let Err(e) = apply_acrylic(&window, None) {
                log::error!("Failed to apply acrylic vibrancy: {:?}", e);

                if let Err(e) = apply_blur(&window) {
                    log::error!("Failed to apply blur vibrancy: {:?}", e);
                }
            }

            if let Err(e) = apply_rounded_corners(&window) {
                log::error!("Failed to apply rounded corners: {:?}", e);
            }
        }

        Ok(())
    })
    .invoke_handler(tauri::generate_handler![start_discord_rpc])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
