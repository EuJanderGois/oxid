use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use macroquad::prelude::{
    DrawTextureParams, Image, Texture2D as MacroquadTexture2D, draw_texture_ex, vec2,
};

use crate::{i18n, renderer::color::Color};

pub struct LoadedTexture {
    pub key: String,
    pub width: f32,
    pub height: f32,
}

struct CachedTexture {
    texture: MacroquadTexture2D,
    width: f32,
    height: f32,
}

thread_local! {
    static TEXTURE_CACHE: RefCell<HashMap<String, CachedTexture>> = RefCell::new(HashMap::new());
}

fn to_mq_color(color: Color) -> macroquad::prelude::Color {
    macroquad::prelude::Color::new(color.r, color.g, color.b, color.a)
}

fn resolve_texture_path(path: &str) -> Result<PathBuf, String> {
    let trimmed = path.trim();

    if trimmed.is_empty() {
        return Err(i18n::text("renderer.texture.empty_path"));
    }

    let resolved = Path::new(trimmed).canonicalize().map_err(|err| {
        let source = err.to_string();
        i18n::text_with(
            "renderer.texture.resolve_failed",
            &[("path", trimmed), ("source", &source)],
        )
    })?;

    if !resolved.is_file() {
        let resolved_path = resolved.display().to_string();
        return Err(i18n::text_with(
            "renderer.texture.not_a_file",
            &[("path", &resolved_path)],
        ));
    }

    Ok(resolved)
}

pub fn load_texture(path: &str) -> Result<LoadedTexture, String> {
    let resolved = resolve_texture_path(path)?;
    let key = resolved.to_string_lossy().into_owned();

    if let Some(texture) = TEXTURE_CACHE.with(|cache| {
        let cache = cache.borrow();
        cache.get(&key).map(|cached| LoadedTexture {
            key: key.clone(),
            width: cached.width,
            height: cached.height,
        })
    }) {
        return Ok(texture);
    }

    let resolved_path = resolved.display().to_string();
    let bytes = fs::read(&resolved).map_err(|err| {
        let source = err.to_string();
        i18n::text_with(
            "renderer.texture.read_failed",
            &[("path", &resolved_path), ("source", &source)],
        )
    })?;
    let image = Image::from_file_with_format(&bytes, None).map_err(|err| {
        let source = err.to_string();
        i18n::text_with(
            "renderer.texture.decode_failed",
            &[("path", &resolved_path), ("source", &source)],
        )
    })?;

    let width = image.width as f32;
    let height = image.height as f32;
    let texture = MacroquadTexture2D::from_image(&image);

    TEXTURE_CACHE.with(|cache| {
        cache.borrow_mut().insert(
            key.clone(),
            CachedTexture {
                texture,
                width,
                height,
            },
        );
    });

    Ok(LoadedTexture { key, width, height })
}

pub fn draw_cached_texture(
    key: &str,
    x: f32,
    y: f32,
    width: Option<f32>,
    height: Option<f32>,
    rotation: f32,
    color: Color,
) -> Result<(), String> {
    let dest_size = match (width, height) {
        (Some(width), Some(height)) => Some(vec2(width, height)),
        (None, None) => None,
        _ => {
            return Err(i18n::text("renderer.texture.invalid_dest_size"));
        }
    };

    TEXTURE_CACHE.with(|cache| {
        let cache = cache.borrow();
        let texture = cache.get(key).ok_or_else(|| {
            i18n::text_with("renderer.texture.not_loaded", &[("texture_key", key)])
        })?;

        draw_texture_ex(
            &texture.texture,
            x,
            y,
            to_mq_color(color),
            DrawTextureParams {
                dest_size,
                rotation,
                ..Default::default()
            },
        );

        Ok(())
    })
}
