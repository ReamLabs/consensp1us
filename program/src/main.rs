// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use ream_consensus::deneb::{beacon_block::BeaconBlock, beacon_state::BeaconState};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // NOTE: BeaconState/BeaconBlock should implement Serialize & Deserialize trait.
    let mut pre_state = sp1_zkvm::io::read::<BeaconState>();
    let block = sp1_zkvm::io::read::<BeaconBlock>();

    // Main logic of the program.
    // State transition of the beacon state.
    let _ = pre_state.process_block_header(block);

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    // NOTE: BeaconState should implement Serialize & Deserialize trait.
    sp1_zkvm::io::commit::<BeaconState>(&pre_state);
}
