pub mod commands;

/// Defines the structure of the cli arguments and flags of the program.
#[derive(clap::Parser)]
#[command(name = "taskios")]
#[command(bin_name = "taskios")]
#[command(about = "A task organization software's API.", long_about = None)]
pub enum MainCli {
    /// The command used to run the API's HTTP server.
    #[command(about = "Run the HTTP server", long_about = None)]
    Run(CliFlags),
    /// The command used to migrate the database.
    #[command(about = "Migrate the database", long_about = None)]
    Migrate(CliFlags)
}

/// Defines the structure of cli flags of the program.
#[derive(clap::Args, Clone)]
pub struct CliFlags {
    #[clap(long, short)]
    config: Option<String>,
}

impl MainCli {
    /// Parses the CLI arguments and flags and runs the program.
    pub async fn run() {
        use clap::Parser;

        let cli = Self::parse();
        cli.execute().await;
    }

    /// Runs the program with already scraped arguments and flags.
    pub async fn execute(self) {
        match self {
            Self::Run(args) => {
                commands::run(args).await;
            },
            Self::Migrate(args) => {
                commands::migrate(args).await;
            }
        };
    }
}
