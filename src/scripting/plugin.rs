//! gerencia plugins nativos
//!
//! plugins nativos são módulos escritos em rust que expõe funcionalidades
//! a script API

use core::fmt::{self};

use rquickjs::{Ctx, Result, module::ModuleDef};

///
/// parametro de função
///
pub struct FunctionParam {
    pub name: &'static str,
    pub ty: ScriptType,
    pub docs: &'static str,
    pub optional: bool,
}

///
/// meta dados de uma função.
///
/// usado pra construção de documentação e arquivos de
/// definição (.d.ts).
///
pub struct FunctionMeta {
    pub module: &'static str,
    pub name: &'static str,
    pub docs: &'static str,
    pub returns: ScriptType,
    pub params: &'static [FunctionParam],
}

pub trait NativeFunction {
    fn meta() -> &'static FunctionMeta;
}

///
/// variantes de tipos possíveis
///
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptType {
    Number,
    String,
    Boolean,
    Void,
    Any,
    Custom(&'static str),
}

impl fmt::Display for ScriptType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        match self {
            ScriptType::Number => write!(f, "number"),
            ScriptType::String => write!(f, "string"),
            ScriptType::Boolean => write!(f, "boolean"),
            ScriptType::Void => write!(f, "void"),
            ScriptType::Any => write!(f, "any"),
            ScriptType::Custom(text) => write!(f, "{text}"),
        }
    } // formata para string
}

///
/// gera definições de função baseado em meta dados
///
/// TODO: generate_module_d_ts
///
pub fn generate_func_d_ts(meta: &FunctionMeta) -> String {
    let mut jsdoc_params = String::new();
    let mut signature_params = String::new();

    for (index, param) in meta.params.iter().enumerate() {
        jsdoc_params.push_str(&format!(" * @param {} {}\n", param.name, param.docs));

        if index > 0 {
            signature_params.push_str(", ");
        }

        if param.optional {
            signature_params.push_str(&format!("{}?: {}", param.name, param.ty));
        } else {
            signature_params.push_str(&format!("{}: {}", param.name, param.ty));
        }
    }

    format!(
        "/**\n * {}\n{} */\nexport function {}({}): {};\n",
        meta.docs, jsdoc_params, meta.name, signature_params, meta.returns
    )
}

///
/// gerencia e registra plugins nativos
///
/// exige que o plugin seja uma estrutura de tamanho conhecido.
/// pode opicionalmente registrar meta dados para geração de definição.
///
pub trait NativePlugin: ModuleDef + Sized {
    const NAME: &'static str;

    fn functions() -> &'static [FunctionMeta] {
        &[]
    }

    fn register<'js>(ctx: &Ctx<'js>) -> Result<()> {
        // Usa o '?' para propagar possíveis erros e descarta o módulo retornado
        rquickjs::Module::declare_def::<Self, _>(ctx.clone(), Self::NAME)?;

        // Retorna sucesso vazio, conforme a assinatura da função exige
        Ok(())
    }
}
