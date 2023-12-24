#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Vault {
    /// Path to root of this Vault
    pub(crate) path: std::path::PathBuf,
}

impl std::fmt::Display for Vault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

impl Vault {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }

    pub fn add_note(&self, note: crate::note::Note) -> color_eyre::Result<()> {
        note.save(&self.path)
    }
}
