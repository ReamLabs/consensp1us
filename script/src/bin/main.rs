use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin, include_elf};
use tracing::{error, info};
use tree_hash::TreeHash;

use ream_consensus::{
    constants::{BEACON_STATE_MERKLE_DEPTH, BEACON_STATE_SLASHINGS_INDEX, BEACON_STATE_SLOT_INDEX},
    electra::beacon_state::BeaconState,
    view::{
        BeaconStateView, PartialBeaconState, PartialBeaconStateBuilder, SLASHINGS_GENERALIZED_INDEX,
    },
};
use ream_lib::{file::read_file, input::OperationInput};

mod cli;
use cli::operation::OperationName;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const OPERATIONS_ELF: &[u8] = include_elf!("ream-operations");
pub const SLASHINGS_RESET_ELF: &[u8] = include_elf!("ream-slashings-reset");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Argument for zkVMs
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    // Argument for STFs

    // EF test (default flow)
    #[clap(long, conflicts_with = "replay")]
    ef_test: bool,

    #[clap(flatten)]
    fork: cli::fork::ForkArgs,

    #[clap(flatten)]
    operation: cli::operation::OperationArgs,

    #[clap(long)]
    excluded_cases: Option<Vec<String>>,

    // Replay test: requires state and block file paths
    #[clap(long, conflicts_with = "ef_test")]
    replay: bool,

    #[clap(long, requires = "replay")]
    state_path: Option<String>,

    #[clap(long, requires = "replay")]
    expected_post_path: Option<String>,

    #[clap(long, requires = "replay")]
    block_path: Option<String>,
}

