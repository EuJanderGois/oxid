use std::{fs, path::Path};

use oxid::i18n;

use super::templates;

pub fn create_project(project_name: &str, locale: &str) -> Result<(), String> {
    let path = Path::new(project_name);

    if path.exists() {
        return Err(i18n::prefixed_with(
            "cli",
            "cli.error.directory_already_exists",
            &[("project_name", project_name)],
        ));
    }

    fs::create_dir(path).map_err(|err| {
        let source = err.to_string();
        i18n::prefixed_with(
            "cli",
            "cli.error.creating_dir",
            &[("project_name", project_name), ("source", &source)],
        )
    })?;

    write_project_file(
        path,
        "package.json",
        &templates::package_json(project_name, locale),
    )?;
    write_project_file(path, "main.js", templates::MAIN_JS)?;
    write_project_file(path, "tsconfig.json", templates::TSCONFIG_JSON)?;
    write_project_file(path, "oxid.d.ts", templates::OXID_D_TS)?;

    println!(
        "{}",
        i18n::prefixed_with(
            "cli",
            "cli.logs.project_created",
            &[("project_name", project_name)],
        )
    );
    println!(
        "{}",
        i18n::prefixed_with(
            "cli",
            "cli.logs.first_run",
            &[("project_name", project_name)],
        )
    );

    Ok(())
}

fn write_project_file(project_dir: &Path, filename: &str, contents: &str) -> Result<(), String> {
    fs::write(project_dir.join(filename), contents).map_err(|err| {
        let source = err.to_string();
        i18n::prefixed_with(
            "cli",
            "cli.error.write_file",
            &[("filename", filename), ("source", &source)],
        )
    })
}
