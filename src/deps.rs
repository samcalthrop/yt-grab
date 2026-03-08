use std::process::Command;

pub fn check_dependencies() -> Result<(), ErrorDependency> {
    let yt_dlp = Dependency {
        name: "yt-dlp".to_string(),
        install_instructions: InstallInstructions {
            linux: "`sudo apt install yt-dlp`".to_string(),
            macos: "`brew install yt-dlp`".to_string(),
            windows: "Use Scoop (`https://scoop.sh/`) or Chocolatey (`https://chocolatey.org/`)"
                .to_string(),
        },
        site_link: "https://github.com/yt-dlp/yt-dlp".to_string(),
    };
    let ffmpeg = Dependency {
        name: "ffmpeg".to_string(),
        install_instructions: InstallInstructions {
            linux: "`sudo apt install ffmpeg`".to_string(),
            macos: "`brew install ffmpeg`".to_string(),
            windows: "Use Scoop (`https://scoop.sh/`) or Chocolatey (`https://chocolatey.org/`)"
                .to_string(),
        },
        site_link: "https://www.ffmpeg.org/".to_string(),
    };

    if !check_yt_dlp(&yt_dlp.name) {
        eprintln!(
            "Error: `{}` (`{}`) is not installed!",
            yt_dlp.name, yt_dlp.site_link
        );
        return Err(ErrorDependency { dependency: yt_dlp });
    }

    if !check_ffmpeg(&ffmpeg.name) {
        eprintln!(
            "Error: `{}` (`{}`) is not installed!",
            ffmpeg.name, ffmpeg.site_link
        );
        return Err(ErrorDependency { dependency: ffmpeg });
    }

    Ok(())
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

pub struct ErrorDependency {
    pub dependency: Dependency,
}

pub struct Dependency {
    pub name: String,
    pub install_instructions: InstallInstructions,
    pub site_link: String,
}

pub struct InstallInstructions {
    pub linux: String,
    pub macos: String,
    pub windows: String,
}

impl InstallInstructions {
    pub fn for_current_os(&self) -> &str {
        if cfg!(target_os = "linux") {
            &self.linux
        } else if cfg!(target_os = "macos") {
            &self.macos
        } else {
            &self.windows
        }
    }
}
