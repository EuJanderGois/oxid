//! Scripting plugin metadata and registration helpers.

pub mod color;
pub mod core;
pub mod input;
pub mod math;
pub mod shapes;
pub mod text;
pub mod texture;

pub mod registry;

use ::std::fmt;

use rquickjs::{Ctx, Result, module::ModuleDef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionParam {
    pub name: &'static str,
    pub ty: ScriptType,
    pub docs: &'static str,
    pub optional: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionMeta {
    pub module: &'static str,
    pub name: &'static str,
    pub docs: &'static str,
    pub returns: ScriptType,
    pub params: &'static [FunctionParam],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypePropertyMeta {
    pub name: &'static str,
    pub ty: ScriptType,
    pub docs: &'static str,
    pub readonly: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeConstructorMeta {
    pub params: &'static [FunctionParam],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TypeMeta {
    pub module: &'static str,
    pub name: &'static str,
    pub docs: &'static str,
    pub constructors: &'static [TypeConstructorMeta],
    pub properties: &'static [TypePropertyMeta],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ModuleMeta {
    pub name: &'static str,
    pub docs: &'static str,
    pub types: &'static [TypeMeta],
    pub functions: &'static [FunctionMeta],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptType {
    Number,
    String,
    Boolean,
    Void,
    Any,
    Custom(&'static str, &'static str),
}

impl fmt::Display for ScriptType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptType::Number => write!(f, "number"),
            ScriptType::String => write!(f, "string"),
            ScriptType::Boolean => write!(f, "boolean"),
            ScriptType::Void => write!(f, "void"),
            ScriptType::Any => write!(f, "any"),
            ScriptType::Custom(_, name) => write!(f, "{name}"),
        }
    }
}

pub trait NativeFunction {
    fn meta() -> &'static FunctionMeta;
}

pub trait ScriptPlugin {
    const NAME: &'static str;

    fn functions() -> &'static [FunctionMeta] {
        &[]
    }

    fn types() -> &'static [TypeMeta] {
        &[]
    }

    fn docs() -> &'static str {
        ""
    }

    fn metadata() -> ModuleMeta {
        ModuleMeta {
            name: Self::NAME,
            docs: Self::docs(),
            types: Self::types(),
            functions: Self::functions(),
        }
    }

    /// Registers the plugin in the current QuickJS context.
    fn register<'js>(ctx: &Ctx<'js>) -> Result<()>;
}

pub fn register_module_def<'js, D>(ctx: &Ctx<'js>, name: &str) -> Result<()>
where
    D: ModuleDef + Sized,
{
    rquickjs::Module::declare_def::<D, _>(ctx.clone(), name)?;
    Ok(())
}

pub struct PluginRegistration {
    pub name: &'static str,
    pub metadata: fn() -> ModuleMeta,
    pub register: for<'js> fn(&Ctx<'js>) -> Result<()>,
}

#[cfg(test)]
mod tests {
    use super::super::generator::generate_d_ts;
    use super::*;

    #[test]
    fn generates_cross_module_type_imports() {
        static FUNCTIONS: [FunctionMeta; 1] = [FunctionMeta {
            module: "oxid/example",
            name: "draw",
            docs: "Draws something.",
            returns: ScriptType::Void,
            params: &[FunctionParam {
                name: "position",
                ty: ScriptType::Custom("oxid/math", "Vector2D"),
                docs: "Draw position.",
                optional: false,
            }],
        }];

        let module = ModuleMeta {
            name: "oxid/example",
            docs: "Example module.",
            types: &[],
            functions: &FUNCTIONS,
        };

        let output = generate_d_ts(&[module]);

        assert!(output.contains("import type { Vector2D } from \"oxid/math\";"));
        assert!(output.contains("draw(position: Vector2D): void;"));
    }

    #[test]
    fn generated_api_uses_vector2d_for_coordinate_values() {
        let output = crate::scripting::generate_api_d_ts();

        assert!(output.contains("export class Vector2D"));
        assert!(!output.contains("Transform2D"));
    }

    #[test]
    fn generates_mutable_and_readonly_properties() {
        static TYPES: [TypeMeta; 1] = [TypeMeta {
            module: "oxid/example",
            name: "Example",
            docs: "Example type.",
            constructors: &[TypeConstructorMeta {
                params: &[FunctionParam {
                    name: "value",
                    ty: ScriptType::Number,
                    docs: "Initial value.",
                    optional: false,
                }],
            }],
            properties: &[
                TypePropertyMeta {
                    name: "value",
                    ty: ScriptType::Number,
                    docs: "Current value.",
                    readonly: false,
                },
                TypePropertyMeta {
                    name: "id",
                    ty: ScriptType::Number,
                    docs: "Identifier.",
                    readonly: true,
                },
            ],
        }];

        let module = ModuleMeta {
            name: "oxid/example",
            docs: "Example module.",
            types: &TYPES,
            functions: &[],
        };

        let output = generate_d_ts(&[module]);

        assert!(output.contains("constructor(value: number);"));
        assert!(output.contains("value: number;"));
        assert!(output.contains("readonly id: number;"));
    }
}
