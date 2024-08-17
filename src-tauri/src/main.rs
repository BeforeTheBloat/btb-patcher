use reqwest::Client;
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::env;
use std::process::Command;
use tauri::Manager;
use tokio;

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
static mut DISCORD_IPC_CLIENT: Option<DiscordIpcClient> = None;

#[tauri::command]
async fn download_minecraft(window: tauri::Window) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let url = "https://archive.org/download/CU-MBAP/Minecraft%20Archives/Minecraft-1.1.zip/Minecraft-1.1.5.0.Appx";
        let appdata = env::var("APPDATA").map_err(|e| e.to_string())?;
        let output_dir = format!("{}/.btb", appdata);
        let output_path = format!("{}/Minecraft-1.1.5.0.Appx", output_dir);

        if std::path::Path::new(&output_path).exists() {
            return Ok(());
        }

        create_dir_all(&output_dir).map_err(|e| e.to_string())?;

        let client = Client::new();
        let mut response = client.get(url).send().await.map_err(|e| e.to_string())?;

        if !response.status().is_success() {
            return Err(format!("Failed to download file. Status: {}", response.status()));
        }

        let total_size = response.content_length().unwrap_or(0);
        let mut file = File::create(&output_path).map_err(|e| e.to_string())?;
        let mut downloaded = 0u64;

        while let Some(chunk) = response.chunk().await.map_err(|e| e.to_string())? {
            file.write_all(&chunk).map_err(|e| e.to_string())?;
            downloaded += chunk.len() as u64;

            window.emit("progress_update", (downloaded as f64 / total_size as f64 * 100.0).round())
                .map_err(|e| e.to_string())?;
        }

        window.emit("download_complete", ()).map_err(|e| e.to_string())?;

        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("This action is only supported on Windows.".to_string())
    }
}

#[tauri::command]
async fn check_minecraft_version() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    {
        let output = Command::new("powershell")
            .arg("-Command")
            .arg("Get-AppxPackage -Name Microsoft.MinecraftUWP | Select-Object -ExpandProperty Version | ForEach-Object { $_.ToString().Trim() }")
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err("Failed to get Minecraft version".to_string());
        }

        let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(version)
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("This action is only supported on Windows.".to_string())
    }
}

#[tauri::command]
async fn open_minecraft(window: tauri::Window) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let appdata = env::var("APPDATA").map_err(|e| e.to_string())?;
        let output_dir = format!("{}/.btb", appdata);
        let appx_path = format!("{}/Minecraft-1.1.5.0.Appx", output_dir);

        download_minecraft(window.clone()).await?;

        let install_output = Command::new("powershell")
            .arg("-Command")
            .arg(format!(
                "Add-AppxPackage -Path \"{}\"",
                appx_path
            ))
            .output()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if !install_output.status.success() {
            let stderr = String::from_utf8_lossy(&install_output.stderr);
            return Err(format!("Failed to install .appx file. Error: {}", stderr));
        }

        let launch_output = Command::new("powershell")
            .arg("-Command")
            .arg("start minecraft://")
            .output()
            .map_err(|e| format!("Failed to start Minecraft: {}", e))?;

        if !launch_output.status.success() {
            let stderr = String::from_utf8_lossy(&launch_output.stderr);
            return Err(format!("Failed to start Minecraft. Error: {}", stderr));
        }

        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        Err("This action is only supported on Windows.".to_string())
    }
}

fn setup_discord_ipc() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        if DISCORD_IPC_CLIENT.is_none() {
            let mut client = DiscordIpcClient::new("1271991647772872706")?;
            client.connect()?;
            client.set_activity(activity::Activity::new()
                .state("Playing Minecraft v1.1.5")
            )?;
            DISCORD_IPC_CLIENT = Some(client);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = setup_discord_ipc() {
        eprintln!("Failed to setup Discord IPC: {:?}", e);
    }

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
                }

                if let Err(e) = apply_blur(&window) {
                    log::error!("Failed to apply blur vibrancy: {:?}", e);
                }

                if let Err(e) = apply_rounded_corners(&window) {
                    log::error!("Failed to apply rounded corners: {:?}", e);
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![download_minecraft, check_minecraft_version, open_minecraft])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
