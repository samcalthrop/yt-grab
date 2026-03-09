use std::process::{Command, exit};

use crate::prompts::Config;

pub fn download(config: Config) {
    check_yt_dlp();
    check_ffmpeg();

    let mut args: Vec<String> = vec![];
    // Download audio?
    if config.file_format == "mp3" {
        args.push("-x".to_string());
        args.push("--audio-format".to_string());
        args.push("mp3".to_string());
    // Download video?
    } else if config.file_format == "mp4" {
        args.push("--merge-output-format".to_string());
        args.push("mp4".to_string());
        args.push("--remux-video".to_string());
        args.push("mp4".to_string());
        args.push("-S".to_string());
        args.push("vcodec:h264,lang,quality,res,fps,hdr:12".to_string());
    }
    // Download playlist?
    if config.download_playlist {
        args.push("-o".to_string());
        args.push("%(playlist)s/%(playlist_index)s - %(title)s.%(ext)s".to_string());
    }
    // Download thumbnail?
    if config.download_thumbnail {
        args.push("--write-thumbnail".to_string());
    }
    // Destination path
    args.push("-P".to_string());
    args.push(config.destination);
    // URL
    args.push(config.url);

    Command::new("yt-dlp")
        .args(&args)
        .status()
        .expect("failed to run yt-dlp");

    // Command::new("ytd")
    //     .arg("-d")
    //     .arg(&config.url)
    //     .arg(config.file_format)
    //     .arg(config.download_playlist.to_string())
    //     .arg(config.destination)
    //     .arg(config.download_thumbnail.to_string())
    //     .status()
    //     .unwrap();
}

fn check_yt_dlp() {
    if Command::new("yt-dlp").arg("--version").output().is_err() {
        eprintln!("yt-dlp not found. Please install it.");
        exit(1);
    }
}

fn check_ffmpeg() {
    if Command::new("ffmpeg").arg("-version").output().is_err() {
        eprintln!("ffmpeg not found. Please install it.");
        exit(1);
    }
}
