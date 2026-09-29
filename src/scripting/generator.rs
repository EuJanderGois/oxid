//! TypeScript declaration generation from scripting API metadata.

use super::plugins::{
    FunctionMeta, FunctionParam, GlobalMeta, ModuleMeta, ScriptType, TypeMeta,
};

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

fn generate_global_constant_d_ts(
    name: &str,
    docs: &str,
    ty: ScriptType,
) -> String {
    let mut output = String::new();
    output.push_str(&generate_jsdoc(docs, &[]));
    output.push_str(&format!("declare const {name}: {ty};\n"));
    output
}

fn generate_global_function_d_ts(meta: &FunctionMeta) -> String {
    let (namespace, name) = meta
        .name
        .split_once('.')
        .map_or((None, meta.name), |(namespace, name)| (Some(namespace), name));

    let declaration = format!(
        "{}declare function {}({}): {};\n",
        generate_jsdoc(meta.docs, meta.params),
        name,
        generate_params(meta.params),
        meta.returns
    );

    match namespace {
        Some(namespace) => {
            let indented = declaration
                .lines()
                .map(|line| format!("  {line}\n"))
                .collect::<String>();
            format!("declare namespace {namespace} {{\n{indented}}}\n")
        }
        None => declaration,
    }
}

pub fn generate_globals_d_ts(globals: &[GlobalMeta]) -> String {
    let mut output = String::new();

    for global in globals {
        if !global.docs.is_empty() {
            output.push_str("/** ");
            output.push_str(global.docs);
            output.push_str(" */\n");
        }

        for constant in global.constants {
            output.push_str(&generate_global_constant_d_ts(
                constant.name,
                constant.docs,
                constant.ty,
            ));
            output.push('\n');
        }

        for function in global.functions {
            output.push_str(&generate_global_function_d_ts(function));
            output.push('\n');
        }
    }

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

pub fn generate_d_ts_with_globals(
    modules: &[ModuleMeta],
    globals: &[GlobalMeta],
) -> String {
    let mut output = String::from(
        "/**\n * Type definitions generated from the Oxid scripting API metadata.\n * Do not edit this file manually.\n */\n\n",
    );

    let global_output = generate_globals_d_ts(globals);
    if !global_output.is_empty() {
        output.push_str(&global_output);
        output.push('\n');
    }

    for module in modules {
        output.push_str(&generate_module_d_ts(module));
        output.push('\n');
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scripting::plugins::{
        FunctionParam, ModuleMeta, ScriptType, TypeConstructorMeta, TypeMeta,
    };

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

        let output = generate_d_ts(&[ModuleMeta {
            name: "oxid/example",
            docs: "Example module.",
            types: &[],
            functions: &FUNCTIONS,
        }]);

        assert!(output.contains("import type { Vector2D } from \"oxid/math\";"));
        assert!(output.contains("draw(position: Vector2D): void;"));
    }

    #[test]
    fn generates_global_declarations() {
        static FUNCTIONS: [FunctionMeta; 1] = [FunctionMeta {
            module: "global",
            name: "console.log",
            docs: "Writes a message.",
            returns: ScriptType::Void,
            params: &[FunctionParam {
                name: "message",
                ty: ScriptType::String,
                docs: "Message to print.",
                optional: false,
            }],
        }];
        static GLOBALS: [GlobalMeta; 1] = [GlobalMeta {
            name: "console",
            docs: "Console utilities.",
            constants: &[],
            functions: &FUNCTIONS,
        }];

        let output = generate_globals_d_ts(&GLOBALS);

        assert!(output.contains("declare namespace console"));
        assert!(output.contains("function log(message: string): void;"));
    }

    #[test]
    fn generated_api_uses_vector2d_for_coordinate_values() {
        let output = crate::scripting::generate_api_d_ts();

        assert!(output.contains("export class Vector2D"));
        assert!(!output.contains("Transform2D"));
    }

    #[test]
    fn generated_metadata_documentation_is_english() {
        let output = crate::scripting::generate_api_d_ts();

        assert!(!output.contains("Desenha"));
        assert!(!output.contains("Retorna"));
        assert!(!output.contains("Posição"));
        assert!(!output.contains("Vetor"));
        assert!(!output.contains("Espessura"));
        assert!(!output.contains("Abertura"));
        assert!(!output.contains("Tamanho da fonte"));
        assert!(!output.contains("Carregamento"));
        assert!(!output.contains("Deve ser maior"));
        assert!(output.contains("Draws"));
        assert!(output.contains("Returns"));
    }

    #[test]
    fn generated_metadata_has_no_stray_diacritics() {
        // A cheap, broader net than the specific-word checks above: none of
        // the strings that reach `oxid.d.ts` should contain characters that
        // never appear in English technical prose. This is what should have
        // caught the pt-BR strings that leaked into shipped `oxid.d.ts`
        // files before this test existed.
        let output = crate::scripting::generate_api_d_ts();

        for ch in ['ã', 'õ', 'ç', 'á', 'é', 'í', 'ó', 'ú', 'â', 'ê'] {
            assert!(
                !output.contains(ch),
                "generated oxid.d.ts contains non-English character '{ch}'; \
                 public API metadata docs must be written in English"
            );
        }
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
                super::super::plugins::TypePropertyMeta {
                    name: "value",
                    ty: ScriptType::Number,
                    docs: "Current value.",
                    readonly: false,
                },
                super::super::plugins::TypePropertyMeta {
                    name: "id",
                    ty: ScriptType::Number,
                    docs: "Identifier.",
                    readonly: true,
                },
            ],
        }];

        let output = generate_d_ts(&[ModuleMeta {
            name: "oxid/example",
            docs: "Example module.",
            types: &TYPES,
            functions: &[],
        }]);

        assert!(output.contains("constructor(value: number);"));
        assert!(output.contains("value: number;"));
        assert!(output.contains("readonly id: number;"));
    }
}
