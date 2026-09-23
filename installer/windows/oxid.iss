#define MyAppName "Oxid"
#define MyAppPublisher "EuJanderGois"
#define MyAppURL "https://eujandergois.github.io/oxid"

#ifndef MyAppVersion
  #define MyAppVersion "0.0.0"
#endif

[Setup]
AppId={{B8E8B7D1-7B3B-4D2C-A5C1-OXIDENGINE}}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}
AppUpdatesURL={#MyAppURL}

DefaultDirName={autopf}\Oxid
DefaultGroupName=Oxid

OutputDir=..\..\release
OutputBaseFilename=Oxid-v{#MyAppVersion}-Setup

Compression=lzma
SolidCompression=yes
WizardStyle=modern

ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

[Files]
Source: "..\..\target\release\oxid.exe"; DestDir: "{app}"; Flags: ignoreversion