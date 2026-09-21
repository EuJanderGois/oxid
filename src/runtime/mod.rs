mod game;
mod project;

use std::path::Path;

pub use project::LoadedProject;

pub fn detect_project_locale(path: Option<&Path>) -> Option<String> {
    match path {
        Some(path) => project::detect_locale(path),
        None => project::detect_locale_from_current_dir(),
    }
}

pub fn load_project(path: &Path) -> Result<LoadedProject, String> {
    project::load(path)
}

pub fn load_project_from_current_dir() -> Result<LoadedProject, String> {
    project::load_from_current_dir()
}

pub fn launch(project: LoadedProject) {
    game::launch(project);
}
