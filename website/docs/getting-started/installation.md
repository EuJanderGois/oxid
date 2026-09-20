---
title: Installation
slug: /getting-started/installation
---

# Install the CLI

Oxid does **not** ship an official `curl | sh` installer in this repository yet.

If you are looking for a one-line install script, treat that as a planned improvement rather than a documented feature. Today, the supported path is to build or install the CLI from the project releases.

## Option 1: Place the binary in your environment

If you prefer a local user install, take the release binary and copy it manually:

In most **Linux** distributions you can:

```bash
install -Dm755 downloads/oxid-v0.1.1 ~/.local/bin/oxid
```

On **Windows**, you can edit the system environment variables and add the path to the binary to `Path`.

### Add `~/.local/bin` to your `PATH`

If `~/.local/bin` is not already available in your shell, add it to your profile:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

For Bash, that usually means placing the line above in `~/.bashrc` or `~/.profile`.

## Option 2: Install with Cargo

From the repository root:

```bash
cargo install --path .
```

This installs the `oxid` binary into Cargo's bin directory.

## Verify the installation

After installing the binary, confirm that the CLI is available:

```bash
oxid --help
```

At the moment the CLI surface is intentionally small, focused on:

- `oxid new`
- `oxid run`
- `--lang <locale>` as a global option