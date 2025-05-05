// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use ream_consensus::electra::beacon_state::BeaconState;
use ream_lib::input::OperationInput;
use ssz::Encode;

#[sp1_derive::cycle_tracker]
pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // NOTE: BeaconState/OperationInput should implement Serialize & Deserialize trait.

    println!("cycle-tracker-report-start: read-pre-state");
    let mut pre_state: BeaconState = sp1_zkvm::io::read();
    println!("cycle-tracker-report-end: read-pre-state");

    println!("cycle-tracker-report-start: read-operation-input");
    let input = sp1_zkvm::io::read::<OperationInput>();
    println!("cycle-tracker-report-end: read-operation-input");

    // Main logic of the program.
    // State transition of the beacon state.

    println!("cycle-tracker-report-start: process-operation");
    match input {
        OperationInput::Attestation(attestation) => {
            let _ = pre_state.process_attestation(&attestation);
        }
        OperationInput::AttesterSlashing(attester_slashing) => {
            let _ = pre_state.process_attester_slashing(&attester_slashing);
        }
        OperationInput::BeaconBlock(block) => {
            let _ = pre_state.process_block_header(&block);
        }
        OperationInput::SignedBLSToExecutionChange(bls_change) => {
            let _ = pre_state.process_bls_to_execution_change(&bls_change);
        }
        OperationInput::Deposit(deposit) => {
            let _ = pre_state.process_deposit(&deposit);
        }
        OperationInput::BeaconBlockBody(_block_body) => {
            panic!("Not implemented");
            // let _ = pre_state.process_execution_payload(&block_body);
        }
        OperationInput::ProposerSlashing(proposer_slashing) => {
            let _ = pre_state.process_proposer_slashing(&proposer_slashing);
        }
        OperationInput::SyncAggregate(sync_aggregate) => {
            let _ = pre_state.process_sync_aggregate(&sync_aggregate);
        }
        OperationInput::SignedVoluntaryExit(voluntary_exit) => {
            let _ = pre_state.process_voluntary_exit(&voluntary_exit);
        }
        OperationInput::ExecutionPayload(execution_payload) => {
            let _ = pre_state.process_withdrawals(&execution_payload);
        }
    }
    println!("cycle-tracker-report-end: process-operation");

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    // NOTE: BeaconState should implement Serialize & Deserialize trait.

    println!("cycle-tracker-report-start: convert-to-ssz-bytes");
    let pre_state_bytes = pre_state.as_ssz_bytes();
    println!("cycle-tracker-report-end: convert-to-ssz-bytes");

    println!("cycle-tracker-report-start: commit");
    sp1_zkvm::io::commit_slice(&pre_state_bytes);
    println!("cycle-tracker-report-end: commit");
}
