# Versioning & Releases

Oxid uses semantic version numbers for stable releases.

Versions follow the format:

```text
MAJOR.MINOR.PATCH
```

For example:

```text
0.1.1
```

The project's Cargo package version is the source of truth for the Oxid version.

```toml
[package]
version = "0.1.1"
```

## Version Components

### MAJOR

Indicates major incompatible changes to the public API or project architecture.

```text
1.0.0 → 2.0.0
```

### MINOR

Indicates new functionality that remains compatible with the existing API.

```text
0.1.0 → 0.2.0
```

### PATCH

Indicates backwards-compatible fixes and small corrections.

```text
0.1.0 → 0.1.1
```

During early development, Oxid may remain in the `0.x` range while its APIs and architecture continue to evolve.

## Version Source of Truth

The version declared in `Cargo.toml` must match the release tag.

For example:

```text
Cargo.toml
    version = "0.1.1"

          ↓

Git tag
    v0.1.1
```

The release workflow automatically verifies this relationship.

A release will fail if the tag and package version do not match.

## Release Tags

Stable releases use tags in the following format:

```text
vX.Y.Z
```

Examples:

```text
v0.1.0
v0.1.1
v0.2.0
v1.0.0
```

Release tags must point to commits contained in `main`.

The `next` branch is intended for development and integration and should not be used as the source of stable release tags.

## Creating a Release

Once the desired changes have been merged into `main`, update the version in `Cargo.toml`.

For example:

```toml
version = "0.1.1"
```

After the version change has been merged into `main`, create the corresponding tag:

```bash
git checkout main
git pull

git tag v0.1.1
git push origin v0.1.1
```

Pushing the tag starts the release workflow automatically.

## Release Validation

Before publishing a release, the automation verifies:

* the tag points to a commit contained in `main`;
* the tag version matches `Cargo.toml`;
* the project passes formatting checks;
* the project passes compilation checks;
* tests pass;
* release binaries can be built.

A release is not published when these validations fail.

## Release Artifacts

Oxid releases are built for multiple platforms.

The release workflow produces platform-specific archives such as:

```text
oxid-v0.1.1-windows-x86_64.zip
oxid-v0.1.1-linux-x86_64.tar.gz
oxid-v0.1.1-macos-x86_64.tar.gz
```

These artifacts are attached automatically to the corresponding GitHub Release.

## Release Checklist

Before creating a release:

* [ ] Development changes are merged into `main`.
* [ ] `Cargo.toml` contains the intended version.
* [ ] The version change is present in `main`.
* [ ] The working tree is clean.
* [ ] The release tag matches the Cargo version.
* [ ] The tag is created from `main`.
* [ ] The tag is pushed to GitHub.

After pushing the tag:

* [ ] Release workflow completes successfully.
* [ ] GitHub Release is created.
* [ ] Platform binaries are available as release assets.
