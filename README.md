A beginner-friendly light weight command line tool for downloading youtube videos.
UX is prioritised by using selection and confirmation dialogues where possible to make the task much easier and less error-prone than using lots of flags.

# Installation

Run the command corresponding to your OS in your terminal:

## UNIX (_macOS/ linux_)

```bash
curl -fsSL https://raw.githubusercontent.com/samcalthrop/yt-grab/main/scripts/install.sh | bash
```

## Windows

```pwsh
powershell -ExecutionPolicy Bypass -c "irm https://raw.githubusercontent.com/samcalthrop/yt-grab/main/scripts/install.ps1 | iex"
```

# Usage

Simply type the keyword `yt-grab` into your terminal and press `enter`. You'll then be guided through a dialog allowing you to choose:

- File format (`mp3`/`mp4`, `default=mp3`)
- Whether to download the entire playlist (where applicable, `default=true`)
- Whether to download the video's thumbnail (`default=false`)
- Where to save the downloaded content (`default=cwd`)

Then, your video(s) will be downloaded!
