pub mod note;
mod vault;
use colored::Colorize;

static STATE_FILENAME: &'static str = "state.json";

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct RoboNote {
    /// Currently active Vault
    pub active_vault: Option<usize>,

    /// All vaults that are registered
    vaults: Vec<vault::Vault>,

    /// Stores state files, some of information is stored here, but other are in .vault.json file
    /// inside vaults.
    #[serde(skip)]
    state_folder: std::path::PathBuf,
}

// impl Drop for RoboNote {
//     #[tracing::instrument]
//     fn drop(&mut self) {
//         let _ = self.save();
//     }
// }

impl RoboNote {
    pub fn new(state_folder: &std::path::PathBuf) -> Self {
        tracing::info!("Creating new RoboNote");
        Self {
            active_vault: None,
            vaults: Vec::new(),
            state_folder: state_folder.clone(),
        }
    }

    /// Try to load from state, if returns error state file exist on path passed to this function
    #[tracing::instrument]
    pub fn from_state(state_folder: &std::path::PathBuf) -> color_eyre::Result<Self> {
        let mut state: Self =
            serde_json::from_str(&std::fs::read_to_string(state_folder.join(STATE_FILENAME))?)?;
        tracing::info!("Trying to create RoboNote from state file");

        state.state_folder = state_folder.to_path_buf();

        Ok(state)
    }

    /// Creates new vault and sets active vault to that
    #[tracing::instrument]
    pub fn create_vault(&mut self, path: std::path::PathBuf) -> color_eyre::Result<()> {
        tracing::info!("Creating new vault");
        let vault = vault::Vault::new(path);

        self.vaults.push(vault);
        self.active_vault = Some(self.vaults.len() - 1);

        Ok(())
    }

    /// Gets current vault, if no current then returns first one
    #[tracing::instrument]
    pub fn get_current(&mut self) -> Option<&mut vault::Vault> {
        tracing::info!("Getting current Vault to modify");
        let len = self.vaults.len();

        self.vaults.get_mut(self.active_vault.unwrap_or({
            tracing::warn!("No current Vault active, using first one");

            if len > 0 {
                0
            } else {
                return None;
            }
        }))
    }

    pub fn list_vaults(&self) {
        for (index, vault) in self.vaults.iter().enumerate() {
            let active = self.active_vault.unwrap_or(0);

            if active == index {
                let path = format!("{}", vault.path.display());
                println!("\u{1F4D6} - {}", path.green());
            } else {
                println!("\u{1F4D4} - {}", vault.path.display());
            }
        }
    }

    #[tracing::instrument]
    pub fn save(&self) -> color_eyre::Result<()> {
        if !self.state_folder.exists() {
            std::fs::create_dir_all(&self.state_folder).unwrap();
        }

        let json = serde_json::to_string(self).expect("Failed to convert RoboNote to json");
        let path = self.state_folder.join(STATE_FILENAME);
        tracing::debug!("Saving state file to {:?}", path);
        std::fs::write(path, json).expect("Failes to write state to file");

        Ok(())
    }
}

#[cfg(test)]
mod robo_notes_tests {
    use pretty_assertions::assert_eq;

    use super::*;
    fn pre_create_state() -> color_eyre::Result<RoboNote> {
        let path = std::env::current_dir()?.join("target/test/state");
        let mut rn = RoboNote::new(&path);
        rn.create_vault(std::env::current_dir()?.join("target/test/vault1"))?;
        Ok(rn)
    }

    #[tracing_test::traced_test]
    #[test]
    fn load_state() -> color_eyre::Result<()> {
        let rn = pre_create_state()?;
        rn.save().unwrap();

        let path = std::env::current_dir()?.join("target/test/state");
        let mut rn = RoboNote::from_state(&path)?;
        assert_eq!(rn.get_current().is_some(), true);
        assert_eq!(rn.vaults.len(), 1, "Vaults len should be one");

        Ok(())
    }
}
