[Setup]
; Basic Information
AppName=Volumetrik
AppVersion=0.1.0
AppPublisher=Odron
DefaultDirName={autopf}\Volumetrik
DefaultGroupName=Volumetrik
UninstallDisplayIcon={app}\volumetrik.exe
Compression=lzma2
SolidCompression=yes
OutputDir=installers
OutputBaseFilename=VolumetrikSetup
; Si vous convertissez votre logo.png en logo.ico, décommentez la ligne suivante :
SetupIconFile=logo.ico

[Files]
; The main executable
Source: "target\release\volumetrik.exe"; DestDir: "{app}"; Flags: ignoreversion
; Include the logo if needed (though it's embedded in the exe now)
Source: "logo.png"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
; Start Menu shortcut
Name: "{group}\Volumetrik"; Filename: "{app}\volumetrik.exe"
; Desktop shortcut
Name: "{autodesktop}\Volumetrik"; Filename: "{app}\volumetrik.exe"; Tasks: desktopicon

[Tasks]
Name: desktopicon; Description: "Créer une icône sur le &Bureau"; GroupDescription: "Icônes supplémentaires :"

[Run]
Filename: "{app}\volumetrik.exe"; Description: "Lancer Volumetrik"; Flags: nowait postinstall skipifsilent
