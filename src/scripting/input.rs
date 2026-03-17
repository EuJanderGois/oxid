use std::string::String as StdString;

use macroquad::prelude::{
    KeyCode,
    MouseButton,
    is_key_down as mq_is_key_down,
    is_key_pressed as mq_is_key_pressed,
    is_key_released as mq_is_key_released,
    is_mouse_button_down as mq_is_mouse_button_down,
    is_mouse_button_pressed as mq_is_mouse_button_pressed,
    is_mouse_button_released as mq_is_mouse_button_released,
    mouse_position as mq_mouse_position,
};
use rquickjs::{
    Class,
    Ctx,
    Exception,
    Function,
    Result,
    module::{Declarations, Exports, ModuleDef},
};

use crate::scripting::{
    math::Transform2D,
    plugin::{FunctionMeta, FunctionParam, NativePlugin, ScriptType},
};

const KEY_NAME_DOCS: &str = "Nome da tecla. Ignora maiúsculas/minúsculas, espaços, '_' e '-'. Exemplos: \"A\", \"ArrowLeft\", \"Space\", \"Enter\", \"Escape\", \"LeftShift\", \"F1\".";
const MOUSE_BUTTON_DOCS: &str = "Nome do botão do mouse. Valores aceitos: \"left\", \"middle\" e \"right\".";

fn normalize_input_name(name: &str) -> StdString {
    let mut normalized = StdString::with_capacity(name.len());

    for ch in name.chars() {
        if matches!(ch, ' ' | '_' | '-') {
            continue;
        }

        normalized.push(ch.to_ascii_lowercase());
    }

    normalized
}

fn invalid_key_name(ctx: &Ctx<'_>, name: &str) -> rquickjs::Error {
    Exception::throw_type(
        ctx,
        &format!(
            "tecla inválida: '{name}'. Exemplos válidos: A, ArrowLeft, Space, Enter, Escape, LeftShift, F1."
        ),
    )
}

fn invalid_mouse_button(ctx: &Ctx<'_>, name: &str) -> rquickjs::Error {
    Exception::throw_type(
        ctx,
        &format!("botão do mouse inválido: '{name}'. Use left, middle ou right."),
    )
}

fn parse_alphanumeric_key(name: &str) -> Option<KeyCode> {
    match name {
        "a" => Some(KeyCode::A),
        "b" => Some(KeyCode::B),
        "c" => Some(KeyCode::C),
        "d" => Some(KeyCode::D),
        "e" => Some(KeyCode::E),
        "f" => Some(KeyCode::F),
        "g" => Some(KeyCode::G),
        "h" => Some(KeyCode::H),
        "i" => Some(KeyCode::I),
        "j" => Some(KeyCode::J),
        "k" => Some(KeyCode::K),
        "l" => Some(KeyCode::L),
        "m" => Some(KeyCode::M),
        "n" => Some(KeyCode::N),
        "o" => Some(KeyCode::O),
        "p" => Some(KeyCode::P),
        "q" => Some(KeyCode::Q),
        "r" => Some(KeyCode::R),
        "s" => Some(KeyCode::S),
        "t" => Some(KeyCode::T),
        "u" => Some(KeyCode::U),
        "v" => Some(KeyCode::V),
        "w" => Some(KeyCode::W),
        "x" => Some(KeyCode::X),
        "y" => Some(KeyCode::Y),
        "z" => Some(KeyCode::Z),
        "0" | "digit0" | "key0" => Some(KeyCode::Key0),
        "1" | "digit1" | "key1" => Some(KeyCode::Key1),
        "2" | "digit2" | "key2" => Some(KeyCode::Key2),
        "3" | "digit3" | "key3" => Some(KeyCode::Key3),
        "4" | "digit4" | "key4" => Some(KeyCode::Key4),
        "5" | "digit5" | "key5" => Some(KeyCode::Key5),
        "6" | "digit6" | "key6" => Some(KeyCode::Key6),
        "7" | "digit7" | "key7" => Some(KeyCode::Key7),
        "8" | "digit8" | "key8" => Some(KeyCode::Key8),
        "9" | "digit9" | "key9" => Some(KeyCode::Key9),
        _ => None,
    }
}

