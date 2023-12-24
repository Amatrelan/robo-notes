#[derive(Debug, clap::Parser)]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Commands {
    /// Add new note to NoteVault
    Add {
        /// Name of the note file
        #[arg(short, long)]
        name: String,

        /// Content of note file
        ///
        /// If content is not provided then this will open $EDITOR
        #[arg(short, long)]
        content: Option<String>,

        /// Add tags to note that is created
        #[arg(short, long = "tag")]
        tags: Option<Vec<String>>,

        /// Open in editor after automatically.
        ///
        /// This doens't do anything if context wasn't given because then always editor is opened
        #[arg(short, long)]
        edit: bool,
    },

    /// Commands to manage note Vault
    #[clap(subcommand)]
    Vault(VaultCommands),
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum VaultCommands {
    Init,
    List,
}
