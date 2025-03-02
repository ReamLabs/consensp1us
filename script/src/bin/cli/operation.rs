use clap::{Parser, ValueEnum};
use derive_more::Display;

#[derive(Debug, Clone, Parser)]
pub struct OperationArgs {
    #[clap(long, short)]
    pub operation_name: OperationName,
}

#[derive(ValueEnum, Debug, Clone, Display)]
#[clap(rename_all = "snake_case")]
pub enum OperationName {
    #[display("attestation")]
    Attestation,
    #[display("attester_slashing")]
    AttesterSlashing,
    #[display("block_header")]
    BlockHeader,
    #[display("bls_to_execution_change")]
    BLSToExecutionChange,
    #[display("deposit")]
    Deposit,
    #[display("execution_payload")]
    ExecutionPayload,
    #[display("proposer_slashing")]
    ProposerSlashing,
    #[display("sync_aggregate")]
    SyncAggregate,
    #[display("voluntary_exit")]
    VoluntaryExit,
    #[display("withdrawals")]
    Withdrawals,
}

impl OperationName {
    pub fn to_input_name(&self) -> String {
        match self {
            OperationName::Attestation => "attestation".to_string(),
            OperationName::AttesterSlashing => "attester_slashing".to_string(),
            OperationName::BlockHeader => "block".to_string(),
            OperationName::BLSToExecutionChange => "address_change".to_string(),
            OperationName::Deposit => "deposit".to_string(),
            OperationName::ExecutionPayload => "body".to_string(),
            OperationName::ProposerSlashing => "proposer_slashing".to_string(),
            OperationName::SyncAggregate => "sync_aggregate".to_string(),
            OperationName::VoluntaryExit => "voluntary_exit".to_string(),
            OperationName::Withdrawals => "execution_payload".to_string(),
        }
    }
}
