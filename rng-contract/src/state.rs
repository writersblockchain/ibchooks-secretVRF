use cosmwasm_std::Binary;
use schemars::JsonSchema;
use secret_toolkit_storage::Item;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]

pub struct RandomBinary {
    pub random_binary: Binary,
}

pub static RANDOM_BINARY: Item<RandomBinary> = Item::new(b"random_binary");
