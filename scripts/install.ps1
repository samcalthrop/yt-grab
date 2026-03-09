$repo = "samcalthrop/yt-grab"

$release = Invoke-RestMethod https://api.github.com/repos/$repo/releases/latest

$file = "yt-grab-windows-x64.exe"

$url = "https://github.com/$repo/releases/download/$($release.tag_name)/$file"

$outDir = "$env:USERPROFILE\yt-grab"
New-Item -ItemType Directory -Force -Path $outDir | Out-Null

$outFile = "$outDir\yt-grab.exe"

Invoke-WebRequest $url -OutFile $outFile

$path = [Environment]::GetEnvironmentVariable("Path", "User")

if ($path -notlike "*$outDir*") {
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$path;$outDir",
        "User"
    )
}

Write-Host "Installed to $outFile"
Write-Host "Restart terminal and run 'yt-grab'"
