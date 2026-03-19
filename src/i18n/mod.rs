mod catalog;
mod message;
mod render;

use serde_json::Value;
use std::sync::{OnceLock, RwLock};

use catalog::{default_catalog, load_catalog};
use message::{prefix_text, resolve_entry};
use render::render_styled_block;

pub use catalog::DEFAULT_LOCALE;

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
    let (resolved_locale, catalog) = load_catalog(locale)
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
    resolve_entry(&state.catalog, default_catalog(), key, args).into_plain_text()
}

pub fn prefixed(scope: &str, key: &str) -> String {
    prefixed_with(scope, key, &[])
}

pub fn prefixed_with(scope: &str, key: &str, args: &[(&str, &str)]) -> String {
    let state = state().read().expect("i18n state poisoned");
    let prefix = prefix_text(&state.catalog, default_catalog(), scope);
    let entry = resolve_entry(&state.catalog, default_catalog(), key, args);

    render_styled_block(&prefix, entry)
}

fn state() -> &'static RwLock<I18nState> {
    STATE.get_or_init(|| RwLock::new(I18nState::default()))
}
