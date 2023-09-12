use clap::{Parser, Subcommand};
use hig::{cmd, BranchType, ConfigType};

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "feat")]
    #[command(about = "Create a new feature branch")]
    #[command(arg_required_else_help = true)]
    Feature {
        /// The name of the feature branch
        name: String,
    },

    #[command(about = "Create a new hotfix branch")]
    #[command(arg_required_else_help = true)]
    Hotfix {
        /// The name of the hotfix branch
        name: String,
    },

    #[command(about = "Create a new release branch")]
    #[command(arg_required_else_help = true)]
    Release {
        /// The name of the release branch
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
        value: Option<String>,

        #[arg(short = 'g', long = "global", default_value_t = false)]
        is_global: bool,
    },

    #[command(about = "Create a new pull request")]
    Pr,
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Feature { name } => cmd::branch::run(&name, BranchType::Feature),
        Commands::Hotfix { name } => cmd::branch::run(&name, BranchType::Hotfix),
        Commands::Release { name } => cmd::branch::run(&name, BranchType::Release),
        Commands::Rm => cmd::rm::run(),
        Commands::Reset => cmd::reset::run(),
        Commands::Config {
            key,
            value,
            is_global,
        } => {
            let t = if is_global {
                ConfigType::Global
            } else {
                ConfigType::Local
            };

            cmd::config::run(&key, &value, t)
        }
        Commands::Pr => cmd::pr::run().await,
    };

    if let Err(e) = result {
        eprintln!("{}", e);
    }
}
