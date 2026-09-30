#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }
}

// Standard palette used by the rendering and scripting APIs.
pub const LIGHTGRAY: Color = Color::new(200.0 / 255.0, 200.0 / 255.0, 200.0 / 255.0, 1.0);
pub const GRAY: Color = Color::new(130.0 / 255.0, 130.0 / 255.0, 130.0 / 255.0, 1.0);
pub const DARKGRAY: Color = Color::new(80.0 / 255.0, 80.0 / 255.0, 80.0 / 255.0, 1.0);
pub const YELLOW: Color = Color::new(253.0 / 255.0, 249.0 / 255.0, 0.0, 1.0);
pub const GOLD: Color = Color::new(255.0 / 255.0, 203.0 / 255.0, 0.0, 1.0);
pub const ORANGE: Color = Color::new(255.0 / 255.0, 161.0 / 255.0, 0.0, 1.0);
pub const PINK: Color = Color::new(255.0 / 255.0, 109.0 / 255.0, 194.0 / 255.0, 1.0);
pub const RED: Color = Color::new(230.0 / 255.0, 41.0 / 255.0, 55.0 / 255.0, 1.0);
pub const MAROON: Color = Color::new(190.0 / 255.0, 33.0 / 255.0, 55.0 / 255.0, 1.0);
pub const GREEN: Color = Color::new(0.0, 228.0 / 255.0, 48.0 / 255.0, 1.0);
pub const LIME: Color = Color::new(0.0, 158.0 / 255.0, 47.0 / 255.0, 1.0);
pub const DARKGREEN: Color = Color::new(0.0, 117.0 / 255.0, 44.0 / 255.0, 1.0);
pub const SKYBLUE: Color = Color::new(102.0 / 255.0, 191.0 / 255.0, 1.0, 1.0);
pub const BLUE: Color = Color::new(0.0, 121.0 / 255.0, 241.0 / 255.0, 1.0);
pub const DARKBLUE: Color = Color::new(0.0, 82.0 / 255.0, 172.0 / 255.0, 1.0);
pub const PURPLE: Color = Color::new(200.0 / 255.0, 122.0 / 255.0, 1.0, 1.0);
pub const VIOLET: Color = Color::new(135.0 / 255.0, 60.0 / 255.0, 190.0 / 255.0, 1.0);
pub const DARKPURPLE: Color = Color::new(112.0 / 255.0, 31.0 / 255.0, 126.0 / 255.0, 1.0);
pub const BEIGE: Color = Color::new(211.0 / 255.0, 176.0 / 255.0, 131.0 / 255.0, 1.0);
pub const BROWN: Color = Color::new(127.0 / 255.0, 106.0 / 255.0, 79.0 / 255.0, 1.0);
pub const DARKBROWN: Color = Color::new(76.0 / 255.0, 63.0 / 255.0, 47.0 / 255.0, 1.0);
pub const WHITE: Color = Color::new(1.0, 1.0, 1.0, 1.0);
pub const BLACK: Color = Color::new(0.0, 0.0, 0.0, 1.0);
pub const BLANK: Color = Color::new(0.0, 0.0, 0.0, 0.0);
pub const MAGENTA: Color = Color::new(1.0, 0.0, 1.0, 1.0);
