// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod wallpaperflare;
use wallpaperflare::scrape;


#[tauri::command]
async fn scrape_wallpapers(query: &str) -> Result<String, String> {
    match scrape(query).await {
        Ok(wallpapers) => Ok(serde_json::to_string(&wallpapers).unwrap()),
        Err(e) => Err(e),
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scrape_wallpapers])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
