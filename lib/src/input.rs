use serde::{Deserialize, Serialize};

use crate::operation::OperationName;

#[derive(Serialize, Deserialize, Debug)]
pub struct OperationInput {
    pub op: OperationName,
    pub data: Vec<u8>,
}
