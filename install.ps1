$ErrorActionPreference = "Stop"

# Configuration
$AppName = "Volumetrik"
$ExeName = "volumetrik.exe"
$SourceExe = "target\release\$ExeName"
$InstallDir = "$env:LOCALAPPDATA\Programs\$AppName"
$StartMenuDir = "$env:APPDATA\Microsoft\Windows\Start Menu\Programs"
$ShortcutPath = "$StartMenuDir\$AppName.lnk"

# Check if release build exists
if (-not (Test-Path $SourceExe)) {
    Write-Host "Release build not found. Building..."
    cargo build --release
    if ($LASTEXITCODE -ne 0) {
        Write-Error "Build failed."
    }
}

# Create Install Directory
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
}

# Copy Executable
Write-Host "Installing to $InstallDir..."
Copy-Item -Path $SourceExe -Destination "$InstallDir\$ExeName" -Force

# Create Shortcut
Write-Host "Creating Start Menu shortcut..."
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut($ShortcutPath)
$Shortcut.TargetPath = "$InstallDir\$ExeName"
$Shortcut.WorkingDirectory = $InstallDir
$Shortcut.Description = "Volumetrik Disk Analyzer"
$Shortcut.Save()

Write-Host "Installation Complete!"
Write-Host "You can now find '$AppName' in your Start Menu."
