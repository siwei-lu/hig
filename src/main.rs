use clap::{Parser, Subcommand};
use hig::cmd;

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "feat")]
    #[command(about = "Create a new feature branch")]
    #[command(arg_required_else_help = true)]
    Feature {
        /// The name of the feature branch
        name: String,
    },

    #[command(about = "Remove the current branch")]
    Rm,

    #[command(about = "Reset the current branch")]
    #[command(
        long_about = "Reset the current branch, usually used after the branch is merged to main by squash merge"
    )]
    Reset,

    #[command(about = "Configure hig")]
    #[command(arg_required_else_help = true)]
    Config {
        /// The key to set
        key: String,
        /// The value to set
        value: String,
    },
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Feature { name } => cmd::feature::run(&name),
        Commands::Rm => cmd::rm::run(),
        Commands::Reset => cmd::reset::run(),
        Commands::Config { key, value } => cmd::config::run(&key, &value),
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
