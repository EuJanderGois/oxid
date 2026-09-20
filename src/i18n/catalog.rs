use jsonc_parser::parse_to_serde_value;
use serde_json::Value;
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub const DEFAULT_LOCALE: &str = "pt-BR";

const PT_BR_CATALOG: &str = include_str!("../assets/locales/pt-BR.jsonc");
const EN_US_CATALOG: &str = include_str!("../assets/locales/en-US.jsonc");

static DEFAULT_CATALOG: OnceLock<Value> = OnceLock::new();

pub fn default_catalog() -> &'static Value {
    DEFAULT_CATALOG.get_or_init(|| {
        parse_catalog(PT_BR_CATALOG).expect("default locale catalog must be valid JSONC")
    })
}

pub fn load_catalog(locale: &str) -> Option<(String, Value)> {
    for candidate in locale_candidates(locale) {
        if let Some(builtin) = builtin_catalog(&candidate) {
            if let Some(catalog) = parse_catalog(builtin) {
                return Some((candidate, catalog));
            }
        }

        if let Some(catalog) = load_catalog_from_paths(&candidate) {
            return Some((candidate, catalog));
        }
    }

    None
}

fn builtin_catalog(locale: &str) -> Option<&'static str> {
    match locale {
        "pt-BR" => Some(PT_BR_CATALOG),
        "en-US" => Some(EN_US_CATALOG),
        _ => None,
    }
}

fn load_catalog_from_paths(locale: &str) -> Option<Value> {
    for path in locale_search_paths(locale) {
        let Ok(contents) = fs::read_to_string(path) else {
            continue;
        };

        if let Some(catalog) = parse_catalog(&contents) {
            return Some(catalog);
        }
    }

    None
}

fn locale_search_paths(locale: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    if let Some(exe_path) = env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(Path::to_path_buf))
    {
        paths.push(
            exe_path
                .join("assets/locales")
                .join(format!("{locale}.jsonc")),
        );
        paths.push(
            exe_path
                .join("assets/locales")
                .join(format!("{locale}.json")),
        );
    }

    paths.push(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/assets/locales")
            .join(format!("{locale}.jsonc")),
    );
    paths.push(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("src/assets/locales")
            .join(format!("{locale}.json")),
    );

    paths
}

fn locale_candidates(locale: &str) -> Vec<String> {
    let trimmed = locale.trim();
    let normalized = normalize_locale(trimmed);

    if trimmed.is_empty() || trimmed == normalized {
        vec![normalized]
    } else {
        vec![trimmed.to_string(), normalized]
    }
}

fn normalize_locale(locale: &str) -> String {
    let sanitized = locale
        .trim()
        .split(['.', '@'])
        .next()
        .unwrap_or_default()
        .replace('_', "-");

    let mut parts = sanitized
        .split('-')
        .filter(|part| !part.is_empty())
        .peekable();

    let Some(language) = parts.next() else {
        return DEFAULT_LOCALE.to_string();
    };

    let mut normalized = language.to_ascii_lowercase();

    for part in parts {
        normalized.push('-');

        if part.len() == 2 {
            normalized.push_str(&part.to_ascii_uppercase());
        } else {
            normalized.push_str(part);
        }
    }

    normalized
}

fn parse_catalog(text: &str) -> Option<Value> {
    parse_to_serde_value(text, &Default::default())
        .ok()
        .flatten()
}
