use clap::Parser;
use url::Url;

/// The arguments for configuring the chain data provider.
#[derive(Debug, Clone, Parser)]
pub struct ProviderArgs {
    /// The rpc url used to fetch data about the block
    #[clap(long)]
    pub rpc_url: Option<Url>,
}
