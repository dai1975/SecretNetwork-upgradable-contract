//use cosmwasm_std::Addr;
use crate::storage;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize}; //use upgradable_contract__storage::msg as storage;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
  pub storage_address: String,
  pub storage_codehash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryGet {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  Get {
    permit: Option<storage::Permit>,
    key: String,
  },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
  Value(Option<u32>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Authz {
  PUBLIC,
  PRIVATE,
  PROTECTED(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  Set {
    permit: Option<storage::Permit>,
    key: String,
    value: u32,
    authz: Authz,
  },
}
