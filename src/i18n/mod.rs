use serde_json::Value;
use std::{
    env, fs,
    path::{Path, PathBuf},
    sync::{OnceLock, RwLock},
};

pub const DEFAULT_LOCALE: &str = "pt-BR";

const PT_BR_CATALOG: &str = include_str!("../assets/locales/pt-BR.json");
const EN_US_CATALOG: &str = include_str!("../assets/locales/en-US.json");

static DEFAULT_CATALOG: OnceLock<Value> = OnceLock::new();
static STATE: OnceLock<RwLock<I18nState>> = OnceLock::new();

struct I18nState {
    locale: String,
    catalog: Value,
}

impl Default for I18nState {
    fn default() -> Self {
        Self {
            locale: DEFAULT_LOCALE.to_string(),
            catalog: default_catalog().clone(),
        }
    }
}

pub fn set_locale(locale: &str) -> String {
    let normalized = normalize_locale(locale);
    let (resolved_locale, catalog) = load_catalog(&normalized)
        .unwrap_or_else(|| (DEFAULT_LOCALE.to_string(), default_catalog().clone()));

    let mut state = state().write().expect("i18n state poisoned");
    state.locale = resolved_locale.clone();
    state.catalog = catalog;

    resolved_locale
}

pub fn current_locale() -> String {
    state().read().expect("i18n state poisoned").locale.clone()
}

pub fn text(key: &str) -> String {
    text_with(key, &[])
}

pub fn text_with(key: &str, args: &[(&str, &str)]) -> String {
    let state = state().read().expect("i18n state poisoned");
    let template = lookup(&state.catalog, key)
        .or_else(|| lookup(default_catalog(), key))
        .unwrap_or(key);

    format_template(template, args)
}

pub fn prefixed(scope: &str, key: &str) -> String {
    prefixed_with(scope, key, &[])
}

pub fn prefixed_with(scope: &str, key: &str, args: &[(&str, &str)]) -> String {
    let prefix_key = format!("{scope}.prefix");
    format!("{}{}", text(&prefix_key), text_with(key, args))
}

fn state() -> &'static RwLock<I18nState> {
    STATE.get_or_init(|| RwLock::new(I18nState::default()))
}

fn default_catalog() -> &'static Value {
    DEFAULT_CATALOG.get_or_init(|| {
        serde_json::from_str(PT_BR_CATALOG).expect("default locale catalog must be valid JSON")
    })
}

fn load_catalog(locale: &str) -> Option<(String, Value)> {
    for candidate in locale_candidates(locale) {
        if let Some(builtin) = builtin_catalog(&candidate) {
            if let Ok(catalog) = serde_json::from_str(builtin) {
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

        let Ok(catalog) = serde_json::from_str(&contents) else {
            continue;
        };

        return Some(catalog);
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
                .join(format!("{locale}.json")),
        );
    }

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

fn lookup<'a>(catalog: &'a Value, key: &str) -> Option<&'a str> {
    let mut current = catalog;

    for segment in key.split('.') {
        current = current.get(segment)?;
    }

    current.as_str()
}

fn format_template(template: &str, args: &[(&str, &str)]) -> String {
    let mut rendered = template.to_string();

    for (name, value) in args {
        rendered = rendered.replace(&format!("{{{name}}}"), value);
    }

    rendered
}
