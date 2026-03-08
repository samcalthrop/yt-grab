use dialoguer::{Confirm, Input, Select};

pub struct Config {
    pub url: String,
    pub destination: String,
    pub file_format: String,
    pub download_playlist: bool,
    pub download_thumbnail: bool,
}

pub fn run_dialog() -> Config {
    let url = get_url();
    let file_format = get_file_format();
    let mut download_playlist = false;
    if url.contains("list=") {
        download_playlist = should_download_playlist();
    }
    let destination = get_destination();
    let download_thumbnail = should_download_thumbnail();

    return Config {
        url,
        destination,
        file_format,
        download_playlist,
        download_thumbnail,
    };
}

macro_rules! confirm {
    ($prompt:expr, $default:expr) => {{
        Confirm::new()
            .with_prompt($prompt)
            .default($default)
            .interact()
            .unwrap()
    }};
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
