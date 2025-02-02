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
use ream_consensus::deneb::{beacon_block::BeaconBlock, beacon_state::BeaconState};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const REAM_ELF: &[u8] = include_elf!("ream-program");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    // Setup the logger.
    sp1_sdk::utils::setup_logger();
    dotenv::dotenv().ok();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Load the test assets.
    // This asset is from consensus-specs repo.
    // Path: tests/mainnet/deneb/operations/block_header/pyspec_tests/basic_block_header
    let base_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("data");

    let pre_state: BeaconState =
        ream_lib::snappy::read_ssz_snappy(&base_dir.join("pre.ssz_snappy"))
            .expect("cannot find test asset(pre.ssz_snappy) or decode it");
    let block: BeaconBlock = ream_lib::snappy::read_ssz_snappy(&base_dir.join("block.ssz_snappy"))
        .expect("cannot find test asset(block.ssz_snappy) or decode it");
    let post_state: BeaconState =
        ream_lib::snappy::read_ssz_snappy(&base_dir.join("post.ssz_snappy"))
            .expect("cannot find test asset(post.ssz_snappy) or decode it");

    // Setup the prover client.
    let client = ProverClient::from_env();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&pre_state);
    stdin.write(&block);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(REAM_ELF, &stdin).run().unwrap();
        println!("Program executed successfully.");

        // Decode the output
        let result: BeaconState = ssz::Decode::from_ssz_bytes(output.as_slice()).unwrap();

        // Compare the output with the expected post state.
        assert_eq!(result, post_state);
        println!("Execution is correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(REAM_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, &stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