fn parse_function_key(name: &str) -> Option<KeyCode> {
    let number = name.strip_prefix('f')?.parse::<u8>().ok()?;

    match number {
        1 => Some(KeyCode::F1),
        2 => Some(KeyCode::F2),
        3 => Some(KeyCode::F3),
        4 => Some(KeyCode::F4),
        5 => Some(KeyCode::F5),
        6 => Some(KeyCode::F6),
        7 => Some(KeyCode::F7),
        8 => Some(KeyCode::F8),
        9 => Some(KeyCode::F9),
        10 => Some(KeyCode::F10),
        11 => Some(KeyCode::F11),
        12 => Some(KeyCode::F12),
        13 => Some(KeyCode::F13),
        14 => Some(KeyCode::F14),
        15 => Some(KeyCode::F15),
        16 => Some(KeyCode::F16),
        17 => Some(KeyCode::F17),
        18 => Some(KeyCode::F18),
        19 => Some(KeyCode::F19),
        20 => Some(KeyCode::F20),
        21 => Some(KeyCode::F21),
        22 => Some(KeyCode::F22),
        23 => Some(KeyCode::F23),
        24 => Some(KeyCode::F24),
        25 => Some(KeyCode::F25),
        _ => None,
    }
}

fn parse_numpad_key(name: &str) -> Option<KeyCode> {
    let suffix = name
        .strip_prefix("numpad")
        .or_else(|| name.strip_prefix("kp"))?;

    match suffix {
        "0" => Some(KeyCode::Kp0),
        "1" => Some(KeyCode::Kp1),
        "2" => Some(KeyCode::Kp2),
        "3" => Some(KeyCode::Kp3),
        "4" => Some(KeyCode::Kp4),
        "5" => Some(KeyCode::Kp5),
        "6" => Some(KeyCode::Kp6),
        "7" => Some(KeyCode::Kp7),
        "8" => Some(KeyCode::Kp8),
        "9" => Some(KeyCode::Kp9),
        "decimal" | "dot" => Some(KeyCode::KpDecimal),
        "divide" | "slash" => Some(KeyCode::KpDivide),
        "multiply" | "star" => Some(KeyCode::KpMultiply),
        "subtract" | "minus" => Some(KeyCode::KpSubtract),
        "add" | "plus" => Some(KeyCode::KpAdd),
        "enter" => Some(KeyCode::KpEnter),
        "equal" | "equals" => Some(KeyCode::KpEqual),
        _ => None,
    }
}

