# Push, Commit, and Pull Request Guidelines

This documentation relates to synchronizing local and remote repositories using Git and GitHub. Please read the information carefully so that we can work together in an organized and clear manner.

## Getting Started: Cloning and Syncing

Before making any changes, you need to get the code and ensure your local environment is up to date with the remote repository.

* **Clone the repository:**
``` bash
git clone https://github.com/EuJanderGois/oxid.git
cd oxid
```

* **Keep your main branch updated:** Before starting any new work, always ensure you are on the main branch and it has the latest changes. 
``` bash
git checkout main
git pull origin main
```

## Local Branching

You should **never** work directly on the `main` branch. Create a new, specific branch for your task directly from the updated `main`:

Example: `git checkout -b feature/change-name` or `git checkout -b fix/error-description`.

## Commit Pattern and Guidelines

After making your changes, you need to use the concept of **Atomic Commits**:

* **Only one type of logical change per commit:** If your current workload is focused on test implementation, for example, you should only add the files related to your tests. Do not mix `fix` or `feat` changes in the same commit.
* **Detail your work:** Keep the commit history clean and descriptive.
* **Conventional Commits:** We follow the standard specification for commit messages. 

### Commit Types

| Type       | When to use | Example |
| :---       | :--- | :--- |
| `feat`     | New functionalities | `feat: add drawRectangle binding` |
| `fix`      | Error/bug fix | `fix: error with Linux window` |
| `docs`     | Documentation changes | `docs: update API documentation` |
| `style`    | Formatting (without logical changes) | `style: adjust mod.ts indentation` |
| `refactor` | Code structure changes | `refactor: simplify drawRectangle function`|
| `perf`     | Performance improvements | `perf: optimize ffi calls` |
| `test`     | Test related changes | `test: add implementation test` |
| `build`    | Build or dependencies changes | `build: update deno tasks` |
| `ci`       | CI/CD changes | `ci: adjust github actions workflow` |
| `chore`    | Maintenance tasks | `chore: include tmp directory to .gitignore` |

## Pushing to Remote

Once your commits are ready, send your branch to the remote repository. 
Run: `git push -u origin branch-name`

## Making the Pull Request (PR)

After pushing your branch, you must open a new PR. You can use the repository template if available to make the review easier.

### GitHub CLI Workflow (Recommended)
With the `gh cli`, the process is easy and integrated:

1. Run: `gh pr create`
2. Select the title of the PR.
3. Select **"Continue in browser"** or **"Edit with editor"**.
4. Fill in the fields and confirm.

## Post-PR Cleanup

Once your PR has been approved and merged into the `main` branch, you need to clean up your local repository to prevent stale branches from accumulating.

1. Switch back to the main branch:
   `git checkout main`
2. Pull the latest changes (which now include your merged PR):
   `git pull origin main`
3. Delete your local feature branch:
   `git branch -d branch-name`

## Project Problems or Needs

If you have identified areas for improvement or have suggestions, please use the **Issues** and **Discussions** tabs.

* If the problem/suggestion already exists, participate in the discussion.
* Otherwise, open a new issue using the **Issue Templates** (Bug Report or Feature Request) that appear when you click "New Issue".

## Final Considerations

Communication is key. Be clear and respectful. Follow the instructions and have fun coding!
If you still have questions, comment on an open issue or use the team's communication channels.