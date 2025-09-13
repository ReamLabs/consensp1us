// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use ream_consensus::electra::beacon_state::BeaconState;
use ream_consensus::{
    attestation::Attestation,
    attester_slashing::AttesterSlashing,
    bls_to_execution_change::SignedBLSToExecutionChange,
    deposit::Deposit,
    electra::{beacon_block::BeaconBlock, execution_payload::ExecutionPayload},
    proposer_slashing::ProposerSlashing,
    sync_aggregate::SyncAggregate,
    voluntary_exit::SignedVoluntaryExit,
};
use ream_lib::input::OperationInput;
use ream_lib::operation::OperationName;
use ssz::Encode;

fn deserialize<T: ssz::Decode>(ssz_bytes: &[u8]) -> T {
    T::from_ssz_bytes(ssz_bytes).unwrap()
}

#[sp1_derive::cycle_tracker]
pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // NOTE: BeaconState/OperationInput should implement Serialize & Deserialize trait.

    println!("cycle-tracker-report-start: read-pre-state-bytes");
    let pre_state_bytes: Vec<u8> = sp1_zkvm::io::read();
    println!("cycle-tracker-report-end: read-pre-state-bytes");

    println!("cycle-tracker-report-start: deserialize-pre-state");
    let mut pre_state: BeaconState = deserialize(&pre_state_bytes);
    println!("cycle-tracker-report-end: deserialize-pre-state");

    println!("cycle-tracker-report-start: read-operation-input");
    let input = sp1_zkvm::io::read::<OperationInput>();
    println!("cycle-tracker-report-end: read-operation-input");

    // Main logic of the program.
    // State transition of the beacon state.

    println!("cycle-tracker-report-start: process-operation");
    match input.op {
        OperationName::Attestation => {
            println!("cycle-tracker-report-start: deserialize-attestation");
            let attestation: Attestation = deserialize(&input.data);
            println!("cycle-tracker-report-end: deserialize-attestation");
            let _ = pre_state.process_attestation(&attestation);
        }
        OperationName::AttesterSlashing => {
            println!("cycle-tracker-report-start: deserialize-attester-slashing");
            let attester_slashing: AttesterSlashing = deserialize(&input.data);
            println!("cycle-tracker-report-end: deserialize-attester-slashing");
            let _ = pre_state.process_attester_slashing(&attester_slashing);
        }
        OperationName::BlockHeader => {
            println!("cycle-tracker-report-start: deserialize-block-header");
            let block: BeaconBlock = deserialize(&input.data);
            println!("cycle-tracker-report-end: deserialize-block-header");
            let _ = pre_state.process_block_header(&block);
        }
        OperationName::BLSToExecutionChange => {
            println!("cycle-tracker-report-start: deserialize-bls-to-execution-change");
            let bls_change: SignedBLSToExecutionChange = deserialize(&input.data);
            println!("cycle-tracker-report-end: deserialize-bls-to-execution-change");
            let _ = pre_state.process_bls_to_execution_change(&bls_change);
        }
        OperationName::Deposit => {
            println!("cycle-tracker-report-start: deserialize-deposit");
            let deposit: Deposit = deserialize(&input.data);
            println!("cycle-tracker-report-end: deserialize-deposit");
            let _ = pre_state.process_deposit(&deposit);
        }
        OperationName::ExecutionPayload => {
            panic!("Not implemented");
            // let block_body: BeaconBlockBody = deserialize(&input.data);
            // let _ = pre_state.process_execution_payload(&block_body);
        }
        OperationName::ProposerSlashing => {
            println!("cycle-tracker-report-start: deserialize-proposer-slashing");
            let proposer_slashing: ProposerSlashing = deserialize(&input.data);
            println!("cycle-tracker-report-end: deserialize-proposer-slashing");
            let _ = pre_state.process_proposer_slashing(&proposer_slashing);
        }
        OperationName::SyncAggregate => {
            println!("cycle-tracker-report-start: deserialize-sync-aggregate");
            let sync_aggregate: SyncAggregate = deserialize(&input.data);
            println!("cycle-tracker-report-end: deserialize-sync-aggregate");
            let _ = pre_state.process_sync_aggregate(&sync_aggregate);
        }
        OperationName::VoluntaryExit => {
            println!("cycle-tracker-report-start: deserialize-voluntary-exit");
            let voluntary_exit: SignedVoluntaryExit = deserialize(&input.data);
            println!("cycle-tracker-report-end: deserialize-voluntary-exit");
            let _ = pre_state.process_voluntary_exit(&voluntary_exit);
        }
        OperationName::Withdrawals => {
            println!("cycle-tracker-report-start: deserialize-withdrawals");
            let execution_payload: ExecutionPayload = deserialize(&input.data);
            println!("cycle-tracker-report-end: deserialize-withdrawals");
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
