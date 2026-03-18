mod game;
mod project;

pub use project::LoadedProject;

pub fn detect_project_locale_from_current_dir() -> Option<String> {
    project::detect_locale_from_current_dir()
}

pub fn load_project_from_current_dir() -> Result<LoadedProject, String> {
    project::load_from_current_dir()
}

pub fn launch(project: LoadedProject) {
    let config = project.window_config();
    macroquad::Window::from_config(config, game::run_game(project.script));
}
