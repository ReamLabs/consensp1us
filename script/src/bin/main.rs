use clap::Parser;
use tracing::error;

mod cli;
mod stf;

use cli::stf_mode::STFMode;

/// The arguments for the command.
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(flatten)]
    stf_mode: cli::stf_mode::STFModeArgs,

    #[clap(flatten)]
    fork: cli::fork::ForkArgs,

    #[clap(flatten)]
    operation: cli::operation::OperationArgs,

    #[clap(long)]
    excluded_cases: Vec<String>,
}

fn main() {
    let test_case_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("mainnet")
        .join("tests")
        .join("mainnet");
    if !std::path::Path::new(&test_case_dir).exists() {
        eprintln!("Error: You must first download test data via `make download`");
        std::process::exit(1);
    }

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        error!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    let stf_mode = args.clone().stf_mode.stf_mode;

    match stf_mode {
        STFMode::Operation => stf::operation::run_operation(test_case_dir, args),
        STFMode::EpochProcessing => todo!(),
    }
}
