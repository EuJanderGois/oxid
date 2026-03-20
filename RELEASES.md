# Versioning with Tags

Tags are used to mark specific release points in your project's history (e.g., `v1.0.0`, `v1.1.0`). We recommend following Semantic Versioning.

## 1. Creating an Annotated Tag

Before tagging, ensure you are on the updated `main` branch. We use **annotated tags** for releases because they store metadata like the date, tagger name, and a release message.

```bash
git checkout main
git pull origin main

# Create the tag
git tag -a v1.0.0 -m "Release version 1.0.0"
```

## 2. Pushing Tags to GitHub

Regular `git push` commands do not send tags to the remote repository. You need to push them explicitly.

Push a single specific tag:
```bash
git push origin v1.0.0
```

Or push all local tags at once:
```bash
git push origin --tags
```

## 3. Deleting a Tag (If you make a mistake)

If you need to remove a tag, you must delete it both locally and remotely.

```bash
# Delete locally
git tag -d v1.0.0

# Delete from the remote repository
git push origin --delete v1.0.0
```