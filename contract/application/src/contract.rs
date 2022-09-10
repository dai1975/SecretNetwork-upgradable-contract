use cosmwasm_std::{
  entry_point, to_binary, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdResult,
};

use crate::contract_config::ContractConfig;
use crate::{msg, state};

#[entry_point]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: msg::InstantiateMsg,
) -> StdResult<Response> {
  let config = ContractConfig {
    my_address: env.contract.address,
    owner_address: info.sender,
    storage_address: deps.api.addr_validate(msg.storage_address.as_str())?,
    storage_codehash: msg.storage_codehash,
  };
  config.save(deps.storage)?;

  Ok(Response::default())
}

#[entry_point]
pub fn execute(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  msg: msg::ExecuteMsg,
) -> StdResult<Response> {
  let config = ContractConfig::load(deps.storage)?;
  match msg {
    msg::ExecuteMsg::Set {
      permit,
      key,
      value,
      authz,
    } => state::set(deps, env, info, config, permit, key, value, authz),
  }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<QueryResponse> {
  let config = ContractConfig::load(deps.storage)?;
  let r: StdResult<msg::QueryAnswer> = match msg {
    msg::QueryMsg::Get { permit, key } => state::get(deps, env, config, permit, key),
  };
  r.and_then(|a| to_binary(&a))
}
