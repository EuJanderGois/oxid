use std::path::PathBuf;

pub fn run_project(path: Option<PathBuf>) -> Result<(), String> {
    let project = match path {
        Some(path) => crate::runtime::load_project(&path)?,
        None => crate::runtime::load_project_from_current_dir()?,
    };

    crate::runtime::launch(project);

    Ok(())
}
