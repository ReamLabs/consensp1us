use clap::{Parser, ValueEnum};
use derive_more::Display;

#[derive(Debug, Clone, Parser)]
pub struct ForkArgs {
    #[clap(long, short, default_value_t = Fork::Deneb)]
    pub fork: Fork,
}

#[derive(ValueEnum, Debug, Clone, Default, Display)]
#[clap(rename_all = "lowercase")]
pub enum Fork {
    #[default]
    #[display("deneb")]
    Deneb,

    #[display("electra")]
    Electra,
}
