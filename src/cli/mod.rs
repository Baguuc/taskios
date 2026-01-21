pub mod commands;

/// # MainCli
///
/// defines the structure of the cli arguments and flags of the program.
///
#[derive(clap::Parser)]
#[command(name = "taskios")]
#[command(bin_name = "taskios")]
#[command(about = "A task organization software's API.", long_about = None)]
pub enum MainCli {
    #[command(about = "Run the HTTP server", long_about = None)]
    Run(CliFlags),
}

/// defines the structure of cli flags of the program.
#[derive(clap::Args, Clone)]
pub struct CliFlags {
    #[clap(long, short)]
    config: Option<String>,
}

impl MainCli {
    /// parses the CLI arguments and flags and runs the program.
    pub async fn run() {
        use clap::Parser;

        let cli = Self::parse();
        cli.execute().await;
    }

    /// runs the program with already scraped arguments and flags.
    pub async fn execute(self) {
        match self {
            Self::Run(args) => {
                commands::run(args).await;
            }
        };
    }
}