fn parse_key_code(ctx: &Ctx<'_>, name: &str) -> Result<KeyCode> {
    let normalized = normalize_input_name(name);

    if let Some(key) = parse_alphanumeric_key(&normalized)
        .or_else(|| parse_function_key(&normalized))
        .or_else(|| parse_numpad_key(&normalized))
    {
        return Ok(key);
    }

    match normalized.as_str() {
        "space" => Ok(KeyCode::Space),
        "apostrophe" | "quote" => Ok(KeyCode::Apostrophe),
        "comma" => Ok(KeyCode::Comma),
        "minus" | "dash" => Ok(KeyCode::Minus),
        "period" | "dot" => Ok(KeyCode::Period),
        "slash" | "forwardslash" => Ok(KeyCode::Slash),
        "semicolon" => Ok(KeyCode::Semicolon),
        "equal" | "equals" => Ok(KeyCode::Equal),
        "leftbracket" | "lbracket" => Ok(KeyCode::LeftBracket),
        "backslash" => Ok(KeyCode::Backslash),
        "rightbracket" | "rbracket" => Ok(KeyCode::RightBracket),
        "graveaccent" | "backquote" | "tilde" => Ok(KeyCode::GraveAccent),
        "world1" => Ok(KeyCode::World1),
        "world2" => Ok(KeyCode::World2),
        "escape" | "esc" => Ok(KeyCode::Escape),
        "enter" | "return" => Ok(KeyCode::Enter),
        "tab" => Ok(KeyCode::Tab),
        "backspace" => Ok(KeyCode::Backspace),
        "insert" => Ok(KeyCode::Insert),
        "delete" | "del" => Ok(KeyCode::Delete),
        "arrowright" | "right" => Ok(KeyCode::Right),
        "arrowleft" | "left" => Ok(KeyCode::Left),
        "arrowdown" | "down" => Ok(KeyCode::Down),
        "arrowup" | "up" => Ok(KeyCode::Up),
        "pageup" => Ok(KeyCode::PageUp),
        "pagedown" => Ok(KeyCode::PageDown),
        "home" => Ok(KeyCode::Home),
        "end" => Ok(KeyCode::End),
        "capslock" => Ok(KeyCode::CapsLock),
        "scrolllock" => Ok(KeyCode::ScrollLock),
        "numlock" => Ok(KeyCode::NumLock),
        "printscreen" => Ok(KeyCode::PrintScreen),
        "pause" => Ok(KeyCode::Pause),
        "leftshift" | "shift" => Ok(KeyCode::LeftShift),
        "leftcontrol" | "control" | "ctrl" => Ok(KeyCode::LeftControl),
        "leftalt" | "alt" => Ok(KeyCode::LeftAlt),
        "leftsuper" | "super" | "meta" | "command" | "cmd" => Ok(KeyCode::LeftSuper),
        "rightshift" => Ok(KeyCode::RightShift),
        "rightcontrol" => Ok(KeyCode::RightControl),
        "rightalt" => Ok(KeyCode::RightAlt),
        "rightsuper" => Ok(KeyCode::RightSuper),
        "menu" => Ok(KeyCode::Menu),
        "back" => Ok(KeyCode::Back),
        _ => Err(invalid_key_name(ctx, name)),
    }
}

fn parse_mouse_button(ctx: &Ctx<'_>, name: &str) -> Result<MouseButton> {
    match normalize_input_name(name).as_str() {
        "left" | "primary" => Ok(MouseButton::Left),
        "middle" | "center" => Ok(MouseButton::Middle),
        "right" | "secondary" => Ok(MouseButton::Right),
        _ => Err(invalid_mouse_button(ctx, name)),
    }
}

fn is_key_down(ctx: Ctx<'_>, key: StdString) -> Result<bool> {
    Ok(mq_is_key_down(parse_key_code(&ctx, &key)?))
}

fn is_key_pressed(ctx: Ctx<'_>, key: StdString) -> Result<bool> {
    Ok(mq_is_key_pressed(parse_key_code(&ctx, &key)?))
}

fn is_key_released(ctx: Ctx<'_>, key: StdString) -> Result<bool> {
    Ok(mq_is_key_released(parse_key_code(&ctx, &key)?))
}

fn mouse_position<'js>(ctx: Ctx<'js>) -> Result<Class<'js, Transform2D>> {
    let (x, y) = mq_mouse_position();
    Class::instance(ctx, Transform2D::new(x, y))
}

fn is_mouse_button_down(ctx: Ctx<'_>, button: StdString) -> Result<bool> {
    Ok(mq_is_mouse_button_down(parse_mouse_button(&ctx, &button)?))
}

fn is_mouse_button_pressed(ctx: Ctx<'_>, button: StdString) -> Result<bool> {
    Ok(mq_is_mouse_button_pressed(parse_mouse_button(&ctx, &button)?))
}

fn is_mouse_button_released(ctx: Ctx<'_>, button: StdString) -> Result<bool> {
    Ok(mq_is_mouse_button_released(parse_mouse_button(&ctx, &button)?))
}

/// gerencia os métodos e módulos de input.
pub struct InputPlugin;

