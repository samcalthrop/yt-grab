use std::process::Command;

use crate::prompts::Config;

pub fn download(config: Config) {
    Command::new("ytd")
        .arg("-d")
        .arg(&config.url)
        .arg(config.file_format)
        .arg(config.download_playlist.to_string())
        .arg(config.destination)
        .arg(config.download_thumbnail.to_string())
        .status()
        .unwrap();
}
