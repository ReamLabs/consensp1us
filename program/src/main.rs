// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use ream_consensus::deneb::{beacon_block::BeaconBlock, beacon_state::BeaconState};
use ssz::Encode;

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
    let block = sp1_zkvm::io::read::<BeaconBlock>();
    println!("cycle-tracker-end: read-block");

    // Main logic of the program.
    // State transition of the beacon state.

    println!("cycle-tracker-start: process-block-header");
    let _ = pre_state.process_block_header(block);
    println!("cycle-tracker-end: process-block-header");

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
