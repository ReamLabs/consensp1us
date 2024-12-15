use std::time::Duration;

use lighthouse_eth2::{
    types::{BlockId, StateId},
    BeaconNodeHttpClient, SensitiveUrl, Timeouts,
};
use lighthouse_types::{BlindedPayload, MainnetEthSpec, SignedBeaconBlock, Slot};

pub struct BeaconClient {
    client: BeaconNodeHttpClient,
}

impl BeaconClient {
    pub fn new(rpc_url: String) -> Result<Self, String> {
        let client = BeaconNodeHttpClient::new(
            SensitiveUrl::parse(&rpc_url)
                .map_err(|e| format!("Failed to parse beacon http server: {:?}", e))?,
            Timeouts::set_all(Duration::from_secs(30)),
        );
        Ok(Self { client })
    }

    pub async fn get_beacon_state(&self, slot: u64) -> Result<Slot, String> {
        let beacon_state = self
            .client
            .get_debug_beacon_states::<MainnetEthSpec>(StateId::Slot(slot.into()))
            .await
            .unwrap()
            .expect("Failed to get beacon state");

        println!("slot: {:?}", beacon_state.data.slot());

        Ok(beacon_state.data.slot())
    }

    pub async fn get_beacon_block(
        &self,
        slot: u64,
    ) -> Result<Option<SignedBeaconBlock<MainnetEthSpec, BlindedPayload<MainnetEthSpec>>>, String>
    {
        let block = self
            .client
            .get_beacon_blinded_blocks::<MainnetEthSpec>(BlockId::Slot(slot.into()))
            .await
            .map_err(|e| format!("Failed to get beacon block: {:?}", e))?;

        let block = match block {
            Some(block) => block,
            None => return Ok(None),
        };

        println!("block: {:?}", block.data.state_root());

        Ok(Some(block.data))
    }

    pub async fn get_beacon_blocks(
        &self,
        start: u64,
        end: u64,
    ) -> Result<Vec<SignedBeaconBlock<MainnetEthSpec, BlindedPayload<MainnetEthSpec>>>, String>
    {
        let mut blocks = Vec::new();

        for slot in start..=end {
            if let Ok(Some(block)) = self.get_beacon_block(slot).await {
                blocks.push(block);
            }
        }

        Ok(blocks)
    }
}
