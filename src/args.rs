use clap::{Parser, Subcommand};

#[derive(Clone, Parser, Debug)]
#[command(author, version, about, long_about = None, /* arg_required_else_help(true) */)]
/// Katac is a tool to help you do katas everyday
pub struct Args {
    /// Custom directory to copy katas from (default: ./katas)
    #[arg(required = false, short, long)]
    pub katas_dir: Option<String>,

    /// Custom directory to copy katas to everyday (default: ./days)
    #[arg(required = false, short, long)]
    pub days_dir: Option<String>,

    /// Custom config file (default: ./katac.toml)
    #[arg(required = false, short, long)]
    pub config_file: Option<String>,

    /// Katas you want to do today
    #[arg(required = false, num_args=1..)]
    pub kata_names_args: Option<Vec<String>>,

    #[command(subcommand)]
    pub subcommand: Option<KatacSubcommands>,
}

#[derive(Clone, Subcommand, Debug)]
pub enum KatacSubcommands {
    /// Katas you want to run today (requires a makefile with the  'run' target in the kata's root folder)
    Run {
        /// Katas to run
        #[arg(required = false, num_args = 1..)]
        kata_names: Option<Vec<String>>,

        /// Run custom command for given kata
        #[arg(short, long)]
        command: Option<String>,
    },

    /// Number of katas you want to do today, randomly taken from katas.toml
    Random {
        /// Katas to run
        #[arg(required = true, num_args = 1..)]
        number_of_katas: u8,
    },

    /// Create a new kata
    Add {
        /// Name of the kata you want to create
        #[arg(required = true, num_args = 1..)]
        kata_name: String,
    },

    Workspace {
        // subcommand list
        #[command(subcommand)]
        subcommand: Option<WorkspaceSubcommands>,
    },
}

#[derive(Clone, Subcommand, Debug)]
pub enum WorkspaceSubcommands {
    /// Add a new workspace
    Add {
        /// Name of the workspace
        #[arg(required = true, num_args = 1..)]
        name: String,

        /// Path to the workspace, can be remote
        #[arg(required = true, short, long)]
        path: String,

        /// Remote repository
        #[arg(short, long)]
        remote: Option<String>,
    },

    /// List all workspaces
    List,

    /// Remove a workspace
    Remove {
        /// Name of the workspace
        #[arg(required = true, num_args = 1..)]
        name: String,
    },

    /// List all katas in a workspace
    ListKatas {
        /// Name of the workspace
        #[arg(required = true, num_args = 1..)]
        workspace_name: String,
    },

    ListAllKatas,
}
