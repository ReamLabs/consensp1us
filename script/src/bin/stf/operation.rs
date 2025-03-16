use std::path::PathBuf;

use ream_consensus::deneb::beacon_state::BeaconState;
use ream_lib::{file::read_file, input::OperationInput};
use sp1_sdk::{include_elf, ProverClient, SP1Stdin};
use tracing::info;

use crate::{
    cli::{operation::OperationName, stf_mode::STFMode},
    Args,
};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const REAM_ELF: &[u8] = include_elf!("ream-operations");

pub fn run_operation(test_case_dir: PathBuf, args: Args) {
    assert!(args.stf_mode.stf_mode == STFMode::Operation);

    let operation_name = args
        .operation
        .operation_name
        .expect("operation-name must be provided");
    let excluded_cases = args.excluded_cases;

    // Load the test assets.
    // These assets are from consensus-specs repo.
    let base_dir = test_case_dir
        .join(format!("{}", args.fork.fork))
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
        let input_path = &case_dir.join(format!("{}.ssz_snappy", operation_name.to_input_name()));

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
            OperationName::SyncAggregate => OperationInput::SyncAggregate(read_file(input_path)),
            OperationName::VoluntaryExit => {
                OperationInput::SignedVoluntaryExit(read_file(input_path))
            }
            OperationName::Withdrawals => OperationInput::ExecutionPayload(read_file(input_path)),
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
            let (output, report) = client.execute(REAM_ELF, &stdin).run().unwrap();
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
            let (pk, vk) = client.setup(REAM_ELF);

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
