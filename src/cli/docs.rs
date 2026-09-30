use std::{fs, path::Path};

use oxid::scripting::api_metadata;

/// Generates the API reference consumed by the Docusaurus site.
///
/// The generated pages intentionally contain only information owned by the
/// scripting metadata: module descriptions, types, properties, constructors,
/// functions and parameters. Conceptual/runtime behavior remains in the
/// hand-written documentation around the generated reference.
pub fn generate_api_docs(output: &Path) -> Result<(), String> {
    fs::create_dir_all(output)
        .map_err(|err| format!("could not create documentation directory: {err}"))?;

    let modules = api_metadata();
    for module in &modules {
        let filename = module
            .name
            .strip_prefix("oxid/")
            .unwrap_or(module.name)
            .replace('/', "-");
        let path = output.join(format!("{filename}.md"));
        fs::write(
            &path,
            oxid::scripting::generator::generate_module_docs_md(module),
        )
        .map_err(|err| format!("could not write {}: {err}", path.display()))?;
    }

    let globals = oxid::scripting::plugins::registry::global_metadata();
    let globals_path = output.join("globals.md");
    fs::write(
        &globals_path,
        oxid::scripting::generator::generate_globals_docs_md(&globals),
    )
    .map_err(|err| format!("could not write {}: {err}", globals_path.display()))?;

    println!(
        "Generated {} API module pages in {}",
        modules.len(),
        output.display()
    );
    Ok(())
}