impl ModuleDef for InputPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("isKeyDown")?;
        declare.declare("isKeyPressed")?;
        declare.declare("isKeyReleased")?;
        declare.declare("mousePosition")?;
        declare.declare("isMouseButtonDown")?;
        declare.declare("isMouseButtonPressed")?;
        declare.declare("isMouseButtonReleased")?;
        Ok(())
    }

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export("isKeyDown", Function::new(ctx.clone(), is_key_down)?)?;
        exports.export("isKeyPressed", Function::new(ctx.clone(), is_key_pressed)?)?;
        exports.export("isKeyReleased", Function::new(ctx.clone(), is_key_released)?)?;
        exports.export("mousePosition", Function::new(ctx.clone(), mouse_position)?)?;
        exports.export(
            "isMouseButtonDown",
            Function::new(ctx.clone(), is_mouse_button_down)?,
        )?;
        exports.export(
            "isMouseButtonPressed",
            Function::new(ctx.clone(), is_mouse_button_pressed)?,
        )?;
        exports.export(
            "isMouseButtonReleased",
            Function::new(ctx.clone(), is_mouse_button_released)?,
        )?;
        Ok(())
    }
}

impl NativePlugin for InputPlugin {
    const NAME: &'static str = "oxid/input";

    fn functions() -> &'static [FunctionMeta] {
        &[
            FunctionMeta {
                module: "oxid/input",
                name: "isKeyDown",
                docs: "Retorna true enquanto a tecla estiver pressionada.",
                returns: ScriptType::Boolean,
                params: &[FunctionParam {
                    name: "key",
                    ty: ScriptType::String,
                    docs: KEY_NAME_DOCS,
                    optional: false,
                }],
            },
            FunctionMeta {
                module: "oxid/input",
                name: "isKeyPressed",
                docs: "Retorna true apenas no frame em que a tecla foi pressionada.",
                returns: ScriptType::Boolean,
                params: &[FunctionParam {
                    name: "key",
                    ty: ScriptType::String,
                    docs: KEY_NAME_DOCS,
                    optional: false,
                }],
            },
            FunctionMeta {
                module: "oxid/input",
                name: "isKeyReleased",
                docs: "Retorna true apenas no frame em que a tecla foi solta.",
                returns: ScriptType::Boolean,
                params: &[FunctionParam {
                    name: "key",
                    ty: ScriptType::String,
                    docs: KEY_NAME_DOCS,
                    optional: false,
                }],
            },
            FunctionMeta {
                module: "oxid/input",
                name: "mousePosition",
                docs: "Retorna a posição atual do mouse em coordenadas de tela.",
                returns: ScriptType::Custom("Transform2D"),
                params: &[],
            },
            FunctionMeta {
                module: "oxid/input",
                name: "isMouseButtonDown",
                docs: "Retorna true enquanto o botão do mouse estiver pressionado.",
                returns: ScriptType::Boolean,
                params: &[FunctionParam {
                    name: "button",
                    ty: ScriptType::String,
                    docs: MOUSE_BUTTON_DOCS,
                    optional: false,
                }],
            },
            FunctionMeta {
                module: "oxid/input",
                name: "isMouseButtonPressed",
                docs: "Retorna true apenas no frame em que o botão do mouse foi pressionado.",
                returns: ScriptType::Boolean,
                params: &[FunctionParam {
                    name: "button",
                    ty: ScriptType::String,
                    docs: MOUSE_BUTTON_DOCS,
                    optional: false,
                }],
            },
            FunctionMeta {
                module: "oxid/input",
                name: "isMouseButtonReleased",
                docs: "Retorna true apenas no frame em que o botão do mouse foi solto.",
                returns: ScriptType::Boolean,
                params: &[FunctionParam {
                    name: "button",
                    ty: ScriptType::String,
                    docs: MOUSE_BUTTON_DOCS,
                    optional: false,
                }],
            },
        ]
    }
}
