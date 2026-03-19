use console::{Style, measure_text_width};

use crate::i18n::message::ResolvedEntry;

pub(crate) fn render_styled_block(prefix: &str, entry: ResolvedEntry) -> String {
    let icon_plain = icon_text(entry.icon.as_deref(), entry.tone.as_deref());
    let accent_style = accent_style(entry.color.as_deref(), entry.tone.as_deref());
    let text_style = text_style(entry.color.as_deref(), entry.tone.as_deref());

    let lines = if entry.lines.is_empty() {
        vec![String::new()]
    } else {
        entry.lines
    };

    let prefix_plain = if prefix.is_empty() {
        String::new()
    } else {
        format!("{prefix} ")
    };
    let first_indent = format!("{prefix_plain}{icon_plain}");
    let continuation = " ".repeat(measure_text_width(&first_indent));

    let prefix_style = Style::new().bold().dim();

    let mut rendered = Vec::with_capacity(lines.len());
    rendered.push(format!(
        "{}{}{}",
        style_segment(&prefix_style, &prefix_plain),
        style_segment(&accent_style, &icon_plain),
        style_segment(&text_style, &lines[0]),
    ));

    for line in lines.into_iter().skip(1) {
        rendered.push(format!(
            "{}{}",
            continuation,
            style_segment(&text_style, &line),
        ));
    }

    rendered.join("\n")
}

fn accent_style(color: Option<&str>, tone: Option<&str>) -> Style {
    base_style(color, tone).bold()
}

fn text_style(color: Option<&str>, tone: Option<&str>) -> Style {
    base_style(color, tone)
}

fn icon_text(icon: Option<&str>, tone: Option<&str>) -> String {
    let icon = icon.or_else(|| default_icon(tone)).unwrap_or_default();

    if icon.is_empty() {
        String::new()
    } else {
        format!("{icon} ")
    }
}

fn base_style(color: Option<&str>, tone: Option<&str>) -> Style {
    let key = color
        .map(normalize_token)
        .or_else(|| tone.map(normalize_token));

    match key.as_deref() {
        Some("error") | Some("red") => Style::new().red(),
        Some("warning") | Some("yellow") => Style::new().yellow(),
        Some("success") | Some("green") => Style::new().green(),
        Some("info") | Some("cyan") => Style::new().cyan(),
        Some("muted") => Style::new().dim(),
        Some("blue") => Style::new().blue(),
        Some("magenta") => Style::new().magenta(),
        Some("white") => Style::new().white(),
        Some("black") => Style::new().black(),
        _ => Style::new(),
    }
}

fn default_icon(tone: Option<&str>) -> Option<&'static str> {
    match tone.map(normalize_token).as_deref() {
        Some("error") => Some("✖"),
        Some("warning") => Some("⚠"),
        Some("success") => Some("✔"),
        Some("info") => Some("ℹ"),
        Some("muted") => Some("•"),
        _ => None,
    }
}

fn normalize_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

fn style_segment(style: &Style, text: &str) -> String {
    if text.is_empty() {
        String::new()
    } else {
        style.apply_to(text).to_string()
    }
}
