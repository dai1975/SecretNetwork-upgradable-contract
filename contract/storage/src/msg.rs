//use cosmwasm_std::Addr;
use schemars::JsonSchema;
use secret_toolkit::permit::Permit as Permit_;
use secret_toolkit::utils::calls::{HandleCallback, InitCallback, Query};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Permissions {
  Access,
}

pub type Permit = Permit_<Permissions>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Authz {
  owner: String,
  everyone_read: bool,
  readers: Vec<String>,
}
impl Authz {
  pub const KEY_EVERYONE: &'static str = "everyone";
  pub const KEY_PREFIX_ACCOUNT: &'static str = "account_";

  pub fn new(owner: &str, everyone_read: bool) -> Self {
    Self {
      owner: owner.to_string(),
      everyone_read: everyone_read,
      readers: vec![],
    }
  }

  pub fn is_owner(&self, account: &str) -> bool {
    self.owner.as_str() == account
  }
  pub fn is_readable(&self, account: &str) -> bool {
    if self.owner == account {
      return true;
    }
    if self.everyone_read {
      return true;
    }
    self
      .readers
      .iter()
      .find(|a| a.as_str() == account)
      .is_some()
  }
  pub fn update_owner(mut self, s: &str) -> Self {
    self.owner = s.to_string();
    self
  }
  pub fn update_everyone(mut self, b: bool) -> Self {
    self.everyone_read = b;
    self
  }
  pub fn update(mut self, account: &str, b: bool) -> Self {
    let find = self
      .readers
      .iter()
      .enumerate()
      .find(|(_i, a)| a.as_str() == account);
    match (find, b) {
      (Some((i, _a)), false) => {
        self.readers.remove(i);
      }
      (None, true) => {
        self.readers.push(account.to_string());
      }
      _ => (),
    }
    self
  }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DataOutput {
  pub key: String,
  pub version: String,
  pub data: Vec<u8>,
  pub authz: Authz,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}
impl InitCallback for InstantiateMsg {
  const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryGet {
  pub permit: Option<Permit>,
  pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  Get(QueryGet),
}
impl Query for QueryMsg {
  const BLOCK_SIZE: usize = 256;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
  Data(Option<DataOutput>),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteSetApplications {
  pub applications: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteStore {
  pub permit: Option<Permit>,
  pub key: String,
  pub version: String,
  pub data: Vec<u8>,
  pub authz: Authz,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteUpdateData {
  pub permit: Option<Permit>,
  pub key: String,
  pub version: String,
  pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteUpdateAuthz {
  pub permit: Option<Permit>,
  pub key: String,
  pub authz: Authz,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ExecuteDelete {
  pub permit: Option<Permit>,
  pub key: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
  SetApplications(ExecuteSetApplications),
  Store(ExecuteStore),
  UpdateData(ExecuteUpdateData),
  UpdateAuthz(ExecuteUpdateAuthz),
  Delete(ExecuteDelete),
}
impl HandleCallback for ExecuteMsg {
  const BLOCK_SIZE: usize = 256;
}
