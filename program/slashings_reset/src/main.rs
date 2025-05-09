// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use ream_consensus::view::PartialBeaconStateBuilder;

#[sp1_derive::cycle_tracker]
pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.

    println!("cycle-tracker-report-start: read-builder");
    let builder: PartialBeaconStateBuilder = sp1_zkvm::io::read();
    println!("cycle-tracker-report-end: read-builder");

    println!("cycle-tracker-report-start: build-partial-beacon-state");
    let mut partial_beacon_state = builder
        .build()
        .expect("Failed to build partial beacon state");
    println!("cycle-tracker-report-end: build-partial-beacon-state");

    // Main logic of the program.
    // State transition of the beacon state.

    println!("cycle-tracker-report-start: process-slashings-reset");
    partial_beacon_state
        .process_slashings_reset()
        .expect("Failed to process slashings reset");
    println!("cycle-tracker-report-end: process-slashings-reset");

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.

    println!("cycle-tracker-report-start: commit");
    sp1_zkvm::io::commit(&partial_beacon_state);
    println!("cycle-tracker-report-end: commit");
}
