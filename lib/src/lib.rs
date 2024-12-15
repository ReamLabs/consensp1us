use lighthouse_state_processing::{BlockReplayError, BlockReplayer};
use lighthouse_types::{BeaconState, BlindedPayload, EthSpec, SignedBeaconBlock, Slot};

use std::array::IntoIter;

// TODO: will be replaced with the actual beacon processor
pub fn isomorphic_function(n: u32) -> u32 {
    tracing::info!("isomorphic_function {}", n);
    n
}

pub fn replay_beacon_blocks<E: EthSpec>(
    start_state: BeaconState<E>,
    blocks: Vec<SignedBeaconBlock<E, BlindedPayload<E>>>,
    target_slot: Slot,
) -> BeaconState<E> {
    tracing::info!(
        "Replay beacon block from slot {} to slot {}",
        start_state.slot(),
        target_slot
    );
    let spec = E::default_spec();
    let state = BlockReplayer::<E, BlockReplayError, IntoIter<_, 0>>::new(start_state, &spec)
        .no_signature_verification()
        .apply_blocks(blocks, Some(target_slot))
        .expect("Block replay failed")
        .into_state();

    state
}
