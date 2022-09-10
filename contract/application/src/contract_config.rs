use crate::defs;
use cosmwasm_std::{Addr, StdError, StdResult, Storage};
use cosmwasm_storage::{singleton, singleton_read};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Deserialize, Clone, PartialEq)]
pub struct ContractConfig {
  pub my_address: Addr,
  pub owner_address: Addr,
  pub storage_address: Addr,
  pub storage_codehash: String,
}

impl ContractConfig {
  pub fn check_owner(&self, addr: &Addr) -> StdResult<()> {
    if &self.owner_address != addr {
      return Err(StdError::generic_err(format!("not a owner")));
    }
    Ok(())
  }
  pub fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
    singleton::<Self>(storage, defs::CONTRACT_CONFIG_KEY_B).save(self)?;
    Ok(())
  }
  pub fn load(storage: &dyn Storage) -> StdResult<Self> {
    singleton_read::<Self>(storage, defs::CONTRACT_CONFIG_KEY_B).load()
  }
}
