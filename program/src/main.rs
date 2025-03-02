// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use ream_consensus::deneb::beacon_state::BeaconState;
use ream_lib::input::OperationInput;
use ssz::Encode;

#[sp1_derive::cycle_tracker]
pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // NOTE: BeaconState/BeaconBlock should implement Serialize & Deserialize trait.

    println!("cycle-tracker-start: read-pre-state");
    let mut pre_state = sp1_zkvm::io::read::<BeaconState>();
    println!("cycle-tracker-end: read-pre-state");

    println!("cycle-tracker-start: read-block");
    let input = sp1_zkvm::io::read::<OperationInput>();
    println!("cycle-tracker-end: read-block");

    // Main logic of the program.
    // State transition of the beacon state.

    match input {
        OperationInput::Attestation(attestation) => {
            println!("cycle-tracker-start: process-attestation");
            let _ = pre_state.process_attestation(&attestation);
            println!("cycle-tracker-end: process-attestation");
        }
        OperationInput::AttesterSlashing(attester_slashing) => {
            println!("cycle-tracker-start: process-attester-slashing");
            let _ = pre_state.process_attester_slashing(&attester_slashing);
            println!("cycle-tracker-end: process-attester-slashing");
        }
        OperationInput::BeaconBlock(block) => {
            println!("cycle-tracker-start: process-block-header");
            let _ = pre_state.process_block_header(&block);
            println!("cycle-tracker-end: process-block-header");
        }
        OperationInput::SignedBLSToExecutionChange(bls_change) => {
            println!("cycle-tracker-start: process-bls-to-execution-change");
            let _ = pre_state.process_bls_to_execution_change(&bls_change);
            println!("cycle-tracker-end: process-bls-to-execution-change");
        }
        OperationInput::Deposit(deposit) => {
            println!("cycle-tracker-start: process-deposit");
            let _ = pre_state.process_deposit(&deposit);
            println!("cycle-tracker-end: process-deposit");
        }
        OperationInput::BeaconBlockBody(_block_body) => {
            panic!("Not implemented");
            // println!("cycle-tracker-start: process-execution-payload");
            // let _ = pre_state.process_execution_payload(&block_body);
            // println!("cycle-tracker-end: process-execution-payload");
        }
        OperationInput::ProposerSlashing(proposer_slashing) => {
            println!("cycle-tracker-start: process-proposer-slashing");
            let _ = pre_state.process_proposer_slashing(&proposer_slashing);
            println!("cycle-tracker-end: process-proposer-slashing");
        }
        OperationInput::SyncAggregate(sync_aggregate) => {
            println!("cycle-tracker-start: process-sync-aggregate");
            let _ = pre_state.process_sync_aggregate(&sync_aggregate);
            println!("cycle-tracker-end: process-sync-aggregate");
        }
        OperationInput::SignedVoluntaryExit(voluntary_exit) => {
            println!("cycle-tracker-start: process-voluntary-exit");
            let _ = pre_state.process_voluntary_exit(&voluntary_exit);
            println!("cycle-tracker-end: process-voluntary-exit");
        }
        OperationInput::ExecutionPayload(execution_payload) => {
            println!("cycle-tracker-start: process-withdrawals");
            let _ = pre_state.process_withdrawals(&execution_payload);
            println!("cycle-tracker-end: process-withdrawals");
        }
    }

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    // NOTE: BeaconState should implement Serialize & Deserialize trait.

    println!("cycle-tracker-start: convert-to-ssz-bytes");
    let pre_state_bytes = pre_state.as_ssz_bytes();
    println!("cycle-tracker-end: convert-to-ssz-bytes");

    println!("cycle-tracker-start: commit");
    sp1_zkvm::io::commit_slice(&pre_state_bytes);
    println!("cycle-tracker-end: commit");
}
