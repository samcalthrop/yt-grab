use dialoguer::{Confirm, Input, Select};
use std::process::Command;

// #[cfg(target_os = "windows")]
// const OS: &str = "windows";

// #[cfg(target_os = "macos")]
// const OS: &str = "macos";

// #[cfg(target_os = "linux")]
// const OS: &str = "linux";

macro_rules! confirm {
    ($prompt:expr, $default:expr) => {{
        Confirm::new()
            .with_prompt($prompt)
            .default($default)
            .interact()
            .unwrap()
    }};
}

fn main() {
    // check dependecies are installed
    if !check_yt_dlp("yt-dlp") {
        eprintln!("Error: `yt-dlp` (`https://github.com/yt-dlp/yt-dlp`) is not installed!");
        if cfg!(target_os = "windows") {
            eprintln!("Try installing via Scoop (`https://scoop.sh/`) or Chocolatey (`https://chocolatey.org/`)")
        } else if cfg!(target_os = "macos") {
            eprintln!("Try `brew install yt-dlp`")
        } else {
            eprintln!("Try `sudo apt install yt-dlp`")
        }
    }

    if !check_ffmpeg("ffmpeg") {
        eprintln!("Error: `ffmpeg` (`https://www.ffmpeg.org/`) is not installed!");
        if cfg!(target_os = "windows") {
            eprintln!("Try installing via Scoop (`https://scoop.sh/`) or Chocolatey (`https://chocolatey.org/`)")
        } else if cfg!(target_os = "macos") {
            eprintln!("Try `brew install ffmpeg`")
        } else {
            eprintln!("Try `sudo apt install ffmpeg`")
        }
    }

    let url = get_url();
    let file_format = get_file_format();
    let mut playlist_option = false;
    if url.contains("list=") {
        playlist_option = should_download_playlist();
    }
    let destination = get_destination();
    let thumbnail_option = should_download_thumbnail();

    Command::new("ytd")
        .arg("-d")
        .arg(&url)
        .arg(file_format)
        .arg(playlist_option.to_string())
        .arg(destination)
        .arg(thumbnail_option.to_string())
        .status()
        .unwrap();
}

fn get_url() -> String {
    Input::new().with_prompt("URL: ").interact_text().unwrap()
}

fn get_destination() -> String {
    let dir = Input::new()
        .with_prompt("Download destination")
        .default("cwd".to_string())
        .interact_text()
        .unwrap();
    if dir == "cwd" {
        return "./".to_string();
    } else {
        return dir;
    }
}

fn get_file_format() -> String {
    let file_formats = vec!["mp3", "mp4"];
    let index = Select::new()
        .with_prompt("File format")
        .items(&file_formats)
        .default(0)
        .interact()
        .unwrap();
    file_formats[index].to_string()
}

fn should_download_playlist() -> bool {
    // let options = vec!["yes", "no"];
    // let index = Select::new()
    //     .with_prompt("Download entire playlist?")
    //     .items(&options)
    //     .default(0)
    //     .interact()
    //     .unwrap();
    // options[index] == "yes"
    confirm!("Download entire playlist?", true)
}

fn should_download_thumbnail() -> bool {
    confirm!("Download thumbnail?", false)
}

fn check_yt_dlp(program: &str) -> bool {
    Command::new(program)
        .arg("--version") // most CLI tools support this
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn check_ffmpeg(program: &str) -> bool {
    Command::new(program)
        .arg("-version") // most CLI tools support this
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
