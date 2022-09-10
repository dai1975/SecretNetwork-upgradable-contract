use cosmwasm_std::{Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult};

use crate::storage; //use upgradable_contract__storage::msg as storage;
use secret_toolkit::utils::calls::{HandleCallback, Query};

use crate::contract_config::ContractConfig;
use crate::msg;

struct Data {
  value: u32,
}
fn format_data(data: &Data) -> (String, Vec<u8>) {
  ("1".to_string(), Vec::from(data.value.to_be_bytes()))
}
fn parse_data(out: &storage::DataOutput) -> StdResult<Data> {
  match out.version.as_str() {
    "1" => {
      let bytes: [u8; 4] = [out.data[0], out.data[1], out.data[2], out.data[3]];
      Ok(Data {
        value: u32::from_be_bytes(bytes),
      })
    }
    _ => Err(StdError::generic_err("unknown format version")),
  }
}

pub fn set(
  _deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  config: ContractConfig,
  permit: Option<storage::Permit>,
  key: String,
  value: u32,
  authz: msg::Authz,
) -> StdResult<Response> {
  let f = format_data(&Data { value: value });
  let storage_authz = match authz {
    //owner is automatically set in storage contract
    msg::Authz::PUBLIC => storage::Authz::new("", true),
    msg::Authz::PRIVATE => storage::Authz::new("", false),
    msg::Authz::PROTECTED(u) => storage::Authz::new("", false).update(u.as_str(), true),
  };
  let msg = storage::ExecuteMsg::Store(storage::ExecuteStore {
    permit: permit,
    key: key,
    version: f.0,
    data: f.1,
    authz: storage_authz,
  });
  let res = Response::new().add_message(msg.to_cosmos_msg(
    config.storage_codehash,
    config.storage_address.to_string(),
    None,
  )?);
  Ok(res)
}

pub fn get(
  deps: Deps,
  _env: Env,
  config: ContractConfig,
  permit: Option<storage::Permit>,
  key: String,
) -> StdResult<msg::QueryAnswer> {
  let msg = storage::QueryMsg::Get(storage::QueryGet {
    permit: permit,
    key: key,
  });
  match msg.query(
    deps.querier,
    config.storage_codehash,
    config.storage_address.to_string(),
  )? {
    storage::QueryAnswer::Data(Some(o)) => {
      let data = parse_data(&o)?;
      Ok(msg::QueryAnswer::Value(Some(data.value)))
    }
    storage::QueryAnswer::Data(None) => Ok(msg::QueryAnswer::Value(None)),
    /*
    _ => {
      Err(StdError::generic_err("unexpected response")),
    }
     */
  }
}
