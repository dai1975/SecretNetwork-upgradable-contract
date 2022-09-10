use crate::{defs, msg};
use cosmwasm_std::{Addr, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Storage};
use cosmwasm_storage::{bucket, bucket_read, Bucket, ReadonlyBucket};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Data {
  version: String,
  data: Vec<u8>,
  authz: msg::Authz,
}

fn bucket_reader<'a>(storage: &'a dyn Storage) -> ReadonlyBucket<'a, Data> {
  bucket_read::<Data>(storage, defs::DATA_BUCKET_KEY)
}
fn bucket_writer<'a>(storage: &'a mut dyn Storage) -> Bucket<'a, Data> {
  bucket::<Data>(storage, defs::DATA_BUCKET_KEY)
}

pub fn store(
  deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  authn: Option<Addr>,
  msg: msg::ExecuteStore,
) -> StdResult<Response> {
  if authn.is_none() {
    return Err(StdError::generic_err("unauthorized"));
  }
  let mut bkt = bucket_writer(deps.storage);

  let key = msg.key.as_bytes();
  if let Some(mut _data) = bkt.may_load(key)? {
    return Err(StdError::generic_err("alrady exists"));
  } else {
    let data = Data {
      version: msg.version,
      data: msg.data,
      authz: msg.authz.update_owner(authn.unwrap().as_str()),
    };
    bkt.save(msg.key.as_bytes(), &data)?;
    Ok(Response::new())
  }
}

pub fn delete(
  deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  authn: Option<Addr>,
  msg: msg::ExecuteDelete,
) -> StdResult<Response> {
  if authn.is_none() {
    return Err(StdError::generic_err("unauthorized"));
  }
  let mut bkt = bucket_writer(deps.storage);
  let key = msg.key.as_bytes();
  if let Some(data) = bkt.may_load(key)? {
    if !data.authz.is_owner(authn.unwrap().as_str()) {
      return Err(StdError::generic_err("not a owner"));
    }
    bkt.remove(key);
  } else {
    return Err(StdError::generic_err("not found"));
  }
  Ok(Response::new())
}

pub fn update_data(
  deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  authn: Option<Addr>,
  msg: msg::ExecuteUpdateData,
) -> StdResult<Response> {
  if authn.is_none() {
    return Err(StdError::generic_err("unauthorized"));
  }
  let mut bkt = bucket_writer(deps.storage);
  let key = msg.key.as_bytes();
  if let Some(mut data) = bkt.may_load(key)? {
    if !data.authz.is_owner(authn.unwrap().as_str()) {
      return Err(StdError::generic_err("not a owner"));
    }
    data.version = msg.version;
    data.data = msg.data;
    bkt.save(key, &data)?;
  } else {
    return Err(StdError::generic_err("not found"));
  }
  Ok(Response::new())
}

pub fn update_authz(
  deps: DepsMut,
  _env: Env,
  _info: MessageInfo,
  authn: Option<Addr>,
  msg: msg::ExecuteUpdateAuthz,
) -> StdResult<Response> {
  if authn.is_none() {
    return Err(StdError::generic_err("unauthorized"));
  }
  let mut bkt = bucket_writer(deps.storage);
  let key = msg.key.as_bytes();
  if let Some(mut data) = bkt.may_load(key)? {
    if !data.authz.is_owner(authn.unwrap().as_str()) {
      return Err(StdError::generic_err("not a owner"));
    }
    data.authz = msg.authz;
    bkt.save(key, &data)?;
  } else {
    return Err(StdError::generic_err("not found"));
  }
  Ok(Response::new())
}

pub fn get(
  deps: Deps,
  _env: Env,
  authn: Option<Addr>,
  msg: msg::QueryGet,
) -> StdResult<msg::QueryAnswer> {
  if authn.is_none() {
    return Err(StdError::generic_err("unauthorized"));
  }
  let bkt = bucket_reader(deps.storage);
  let key = msg.key.as_bytes();
  if let Some(data) = bkt.may_load(key)? {
    if !data.authz.is_readable(authn.unwrap().as_str()) {
      return Err(StdError::generic_err("unauthorized"));
    }
    let out = msg::DataOutput {
      key: msg.key,
      version: data.version,
      data: data.data,
      authz: data.authz,
    };
    Ok(msg::QueryAnswer::Data(Some(out)))
  } else {
    Ok(msg::QueryAnswer::Data(None))
  }
}
