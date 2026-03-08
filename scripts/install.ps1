Invoke-WebRequest `
  -Uri https://github.com/USER/yt-grab/releases/latest/download/yt-grab-windows.exe `
  -OutFile yt-grab.exe

Move-Item yt-grab.exe $env:USERPROFILE\bin\
