use clap::Parser;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod cli;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let parsed = cli::Cli::parse();

    let state_path = get_state_path()?;

    let mut notebook = match robo_notes::RoboNote::from_state(&state_path) {
        Ok(state) => state,
        Err(_e) => {
            tracing::warn!("No state in state path, creating new");
            robo_notes::RoboNote::new(&state_path)
        }
    };

    match parsed.command {
        cli::Commands::Add {
            name,
            content,
            tags,
            edit,
        } => {
            let Some(current) = notebook.get_current() else {
                tracing::warn!("No vaults in state");
                return Err(color_eyre::eyre::eyre!("No vaults"));
            };

            let mut note = robo_notes::note::Note::new(&name, &content, &tags);

            if content.is_none() || edit {
                let editor = std::env::var("EDITOR").unwrap_or({
                    tracing::warn!("No EDITOR env var, using vi");
                    "vi".to_string()
                });
                let tmp_path = note.get_tmp_path()?;
                std::process::Command::new(editor)
                    .arg(&tmp_path)
                    .spawn()?
                    .wait()?;

                let new_content = std::fs::read_to_string(&tmp_path)?;
                note.content = Some(new_content);
            }

            if note.content.is_none() {
                tracing::info!("Note without content, skipping");
                return Ok(());
            }

            current.add_note(note)?;
        }
        cli::Commands::Vault(command) => match command {
            cli::VaultCommands::Init => {
                let path = std::env::current_dir()?;
                notebook.create_vault(path)?;
                notebook.save()?;
            }
            cli::VaultCommands::List => {
                notebook.list_vaults();
            }
        },
    }

    Ok(())
}

fn get_state_path() -> color_eyre::Result<std::path::PathBuf> {
    let xdg_state_path = match std::env::var("XDG_STATE_HOME") {
        Ok(path) => std::path::PathBuf::from(path),
        Err(e) => {
            tracing::warn!("XDG_STATE_HOME not set, {e}");
            tracing::info!("Trying to create path");
            let home: std::path::PathBuf = std::env::var("HOME").map(std::path::PathBuf::from)?;
            home.join(".local").join("state")
        }
    };

    Ok(xdg_state_path.join(env!("CARGO_PKG_NAME")))
}
