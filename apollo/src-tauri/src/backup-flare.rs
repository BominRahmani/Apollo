use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path; // Add this line to import Path


#[derive(Debug, Serialize, Deserialize)]
pub struct Wallpaper {
    url: String,
    preview: String,
    resolution: String,
    local_preview: String,
}

const BASE_WALLPAPER_SOURCE: &str = "https://wallpaperflare.com/search?wallpaper=";

fn format_query(query: &str) -> String {
    query.replace(" ", "+")
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

fn extract_wallpapers(body: &str) -> Result<Vec<Wallpaper>, String> {
    let document = Html::parse_document(body);
    let gallery_selector = Selector::parse("ul[itemtype='http://schema.org/ImageGallery'][class='gallery'][id='gallery']").unwrap();
    let item_selector = Selector::parse("li[itemprop='associatedMedia'][itemscope][itemtype='http://schema.org/ImageObject']").unwrap();

    let mut wallpapers = Vec::new();

    if let Some(gallery) = document.select(&gallery_selector).next() {
        for item in gallery.select(&item_selector) {
            let image_preview = item
                .select(&Selector::parse("figure a img[itemprop='contentUrl']").unwrap())
                .next()
                .and_then(|el| el.value().attr("data-src"))
                .unwrap_or("")
                .to_string();

            let image_url = item
                .select(&Selector::parse("figure a[itemprop='url']").unwrap())
                .next()
                .and_then(|el| el.value().attr("href"))
                .unwrap_or("")
                .to_string();

            let image_resolution = item
                .select(&Selector::parse("span.res").unwrap())
                .next()
                .map(|el| el.text().collect::<String>().trim().to_string())
                .unwrap_or_default();

            wallpapers.push(Wallpaper {
                url: image_url,
                preview: image_preview,
                resolution: image_resolution,
                local_preview: String::new(), // This will be filled later
            });
        }
    }

    Ok(wallpapers)
}

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
