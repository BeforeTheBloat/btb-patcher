#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::Manager;
use std::fs::File;
use std::io::copy;
use reqwest::get;

const APK_URL: &str = "https://download2285.mediafire.com/z2s1exrhqmwgmMPGuoiGywqWm1skfRXIXeQbMmjpo8Lu0kB19O6hs4TWLupqpVVcYBHLWegY3LV8e2TfDQiMuDjbFPvxBbLyTiMtlDrXocPyWH3b0530Q2u6rznNJThU-pkQPDWtX04oI8kg-nfpjw26l9u8xHDICYgagvDP-5dD/sx17ekm2z40v6ls/MCPE+.1-1-3.apk"; // URL for APK

#[tauri::command]
async fn prepare_emulator_command() -> Result<String, String> {
    // Ensure the AVD exists
    let avd_name = "myEmulator";

    let avd_list_output = Command::new("emulator")
        .arg("-list-avds")
        .output()
        .map_err(|e| e.to_string())?;

    let avds = String::from_utf8_lossy(&avd_list_output.stdout);
    if !avds.contains(avd_name) {
        // Create the AVD if it doesn't exist
        let output = Command::new("avdmanager")
            .arg("create")
            .arg("avd")
            .arg("-n")
            .arg(avd_name)
            .arg("-k")
            .arg("system-images;android-30;google_apis;x86_64")
            .output()
            .map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }
    }

    // Start the AVD
    let output = Command::new("emulator")
        .arg("-avd")
        .arg(avd_name)
        .output()
        .map_err(|e| e.to_string())?;


println!("Command output: {:?}", String::from_utf8_lossy(&output.stdout));
println!("Command error: {:?}", String::from_utf8_lossy(&output.stderr));


    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[tauri::command]
async fn launch_apk_command() -> Result<String, String> {
    let apk_local_path = "MCPE.apk"; // Temporary local file path for downloaded APK

    // Download the APK
    let response = get(APK_URL)
        .await
        .map_err(|e| e.to_string())?;
    let mut file = File::create(apk_local_path)
        .map_err(|e| e.to_string())?;
    copy(&mut response.bytes().await.map_err(|e| e.to_string())?.as_ref(), &mut file)
        .map_err(|e| e.to_string())?;

    // Install the APK using adb
    let output = Command::new("adb")
        .arg("install")
        .arg(apk_local_path)
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    } else {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
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
    .invoke_handler(tauri::generate_handler![prepare_emulator_command, launch_apk_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
