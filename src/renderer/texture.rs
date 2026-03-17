use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use macroquad::prelude::{
    DrawTextureParams,
    Image,
    Texture2D as MacroquadTexture2D,
    draw_texture_ex,
    vec2,
};

use crate::renderer::color::Color;

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
        return Err("texture path não pode ser vazio.".to_string());
    }

    let resolved = Path::new(trimmed)
        .canonicalize()
        .map_err(|err| format!("falha ao localizar textura '{trimmed}': {err}"))?;

    if !resolved.is_file() {
        return Err(format!(
            "o caminho '{}' não aponta para um arquivo de textura.",
            resolved.display(),
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

    let bytes = fs::read(&resolved)
        .map_err(|err| format!("falha ao ler textura '{}': {err}", resolved.display()))?;
    let image = Image::from_file_with_format(&bytes, None)
        .map_err(|err| format!("falha ao decodificar textura '{}': {err}", resolved.display()))?;

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
            return Err("dest_size da textura deve informar largura e altura juntas.".to_string());
        }
    };

    TEXTURE_CACHE.with(|cache| {
        let cache = cache.borrow();
        let texture = cache.get(key).ok_or_else(|| {
            format!("textura '{key}' não foi carregada antes de desenhar.")
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
