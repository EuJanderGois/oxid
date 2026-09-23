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

OutputDir=..\..\release
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
Source: "..\..\target\release\oxid.exe"; DestDir: "{app}"; Flags: ignoreversion

[Code]

function NormalizePath(const Value: string): string;
begin
  Result := Trim(Value);

  while (Length(Result) > 0) and
        (Result[Length(Result)] = '\') do
    Delete(Result, Length(Result), 1);
end;

function PathContains(const Path: string; const Target: string): Boolean;
var
  Remaining: string;
  Entry: string;
  Separator: Integer;
begin
  Result := False;
  Remaining := Path;

  while Remaining <> '' do
  begin
    Separator := Pos(';', Remaining);

    if Separator = 0 then
    begin
      Entry := Remaining;
      Remaining := '';
    end
    else
    begin
      Entry := Copy(Remaining, 1, Separator - 1);
      Delete(Remaining, 1, Separator);
    end;

    Entry := NormalizePath(Entry);

    if CompareText(Entry, Target) = 0 then
    begin
      Result := True;
      Exit;
    end;
  end;
end;

function RemovePathEntry(const Path: string; const Target: string): string;
var
  Remaining: string;
  Entry: string;
  Separator: Integer;
  NewPath: string;
begin
  Remaining := Path;
  NewPath := '';

  while Remaining <> '' do
  begin
    Separator := Pos(';', Remaining);

    if Separator = 0 then
    begin
      Entry := Remaining;
      Remaining := '';
    end
    else
    begin
      Entry := Copy(Remaining, 1, Separator - 1);
      Delete(Remaining, 1, Separator);
    end;

    Entry := Trim(Entry);

    if Entry = '' then
      Continue;

    if CompareText(NormalizePath(Entry), Target) = 0 then
      Continue;

    if NewPath = '' then
      NewPath := Entry
    else
      NewPath := NewPath + ';' + Entry;
  end;

  Result := NewPath;
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

  if PathContains(CurrentPath, OxidPath) then
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
  NewPath: string;
begin
  if not RegQueryStringValue(
    HKEY_CURRENT_USER,
    'Environment',
    'Path',
    CurrentPath
  ) then
    Exit;

  OxidPath := NormalizePath(ExpandConstant('{app}'));

  NewPath := RemovePathEntry(CurrentPath, OxidPath);

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