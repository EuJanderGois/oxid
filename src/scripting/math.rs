use rquickjs::module::{Declarations, Exports, ModuleDef};
use rquickjs::{Class, Ctx, JsLifetime, Result, class::Trace};

use crate::scripting::plugin::{
    FunctionParam, NativePlugin, ScriptType, TypeConstructorMeta, TypeMeta, TypePropertyMeta,
};

///
/// vetor de x e y.
///
#[rquickjs::class]
#[derive(Clone, Trace, JsLifetime)]
pub struct Vector2D {
    #[qjs(get, set)]
    pub x: f32,
    #[qjs(get, set)]
    pub y: f32,
}

#[rquickjs::methods]
impl Vector2D {
    #[qjs(constructor)]
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

///
/// gerencia os métodos e módulos de matemática.
///
pub struct MathPlugin;

impl ModuleDef for MathPlugin {
    fn declare<'js>(declare: &Declarations<'js>) -> Result<()> {
        declare.declare("Vector2D")?;
        Ok(())
    } // declara ao script

    fn evaluate<'js>(ctx: &Ctx<'js>, exports: &Exports<'js>) -> Result<()> {
        exports.export("Vector2D", Class::<Vector2D>::create_constructor(ctx)?)?;
        Ok(())
    } // exporta ao script
}

impl NativePlugin for MathPlugin {
    const NAME: &'static str = "oxid/math";

    fn docs() -> &'static str {
        "Tipos e utilitários matemáticos 2D."
    }

    fn types() -> &'static [TypeMeta] {
        static TYPES: [TypeMeta; 1] = [TypeMeta {
            module: "oxid/math",
            name: "Vector2D",
            docs: "Vetor 2D mutável com componentes x e y.",
            constructors: &[TypeConstructorMeta {
                params: &[
                    FunctionParam {
                        name: "x",
                        ty: ScriptType::Number,
                        docs: "Componente horizontal.",
                        optional: false,
                    },
                    FunctionParam {
                        name: "y",
                        ty: ScriptType::Number,
                        docs: "Componente vertical.",
                        optional: false,
                    },
                ],
            }],
            properties: &[
                TypePropertyMeta {
                    name: "x",
                    ty: ScriptType::Number,
                    docs: "Componente horizontal.",
                    readonly: false,
                },
                TypePropertyMeta {
                    name: "y",
                    ty: ScriptType::Number,
                    docs: "Componente vertical.",
                    readonly: false,
                },
            ],
        }];
        &TYPES
    }
}
