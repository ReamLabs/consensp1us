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
use cli::operation::OperationName;
use ream_consensus::{
    attestation::Attestation,
    attester_slashing::AttesterSlashing,
    bls_to_execution_change::SignedBLSToExecutionChange,
    deneb::{
        beacon_block::BeaconBlock, beacon_block_body::BeaconBlockBody, beacon_state::BeaconState,
        execution_payload::ExecutionPayload,
    },
    deposit::Deposit,
    proposer_slashing::ProposerSlashing,
    sync_aggregate::SyncAggregate,
    voluntary_exit::SignedVoluntaryExit,
};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};

mod cli;

use ream_lib::input::OperationInput;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const REAM_ELF: &[u8] = include_elf!("ream-operations");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Argument for zkVMs

    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    /// Argument for STFs

    #[clap(flatten)]
    fork: cli::fork::ForkArgs,

    #[clap(flatten)]
    operation: cli::operation::OperationArgs,
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
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    let fork: cli::fork::Fork = args.fork.fork;
    let operation_name: cli::operation::OperationName = args.operation.operation_name;

    // Load the test assets.
    // These assets are from consensus-specs repo.
    let base_dir = test_case_dir
        .join(format!("{}", fork))
        .join("operations")
        .join(format!("{}", operation_name))
        .join("pyspec_tests");

    let test_cases = ream_lib::file::get_test_cases(&base_dir);
    for test_case in test_cases {
        println!("[{}] Test case: {}", operation_name, test_case);

        let case_dir = &base_dir.join(test_case);
        let input_path = &case_dir.join(format!("{}.ssz_snappy", operation_name.to_input_name()));

        let pre_state: BeaconState =
            ream_lib::snappy::read_ssz_snappy(&case_dir.join("pre.ssz_snappy"))
                .expect("cannot find test asset(pre.ssz_snappy) or decode it");
        let input = match operation_name {
            OperationName::Attestation => {
                let input: Attestation = ream_lib::snappy::read_ssz_snappy(input_path)
                    .expect("cannot find input asset or decode it");
                OperationInput::Attestation(input)
            }
            OperationName::AttesterSlashing => {
                let input: AttesterSlashing = ream_lib::snappy::read_ssz_snappy(input_path)
                    .expect("cannot find input asset or decode it");
                OperationInput::AttesterSlashing(input)
            }
            OperationName::BlockHeader => {
                let input: BeaconBlock = ream_lib::snappy::read_ssz_snappy(input_path)
                    .expect("cannot find input asset or decode it");
                OperationInput::BeaconBlock(input)
            }
            OperationName::BLSToExecutionChange => {
                let input: SignedBLSToExecutionChange =
                    ream_lib::snappy::read_ssz_snappy(input_path)
                        .expect("cannot find input asset or decode it");
                OperationInput::SignedBLSToExecutionChange(input)
            }
            OperationName::Deposit => {
                let input: Deposit = ream_lib::snappy::read_ssz_snappy(input_path)
                    .expect("cannot find input asset or decode it");
                OperationInput::Deposit(input)
            }
            OperationName::ExecutionPayload => {
                let input: BeaconBlockBody = ream_lib::snappy::read_ssz_snappy(input_path)
                    .expect("cannot find input asset or decode it");
                OperationInput::BeaconBlockBody(input)
            }
            OperationName::ProposerSlashing => {
                let input: ProposerSlashing = ream_lib::snappy::read_ssz_snappy(input_path)
                    .expect("cannot find input asset or decode it");
                OperationInput::ProposerSlashing(input)
            }
            OperationName::SyncAggregate => {
                let input: SyncAggregate = ream_lib::snappy::read_ssz_snappy(input_path)
                    .expect("cannot find input asset or decode it");
                OperationInput::SyncAggregate(input)
            }
            OperationName::VoluntaryExit => {
                let input: SignedVoluntaryExit = ream_lib::snappy::read_ssz_snappy(input_path)
                    .expect("cannot find input asset or decode it");
                OperationInput::SignedVoluntaryExit(input)
            }
            OperationName::Withdrawals => {
                let input: ExecutionPayload = ream_lib::snappy::read_ssz_snappy(input_path)
                    .expect("cannot find input asset or decode it");
                OperationInput::ExecutionPayload(input)
            }
        };
        let post_state_opt: Option<BeaconState> = {
            if case_dir.join("post.ssz_snappy").exists() {
                let post_state: BeaconState =
                    ream_lib::snappy::read_ssz_snappy(&case_dir.join("pre.ssz_snappy"))
                        .expect("cannot find test asset(pre.ssz_snappy) or decode it");
                Some(post_state)
            } else {
                None
            }
        };

        // Setup the prover client.
        let client = ProverClient::from_env();

        // Setup the inputs.
        let mut stdin = SP1Stdin::new();
        stdin.write(&pre_state);
        stdin.write(&input);

        if args.execute {
            // Execute the program
            let (output, report) = client.execute(REAM_ELF, &stdin).run().unwrap();
            println!("Program executed successfully.");

            // Decode the output
            let result: BeaconState = ssz::Decode::from_ssz_bytes(output.as_slice()).unwrap();

            match post_state_opt {
                Some(post_state) => {
                assert_eq!(result, post_state);
                    println!("Execution is correct!: State mutated");
                }
                None => {
                    assert_eq!(result, pre_state);
                    println!("Execution is correct!: State should not be mutated");
                }
            }

            // Record the number of cycles executed.
            println!("Number of cycles: {}", report.total_instruction_count());
            println!("Number of syscall count: {}", report.total_syscall_count());
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
}