fn main() {
    if std::env::var("RUST_LOG").is_err() {
        unsafe {
            std::env::set_var("RUST_LOG", "info");
        }
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

    if args.ef_test == args.replay {
        error!("Error: You must specify either --ef-test or --replay");
        std::process::exit(1);
    }

    if args.replay {
        info!("Executing with replay test...");

        let mut pre_state: BeaconState = read_file(&std::path::PathBuf::from(
            args.state_path.unwrap_or_default(),
        ));
        let expected_post: BeaconState = read_file(&std::path::PathBuf::from(
            args.expected_post_path.unwrap_or_default(),
        ));
        let root = pre_state.tree_hash_root();

        let all_leaves = pre_state.merkle_leaves();
        let tree = ream_merkle::merkle_tree(&all_leaves, BEACON_STATE_MERKLE_DEPTH)
            .expect("Failed to create merkle tree");

        let target_indices = vec![BEACON_STATE_SLOT_INDEX, BEACON_STATE_SLASHINGS_INDEX];
        let multiproof =
            ream_merkle::multiproof::Multiproof::generate::<BEACON_STATE_MERKLE_DEPTH>(
                &tree,
                &target_indices,
            )
            .expect("Failed to generate multiproof");

        let builder = PartialBeaconStateBuilder::from_root(root)
            .with_multiproof(multiproof)
            .with_slot(pre_state.slot)
            .with_slashings(&pre_state.slashings);

        // Setup the prover client.
        let client = ProverClient::from_env();

        // Setup the inputs.
        let mut stdin = SP1Stdin::new();

        stdin.write(&builder);

        // Execute the program
        let (output, report) = client.execute(SLASHINGS_RESET_ELF, &stdin).run().unwrap();
        info!("Program executed successfully.");

        // Decode the output
        let result: PartialBeaconState = bincode::deserialize(output.as_slice()).unwrap();

        for &mutated in result.dirty.iter() {
            match mutated {
                SLASHINGS_GENERALIZED_INDEX => {
                    pre_state.slashings = result.slashings().unwrap().clone();
                }
                _ => {
                    panic!("Unexpected mutated index: {}", mutated);
                }
            }
        }

        assert_eq!(expected_post.tree_hash_root(), pre_state.tree_hash_root());

        // Record the number of cycles executed.
        info!("----- Cycle Tracker -----");
        info!("Number of cycles: {}", report.total_instruction_count());
        info!("Number of syscall count: {}", report.total_syscall_count());
        for (key, value) in report.cycle_tracker.iter() {
            info!("{}: {}", key, value);
        }
        info!("----- Cycle Tracker End -----");
    }

    if args.ef_test {
        info!("Executing with EF test...");

        let test_case_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("mainnet")
            .join("tests")
            .join("mainnet");
        if !std::path::Path::new(&test_case_dir).exists() {
            eprintln!("Error: You must first download test data via `make download`");
            std::process::exit(1);
        }

        let fork = args.fork.fork;
        let operation_name = args
            .operation
            .operation_name
            .expect("Operation name is required");
        let excluded_cases = args.excluded_cases.unwrap_or_default();

        // Load the test assets.
        // These assets are from consensus-specs repo.
        let base_dir = test_case_dir
            .join(format!("{}", fork))
            .join("operations")
            .join(format!("{}", operation_name))
            .join("pyspec_tests");

        let test_cases = ream_lib::file::get_test_cases(&base_dir);
        for test_case in test_cases {
            if excluded_cases.contains(&test_case) {
                info!("Skipping test case: {}", test_case);
                continue;
            }

            info!("{}", "-".repeat(50));
            info!("[{}] Test case: {}", operation_name, test_case);

            let case_dir = &base_dir.join(&test_case);
            let input_path =
                &case_dir.join(format!("{}.ssz_snappy", operation_name.to_input_name()));

            println!("case path: {:?}", case_dir);

            let pre_state: BeaconState = read_file(&case_dir.join("pre.ssz_snappy"));
            let input = match operation_name {
                OperationName::Attestation => OperationInput::Attestation(read_file(input_path)),
                OperationName::AttesterSlashing => {
                    OperationInput::AttesterSlashing(read_file(input_path))
                }
                OperationName::BlockHeader => OperationInput::BeaconBlock(read_file(input_path)),
                OperationName::BLSToExecutionChange => {
                    OperationInput::SignedBLSToExecutionChange(read_file(input_path))
                }
                OperationName::Deposit => OperationInput::Deposit(read_file(input_path)),
                OperationName::ExecutionPayload => {
                    OperationInput::BeaconBlockBody(read_file(input_path))
                }
                OperationName::ProposerSlashing => {
                    OperationInput::ProposerSlashing(read_file(input_path))
                }
                OperationName::SyncAggregate => {
                    OperationInput::SyncAggregate(read_file(input_path))
                }
                OperationName::VoluntaryExit => {
                    OperationInput::SignedVoluntaryExit(read_file(input_path))
                }
                OperationName::Withdrawals => {
                    OperationInput::ExecutionPayload(read_file(input_path))
                }
            };
            let post_state_opt: Option<BeaconState> = {
                if case_dir.join("post.ssz_snappy").exists() {
                    Some(read_file(&case_dir.join("post.ssz_snappy")))
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
                let (output, report) = client.execute(OPERATIONS_ELF, &stdin).run().unwrap();
                info!("Program executed successfully.");

                // Decode the output
                let result: BeaconState = ssz::Decode::from_ssz_bytes(output.as_slice()).unwrap();

                // Match `post_state_opt`: some test cases should not mutate beacon state.
                match post_state_opt {
                    Some(post_state) => {
                        assert_eq!(result, post_state);
                        info!("Execution is correct!: State mutated");
                    }
                    None => {
                        assert_eq!(result, pre_state);
                        info!("Execution is correct!: State should not be mutated");
                    }
                }

                // Record the number of cycles executed.
                info!("----- Cycle Tracker -----");
                info!("[{}] Test case: {}", operation_name, test_case);
                info!("Number of cycles: {}", report.total_instruction_count());
                info!("Number of syscall count: {}", report.total_syscall_count());
                for (key, value) in report.cycle_tracker.iter() {
                    info!("{}: {}", key, value);
                }
                info!("----- Cycle Tracker End -----");
            } else {
                // Setup the program for proving.
                let (pk, vk) = client.setup(OPERATIONS_ELF);

                // Generate the proof
                let proof = client
                    .prove(&pk, &stdin)
                    .run()
                    .expect("failed to generate proof");

                info!("Successfully generated proof!");

                // Verify the proof.
                client.verify(&proof, &vk).expect("failed to verify proof");
                info!("Successfully verified proof!");
            }
            info!("{}", "-".repeat(50));
        }
    }
}
