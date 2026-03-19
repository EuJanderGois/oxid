---
title: Installation
slug: /getting-started/installation
---

# Install the CLI on Linux

Oxid does **not** ship an official `curl | sh` installer in this repository yet.

If you are looking for a one-line install script, treat that as a planned improvement rather than a documented feature. Today, the supported path is to build or install the CLI from the Rust project itself.

## Option 1: Install with Cargo

From the repository root:

```bash
cargo install --path .
```

This installs the `oxid` binary into Cargo's bin directory.

## Option 2: Build and place the binary in `~/.local/bin`

If you prefer a local user install without `cargo install`, build the release binary and copy it manually:

```bash
cargo build --release
install -Dm755 target/release/oxid ~/.local/bin/oxid
```

## Add `~/.local/bin` to your `PATH`

If `~/.local/bin` is not already available in your shell, add it to your profile:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

For Bash, that usually means placing the line above in `~/.bashrc` or `~/.profile`.

## Verify the installation

After installing the binary, confirm that the CLI is available:

```bash
oxid --help
```

At the moment the CLI surface is intentionally small, focused on:

- `oxid new`
- `oxid run`
- `--lang <locale>` as a global option
