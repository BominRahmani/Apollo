use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

extern crate cloudflare_bypasser;

use tauri::utils::html::NodeRef;
// Add this line to import Path


#[derive(Debug, Serialize, Deserialize)]
pub struct Wallpaper {
    url: String,
    preview: String,
    resolution: String,
    local_preview: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageFile {
    url_to_imgs: String,
    preview: String,
    num_imgs: String,
    title: String,
}

const BASE_WALLPAPER_SOURCE: &str = "https://wallpapercave.com/search?q=";

fn format_query(query: &str) -> String {
    query.replace(" ", "+")
}

pub async fn scrape(query: &str) -> Result<Vec<ImageFile>, String> {
  let formatted_query = format_query(query);
  let url = format!("{}{}", BASE_WALLPAPER_SOURCE, formatted_query);

    let output = Command::new("curl")
        .arg("-A")
        .arg("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/58.0.3029.110 Safari/537.3")
        .arg(&url)
        .output()
        .map_err(|e| format!("Failed to execute curl: {}", e))?;

    if !output.status.success() {
        return Err(format!("curl command failed: {}", String::from_utf8_lossy(&output.stderr)));
    }

    let html = String::from_utf8(output.stdout)
        .map_err(|e| format!("Failed to parse curl output as UTF-8: {}", e))?;

    let files = extract_wallpapers_from_folder(&html);

    Ok(files?)
}

//pub async fn scrape(query: &str) -> Result<Vec<Wallpaper>, String> {
//    //let formatted_query = format_query(query);
//    //let url = format!("{}{}", BASE_WALLPAPER_SOURCE, formatted_query);
//    //
//    //let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
//    //let body = response.text().await.map_err(|e| e.to_string())?;
//    //
//    //let wallpapers = extract_wallpapers(&body)?;
//    //
//
//
//    let wallpaper_futures: Vec<_> = wallpapers.into_iter().enumerate().map(|(index, wallpaper)| {
//        async move {
//            let local_preview = download_preview(&wallpaper.preview, index).await?;
//            Ok::<Wallpaper, String>(Wallpaper {
//                url: wallpaper.url,
//                preview: wallpaper.preview,
//                resolution: wallpaper.resolution,
//                local_preview,
//            })
//        }
//    }).collect();
//
//    let catalogue: Vec<Wallpaper> = join_all(wallpaper_futures)
//        .await
//        .into_iter()
//        .filter_map(Result::ok)
//        .collect();
//
//    if catalogue.is_empty() {
//        return Err(format!("No wallpapers found for query: {}", query));
//    }
//
//    Ok(catalogue)
//}
//
//



fn extract_wallpapers_from_folder(body: &str) -> Result<Vec<ImageFile>, String> {
    let document = Html::parse_document(body);
    println!("HTML body: {}", body);

    let file_selector = Selector::parse("#popular a.albumthumbnail").unwrap();

    let image_files = document.select(&file_selector)
        .map(|album_cover| {
            let url_to_imgs = album_cover.value().attr("href")
                .map(|s| format!("https://wallpapercave.com{}", s))
                .unwrap_or_default();

            let preview = album_cover.select(&Selector::parse(".albumphoto img").unwrap())
                .next()
                .and_then(|img| img.value().attr("src"))
                .map(|s| s.to_string())
                .unwrap_or_default();

            let num_imgs = album_cover.select(&Selector::parse(".overlay").unwrap())
                .next()
                .and_then(|span| span.text().next())
                .map(|s| s.to_string())
                .unwrap_or_default();

            let title = album_cover.select(&Selector::parse(".title").unwrap())
                .next()
                .and_then(|p| p.text().next())
                .map(|s| s.to_string())
                .unwrap_or_default();

            ImageFile {
                url_to_imgs,
                preview,
                num_imgs,
                title,
            }
        })
        .collect();

    Ok(image_files)
}


//
//fn extract_wallpapers_from_folder(body: &str) -> Result<Vec<ImageFile>, String> {
//    let document = Html::parse_document(body);
//    let file_selector = Selector::parse("#popular a.albumthumbnail").unwrap();
//
//    let image_files = document.select(&file_selector)
//        .map(|album_cover| {
//            let url_to_imgs = album_cover.value().attr("href")
//                .map(|s| format!("https://wallpapercave.com{}", s))
//                .unwrap_or_default();
//
//            let preview = album_cover.select(&Selector::parse(".albumphoto img").unwrap())
//                .next()
//                .and_then(|img| img.value().attr("src"))
//                .map(|s| s.to_string())
//                .unwrap_or_default();
//
//            let num_imgs = album_cover.select(&Selector::parse(".overlay").unwrap())
//                .next()
//                .and_then(|span| span.text().next())
//                .map(|s| s.to_string())
//                .unwrap_or_default();
//
//            let title = album_cover.select(&Selector::parse(".title").unwrap())
//                .next()
//                .and_then(|p| p.text().next())
//                .map(|s| s.to_string())
//                .unwrap_or_default();
//
//            ImageFile {
//                url_to_imgs,
//                preview,
//                num_imgs,
//                title,
//            }
//        })
//        .collect();
//
//    Ok(image_files)
//}
//



/* async fn download_preview(preview_url: &str, index: usize) -> Result<String, String> {
    let response = reqwest::get(preview_url).await.map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;

    // Get the current working directory (should be src-tauri)
    let current_dir = std::env::current_dir().map_err(|e| e.to_string())?;
    // Navigate up one level to the project root
    let project_root = current_dir.parent()
        .ok_or_else(|| "Unable to find project root directory".to_string())?;
    let preview_dir = project_root.join("public").join("previews");
    fs::create_dir_all(&preview_dir).map_err(|e| e.to_string())?;

    let file_name = format!("preview_{}.jpg", index);
    let file_path = preview_dir.join(&file_name);

    fs::write(&file_path, &bytes).map_err(|e| e.to_string())?;

    Ok(format!("previews/{}", file_name))
} */

async fn download_preview(preview_url: &str, index: usize) -> Result<String, String> {
    let response = reqwest::get(preview_url).await.map_err(|e| e.to_string())?;
    let bytes = response.bytes().await.map_err(|e| e.to_string())?;

    let preview_dir = Path::new("previews");
    fs::create_dir_all(preview_dir).map_err(|e| e.to_string())?;

    let file_name = format!("preview_{}.jpg", index);
    let file_path = preview_dir.join(&file_name);

    fs::write(&file_path, &bytes).map_err(|e| e.to_string())?;

    Ok(file_path.to_string_lossy().into_owned())
}
