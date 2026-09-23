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

OutputDir=....\release
OutputBaseFilename=Oxid-v{#MyAppVersion}-Setup

Compression=lzma
SolidCompression=yes
WizardStyle=modern

ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible

ChangesEnvironment=yes

Uninstallable=yes
UninstallDisplayName=Oxid

[Files]
Source: "....\target\release\oxid.exe"; DestDir: "{app}"; Flags: ignoreversion

[Code]

function NormalizePath(const Value: string): string;
begin
Result := Value;

while (Length(Result) > 0) and (Result[Length(Result)] = '') do
Delete(Result, Length(Result), 1);
end;

procedure AddToUserPath;
var
CurrentPath: string;
OxidPath: string;
begin
OxidPath := NormalizePath(ExpandConstant('{app}'));

if not RegQueryStringValue(
HKEY_CURRENT_USER,
'Environment',
'Path',
CurrentPath
) then
CurrentPath := '';

if Pos(';' + LowerCase(OxidPath) + ';', ';' + LowerCase(CurrentPath) + ';') > 0 then
Exit;

if CurrentPath = '' then
CurrentPath := OxidPath
else
CurrentPath := CurrentPath + ';' + OxidPath;

RegWriteExpandStringValue(
HKEY_CURRENT_USER,
'Environment',
'Path',
CurrentPath
);
end;

procedure RemoveFromUserPath;
var
CurrentPath: string;
OxidPath: string;
Parts: TArrayOfString;
NewPath: string;
I: Integer;
begin
if not RegQueryStringValue(
HKEY_CURRENT_USER,
'Environment',
'Path',
CurrentPath
) then
Exit;

OxidPath := LowerCase(NormalizePath(ExpandConstant('{app}')));

Parts := SplitString(CurrentPath, ';');
NewPath := '';

for I := 0 to GetArrayLength(Parts) - 1 do
begin
if NormalizePath(LowerCase(Parts[I])) = OxidPath then
Continue;

```
if Parts[I] = '' then
  Continue;

if NewPath = '' then
  NewPath := Parts[I]
else
  NewPath := NewPath + ';' + Parts[I];
```

end;

RegWriteExpandStringValue(
HKEY_CURRENT_USER,
'Environment',
'Path',
NewPath
);
end;

procedure CurStepChanged(CurStep: TSetupStep);
begin
if CurStep = ssPostInstall then
AddToUserPath;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
begin
if CurUninstallStep = usPostUninstall then
RemoveFromUserPath;
end;
