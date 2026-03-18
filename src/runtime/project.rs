use serde::Deserialize;
use std::{fs, path::Path};

use oxid::i18n;

#[derive(Deserialize)]
struct OxidConfig {
    title: String,
    width: i32,
    height: i32,
}

#[derive(Deserialize)]
struct PackageJson {
    main: String,
    oxid: OxidConfig,
}

pub struct LoadedProject {
    pub script: String,
    title: String,
    width: i32,
    height: i32,
}

impl LoadedProject {
    pub fn window_config(&self) -> macroquad::conf::Conf {
        let mut conf = macroquad::conf::Conf::default();
        conf.miniquad_conf.window_title = self.title.clone();
        conf.miniquad_conf.window_width = self.width;
        conf.miniquad_conf.window_height = self.height;
        conf
    }
}

#[derive(Deserialize)]
struct OxidLocaleConfig {
    #[serde(default)]
    locale: Option<String>,
}

#[derive(Deserialize)]
struct PackageLocaleProbe {
    #[serde(default)]
    oxid: Option<OxidLocaleConfig>,
}

pub fn detect_locale_from_current_dir() -> Option<String> {
    let manifest_content = fs::read_to_string("package.json").ok()?;
    let package = serde_json::from_str::<PackageLocaleProbe>(&manifest_content).ok()?;

    package.oxid.and_then(|oxid| oxid.locale)
}

pub fn load_from_current_dir() -> Result<LoadedProject, String> {
    let manifest_path = Path::new("package.json");

    if !manifest_path.exists() {
        return Err(i18n::prefixed(
            "runtime",
            "runtime.error.config_file_not_found",
        ));
    }

    let manifest_content = fs::read_to_string(manifest_path).map_err(|err| {
        let source = err.to_string();
        i18n::prefixed_with(
            "runtime",
            "runtime.error.read_config_file",
            &[("source", &source)],
        )
    })?;

    let package: PackageJson = serde_json::from_str(&manifest_content).map_err(|err| {
        let source = err.to_string();
        i18n::prefixed_with("runtime", "runtime.error.parsing", &[("source", &source)])
    })?;

    let entry_path = Path::new(&package.main);

    if !entry_path.exists() {
        return Err(i18n::prefixed_with(
            "runtime",
            "runtime.error.entry_file_not_found",
            &[("entry_file", &package.main)],
        ));
    }

    let script = fs::read_to_string(entry_path).map_err(|err| {
        let source = err.to_string();
        i18n::prefixed_with(
            "runtime",
            "runtime.error.script_read",
            &[("entry_file", &package.main), ("source", &source)],
        )
    })?;

    Ok(LoadedProject {
        script,
        title: package.oxid.title,
        width: package.oxid.width,
        height: package.oxid.height,
    })
}
