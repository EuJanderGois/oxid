use serde::Deserialize;
use serde_json::Value;

#[derive(Clone, Deserialize)]
#[serde(untagged)]
enum CatalogEntry {
    Text(String),
    Rich(RichCatalogEntry),
}

#[derive(Clone, Deserialize)]
struct RichCatalogEntry {
    #[serde(default)]
    tone: Option<String>,
    #[serde(default)]
    color: Option<String>,
    #[serde(default)]
    icon: Option<String>,
    message: MessageBody,
}

#[derive(Clone, Deserialize)]
#[serde(untagged)]
enum MessageBody {
    One(String),
    Many(Vec<String>),
}

pub(crate) struct ResolvedEntry {
    pub(crate) tone: Option<String>,
    pub(crate) color: Option<String>,
    pub(crate) icon: Option<String>,
    pub(crate) lines: Vec<String>,
}

pub(crate) fn resolve_entry(
    active_catalog: &Value,
    default_catalog: &Value,
    key: &str,
    args: &[(&str, &str)],
) -> ResolvedEntry {
    lookup_entry(active_catalog, key, args)
        .or_else(|| lookup_entry(default_catalog, key, args))
        .unwrap_or_else(|| ResolvedEntry::plain(vec![format_template(key, args)]))
}

pub(crate) fn prefix_text(active_catalog: &Value, default_catalog: &Value, scope: &str) -> String {
    let key = format!("{scope}.prefix");

    lookup(active_catalog, &key)
        .and_then(Value::as_str)
        .or_else(|| lookup(default_catalog, &key).and_then(Value::as_str))
        .unwrap_or_default()
        .to_string()
}

impl ResolvedEntry {
    pub(crate) fn into_plain_text(self) -> String {
        if self.lines.is_empty() {
            String::new()
        } else {
            self.lines.join("\n")
        }
    }

    fn plain(lines: Vec<String>) -> Self {
        Self {
            tone: None,
            color: None,
            icon: None,
            lines,
        }
    }

    fn from_catalog_entry(entry: CatalogEntry, args: &[(&str, &str)]) -> Self {
        match entry {
            CatalogEntry::Text(text) => Self::plain(vec![format_template(&text, args)]),
            CatalogEntry::Rich(entry) => Self {
                tone: entry.tone,
                color: entry.color,
                icon: entry.icon,
                lines: render_message_body(entry.message, args),
            },
        }
    }
}

fn lookup_entry(catalog: &Value, key: &str, args: &[(&str, &str)]) -> Option<ResolvedEntry> {
    let raw = lookup(catalog, key)?.clone();
    let entry = serde_json::from_value::<CatalogEntry>(raw).ok()?;
    Some(ResolvedEntry::from_catalog_entry(entry, args))
}

fn lookup<'a>(catalog: &'a Value, key: &str) -> Option<&'a Value> {
    let mut current = catalog;

    for segment in key.split('.') {
        current = current.get(segment)?;
    }

    Some(current)
}

fn render_message_body(body: MessageBody, args: &[(&str, &str)]) -> Vec<String> {
    match body {
        MessageBody::One(line) => vec![format_template(&line, args)],
        MessageBody::Many(lines) => lines
            .into_iter()
            .map(|line| format_template(&line, args))
            .collect(),
    }
}

fn format_template(template: &str, args: &[(&str, &str)]) -> String {
    let mut rendered = template.to_string();

    for (name, value) in args {
        rendered = rendered.replace(&format!("{{{name}}}"), value);
    }

    rendered
}
