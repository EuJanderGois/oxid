use std::fmt;

#[derive(Debug)]
pub enum ScriptEngineError {
    RuntimeInit(String),
    ContextInit(String),
    PluginRegister {
        plugin: &'static str,
        source: String,
    },
    StdlibRegister(String),
    EntryModuleDeclare(String),
    EntryModuleEval(String),
    MainNamespace(String),
    AppInstance(String),
    HookCompile {
        hook: &'static str,
        source: String,
    },
    HookExecution {
        hook: &'static str,
        source: String,
    },
}

impl fmt::Display for ScriptEngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScriptEngineError::RuntimeInit(msg) => 
                write!(f, "falha na inicialização do script runtime: {msg}"),
            ScriptEngineError::ContextInit(msg) => 
                write!(f, "falha ao criar contexto do script runtime: {msg}"),
            ScriptEngineError::PluginRegister { plugin, source } => 
                write!(f, "falha ao registrar plugin '{plugin}': {source}"),
            ScriptEngineError::StdlibRegister(msg) => 
                write!(f, "falha ao registrar stdlib: {msg}"),
            ScriptEngineError::EntryModuleDeclare(msg) => 
                write!(f, "falha ao declarar módulo de entrada: {msg}"),
            ScriptEngineError::EntryModuleEval(msg) => 
                write!(f, "falha ao processar módulo de entrada: {msg}"),
            ScriptEngineError::MainNamespace(msg) => 
                write!(f, "falha ao obter main namespace: {msg}"),
            ScriptEngineError::AppInstance(msg) => 
                write!(f, "falha ao criar instância da aplicação: {msg}"),
            ScriptEngineError::HookCompile { hook, source } => 
                write!(f, "falha ao compilar hook '{hook}': {source}"),
            ScriptEngineError::HookExecution { hook, source } => 
                write!(f, "falha ao executar hook '{hook}': {source}"),
        }
    }
}

impl std::error::Error for ScriptEngineError {}