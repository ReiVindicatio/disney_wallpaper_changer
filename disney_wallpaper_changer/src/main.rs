use std::{fs, path::Path};

use chrono::{DateTime, Datelike, Local};
use reqwest;

async fn download_wallpaper(url: String, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(&url).await?;
    let mut dest = std::fs::File::create(path)?;
    let content = response.bytes().await?;
    std::io::copy(&mut content.as_ref(), &mut dest)?;
    Ok(())
}

fn set_wallpaper(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let script = format!("tell application \"Finder\" to set desktop picture to POSIX file \"{}\"", path);
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("osascript error: {}", stderr).into())
    } else {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let local: DateTime<Local> = Local::now();
    let year = local.year();
    let month = local.month();
    let year_month = format!("{:04}{:02}", year, month);

    let url = format!(
        "https://media2.tokyodisneyresort.jp/home/tdr/wallpaper{}/{}/wallpaper_{}_1.jpg",
        year, month, year_month
    );

    let home_dir = std::env::var("HOME").expect("Could not retrieve home directory");
    let wallpaper_dir = format!("{}/Downloads/wallpaper", home_dir);
    let wallpaper_path = format!("{}/wallpaper{}.jpg", wallpaper_dir, year_month);

    if fs::metadata(&wallpaper_path).is_ok() {
        return Ok(());
    }

    if !Path::new(&wallpaper_dir).exists() {
        fs::create_dir_all(&wallpaper_dir)?;
    }
    download_wallpaper(url, &wallpaper_path).await?;
    set_wallpaper(&wallpaper_path)?;

    Ok(())
}
