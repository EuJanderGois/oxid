mod args;
mod new;
mod templates;

use clap::Parser;
use std::{env, process};

use args::{Cli, Commands};

pub fn run() {
    let cli = Cli::parse();
    let active_locale = configure_locale(&cli);

    let result = match cli.command {
        Commands::New { project_name } => new::create_project(&project_name, &active_locale),
        Commands::Run => run_project(),
    };

    if let Err(err) = result {
        eprintln!("{err}");
        process::exit(1);
    }
}

fn run_project() -> Result<(), String> {
    let project = crate::runtime::load_project_from_current_dir()?;
    crate::runtime::launch(project);
    Ok(())
}

fn configure_locale(cli: &Cli) -> String {
    let env_lang = env::var("OXID_LANG").ok();
    let project_lang = match (&cli.command, cli.lang.as_ref(), env_lang.as_ref()) {
        (Commands::Run, None, None) => crate::runtime::detect_project_locale_from_current_dir(),
        _ => None,
    };

    let preferred_locale = cli
        .lang
        .as_deref()
        .or(env_lang.as_deref())
        .or(project_lang.as_deref())
        .unwrap_or(oxid::i18n::DEFAULT_LOCALE);

    oxid::i18n::set_locale(preferred_locale)
}
