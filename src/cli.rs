use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Written tasks to the journal file.
pub enum Action {
    Add {
        /// The task description text.
        text: String,
    },
    /// Remove an entry from the journal file by positon.
    Done {
        #[structopt()]
        position: usize,
    },
    /// List all tasks in the journal file.
    List,
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "Ruty Journal",
    about = "A command line to-do app written in Rust"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    /// Use a differnt journal file.
    #[structopt(parse(from_os_str), short, long)]
    pub journal_file: Option<PathBuf>,
}
