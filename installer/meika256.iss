[Setup]
AppName=Meika256
AppVersion=1.0.0
DefaultDirName={pf}\Meika256
DefaultGroupName=Meika256
OutputDir=installer\output
OutputBaseFilename=Meika256-Setup
Compression=lzma
SolidCompression=yes
ArchitecturesInstallIn64BitMode=x64
DisableProgramGroupPage=yes

[Files]
Source: "staging\bin\meika256-cli.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "staging\bin\meika256.dll"; DestDir: "{app}\sdk\bin"; Flags: ignoreversion
Source: "staging\sdk\include\meika256.h"; DestDir: "{app}\sdk\include"; Flags: ignoreversion
Source: "staging\sdk\lib\meika256.lib"; DestDir: "{app}\sdk\lib"; Flags: ignoreversion

[Icons]
Name: "{group}\Meika256 CLI"; Filename: "{app}\meika256-cli.exe"

[Run]
Filename: "{app}\meika256-cli.exe"; Description: "Run Meika256 CLI"; Flags: nowait postinstall skipifsilent
