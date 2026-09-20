---

title: Installation
slug: /getting-started/installation
---

# Install the CLI

The Oxid CLI is distributed through GitHub Releases and can be installed either by downloading a prebuilt binary or by building the CLI from source.

## Requirements

Choose the installation method that best fits your workflow:

* **Prebuilt binary** — recommended for most users.
* **Cargo** — useful for Rust developers and contributors.

## Option 1: Install a prebuilt binary

Prebuilt Oxid binaries are available for supported platforms on the project's GitHub Releases page.

Each release provides a platform-specific archive:

```text
oxid-vX.Y.Z-windows-x86_64.zip
oxid-vX.Y.Z-linux-x86_64.tar.gz
oxid-vX.Y.Z-macos-x86_64.tar.gz
```

Download the archive corresponding to your operating system and architecture, then extract the `oxid` executable.

### Linux

Place the executable in a directory included in your `PATH`.

For example:

```bash
mkdir -p ~/.local/bin
install -m 755 oxid ~/.local/bin/oxid
```

If `~/.local/bin` is not already in your `PATH`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

For Bash, this can be added to `~/.bashrc` or `~/.profile`.

### Windows

Extract `oxid.exe` from the release archive.

You can either:

* run it from its current directory; or
* add the directory containing `oxid.exe` to your user or system `PATH`.

After adding the directory to `PATH`, restart your terminal so the change is detected.

### macOS

Extract the `oxid` executable and place it in a directory included in your `PATH`.

For example:

```bash
mkdir -p ~/.local/bin
install -m 755 oxid ~/.local/bin/oxid
```

Then ensure that `~/.local/bin` is available in your `PATH`:

```bash
export PATH="$HOME/.local/bin:$PATH"
```

Depending on your shell, this can be added to `~/.zshrc` or another shell profile.

## Option 2: Install with Cargo

If you have Rust and Cargo installed, you can build and install the CLI directly from the repository.

From the repository root:

```bash
cargo install --path .
```

Cargo will install the `oxid` executable into its configured binary directory.

Make sure Cargo's binary directory is available in your `PATH`.

## Verify the installation

After installing Oxid, open a new terminal and run:

```bash
oxid --help
```

If the installation was successful, the CLI help should be displayed.

You can also verify the installed version:

```bash
oxid --version
```

## Next Steps

Once the CLI is installed, create your first Oxid project:

```bash
oxid new my-game
```

Then enter the project directory:

```bash
cd my-game
```

And run it:

```bash
oxid run
```

For more information about the available CLI commands, see the [CLI documentation](/cli).
