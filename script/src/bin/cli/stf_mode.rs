use clap::{Parser, ValueEnum};
use derive_more::Display;

#[derive(Debug, Clone, Parser)]
pub struct STFModeArgs {
    #[clap(long, short)]
    pub stf_mode: STFMode,
}

#[derive(ValueEnum, Debug, Clone, Display, PartialEq, Eq)]
#[clap(rename_all = "snake_case")]
pub enum STFMode {
    #[display("operation")]
    Operation,
    #[display("epoch_processing")]
    EpochProcessing,
}
