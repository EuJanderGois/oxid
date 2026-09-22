//! Native scripting API metadata and registration helpers.

use core::fmt;

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

pub trait NativeFunction {
    fn meta() -> &'static FunctionMeta;
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

fn type_imports(
    ty: ScriptType,
    current_module: &str,
    imports: &mut Vec<(&'static str, &'static str)>,
) {
    if let ScriptType::Custom(module, name) = ty {
        if module != current_module && !imports.contains(&(module, name)) {
            imports.push((module, name));
        }
    }
}

fn function_imports(meta: &FunctionMeta, imports: &mut Vec<(&'static str, &'static str)>) {
    type_imports(meta.returns, meta.module, imports);
    for param in meta.params {
        type_imports(param.ty, meta.module, imports);
    }
}

fn type_definition_imports(meta: &TypeMeta, imports: &mut Vec<(&'static str, &'static str)>) {
    for constructor in meta.constructors {
        for param in constructor.params {
            type_imports(param.ty, meta.module, imports);
        }
    }

    for property in meta.properties {
        type_imports(property.ty, meta.module, imports);
    }
}

fn generate_params(params: &[FunctionParam]) -> String {
    params
        .iter()
        .map(|param| {
            if param.optional {
                format!("{}?: {}", param.name, param.ty)
            } else {
                format!("{}: {}", param.name, param.ty)
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn generate_jsdoc(docs: &str, params: &[FunctionParam]) -> String {
    let mut output = String::from("/**\n");
    if !docs.is_empty() {
        output.push_str(" * ");
        output.push_str(docs);
        output.push('\n');
    }

    for param in params {
        output.push_str(" * @param ");
        output.push_str(param.name);
        output.push(' ');
        output.push_str(param.docs);
        output.push('\n');
    }

    output.push_str(" */\n");
    output
}

pub fn generate_func_d_ts(meta: &FunctionMeta) -> String {
    format!(
        "{}export function {}({}): {};\n",
        generate_jsdoc(meta.docs, meta.params),
        meta.name,
        generate_params(meta.params),
        meta.returns
    )
}

fn generate_type_d_ts(meta: &TypeMeta) -> String {
    let mut output = String::new();
    output.push_str(&generate_jsdoc(meta.docs, &[]));
    output.push_str(&format!("export class {} {{\n", meta.name));

    for constructor in meta.constructors {
        output.push_str("  constructor(");
        output.push_str(&generate_params(constructor.params));
        output.push_str(");\n");
    }

    for property in meta.properties {
        output.push_str("  ");
        if property.readonly {
            output.push_str("readonly ");
        }
        output.push_str(property.name);
        output.push_str(": ");
        output.push_str(&property.ty.to_string());
        output.push_str(";\n");
    }

    output.push_str("}\n");
    output
}

pub fn generate_module_d_ts(meta: &ModuleMeta) -> String {
    let mut imports = Vec::new();
    for ty in meta.types {
        type_definition_imports(ty, &mut imports);
    }
    for function in meta.functions {
        function_imports(function, &mut imports);
    }

    let mut output = String::new();
    output.push_str(&format!("declare module \"{}\" {{\n", meta.name));

    if !meta.docs.is_empty() {
        output.push_str("  /** ");
        output.push_str(meta.docs);
        output.push_str(" */\n");
    }

    let has_imports = !imports.is_empty();
    for (module, name) in imports {
        output.push_str(&format!(
            "  import type {{ {} }} from \"{}\";\n",
            name, module
        ));
    }
    if !meta.types.is_empty() && has_imports {
        output.push('\n');
    }

    for ty in meta.types {
        for line in generate_type_d_ts(ty).lines() {
            output.push_str("  ");
            output.push_str(line);
            output.push('\n');
        }
        output.push('\n');
    }

    for function in meta.functions {
        for line in generate_func_d_ts(function).lines() {
            output.push_str("  ");
            output.push_str(line);
            output.push('\n');
        }
        output.push('\n');
    }

    output.push_str("}\n");
    output
}

pub fn generate_d_ts(modules: &[ModuleMeta]) -> String {
    let mut output = String::from(
        "/**\n * Type definitions generated from the Oxid scripting API metadata.\n * Do not edit this file manually.\n */\n\n",
    );

    for module in modules {
        output.push_str(&generate_module_d_ts(module));
        output.push('\n');
    }

    output
}

pub trait NativePlugin: ModuleDef + Sized {
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

    fn register<'js>(ctx: &Ctx<'js>) -> Result<()> {
        rquickjs::Module::declare_def::<Self, _>(ctx.clone(), Self::NAME)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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
