use clap::Parser;
use ream_lib::operation::OperationName;

#[derive(Debug, Clone, Parser)]
pub struct OperationArgs {
    #[clap(long, short)]
    pub operation_name: Option<OperationName>,
}
