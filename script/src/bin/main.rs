//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use clap::Parser;
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

mod beacon_client;
mod cli;
use beacon_client::BeaconClient;
use cli::ProviderArgs;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ETH_CONSENSUS: &[u8] = include_elf!("eth-consensus");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "20")]
    n: u32,

    #[clap(long)]
    slot_number: u64,

    #[clap(flatten)]
    provider: ProviderArgs,
}

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    let beacon_client =
        BeaconClient::new(args.provider.rpc_url.unwrap()).expect("Failed to create beacon client");

    // temporarily code for testing
    let _ = beacon_client.get_beacon_state(args.slot_number).await;
    let _ = beacon_client
        .get_beacon_blocks(args.slot_number, args.slot_number + 10)
        .await;

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.n);

    println!("n: {}", args.n);

    if args.execute {
        // Execute the program
        let (mut output, report) = client.execute(ETH_CONSENSUS, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output
        let result: u32 = output.read::<u32>();
        println!("Result: {}", result);

        let expected = beacon_lib::isomorphic_function(args.n);
        assert_eq!(result, expected);
        println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(ETH_CONSENSUS);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
