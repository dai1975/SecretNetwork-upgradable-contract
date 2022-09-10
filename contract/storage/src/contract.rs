use cosmwasm_std::{
  entry_point, to_binary, Addr, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError,
  StdResult,
};
use secret_toolkit::permit::validate;
//use std::cmp::max;

use crate::contract_config::ContractConfig;
use crate::{defs, msg, state};

fn authenticate(
  deps: Deps,
  config: &ContractConfig,
  permit: Option<&msg::Permit>,
) -> StdResult<Option<Addr>> {
  if permit.is_none() {
    return Ok(None);
  }
  let permit = permit.unwrap();
  if permit.params.allowed_tokens.len() == 0 {
    return Err(StdError::generic_err("no allowed_tokens"));
  }

  // check allowed_tokens is one of applications address
  let current_token_address = config
    .applications_address
    .iter()
    .find(|a| permit.check_token(a.as_str()));
  if current_token_address.is_none() {
    return Err(StdError::generic_err("no allowed address"));
  }

  let addr_s = validate(
    deps,
    defs::PREFIX_REVOKED_PERMIT,
    &permit,
    current_token_address.unwrap().to_string(),
    None,
  )?;
  let addr_h = deps.api.addr_validate(addr_s.as_str())?;
  Ok(Some(addr_h))
}

#[entry_point]
pub fn instantiate(
  deps: DepsMut,
  env: Env,
  info: MessageInfo,
  _msg: msg::InstantiateMsg,
) -> StdResult<Response> {
  let config = ContractConfig {
    my_address: env.contract.address,
    owner_address: info.sender,
    applications_address: Vec::new(),
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
    msg::ExecuteMsg::SetApplications(m) => {
      let mut c = config.clone();
      c.applications_address = m
        .applications
        .iter()
        .map(|s| deps.api.addr_validate(s.as_str()))
        .collect::<Result<Vec<_>, _>>()?;
      c.save(deps.storage)?;
      Ok(Response::new())
    }
    msg::ExecuteMsg::Store(m) => {
      let authn = authenticate(deps.as_ref(), &config, m.permit.as_ref())?;
      state::store(deps, env, info, authn, m)
    }
    msg::ExecuteMsg::UpdateData(m) => {
      let authn = authenticate(deps.as_ref(), &config, m.permit.as_ref())?;
      state::update_data(deps, env, info, authn, m)
    }
    msg::ExecuteMsg::UpdateAuthz(m) => {
      let authn = authenticate(deps.as_ref(), &config, m.permit.as_ref())?;
      state::update_authz(deps, env, info, authn, m)
    }
    msg::ExecuteMsg::Delete(m) => {
      let authn = authenticate(deps.as_ref(), &config, m.permit.as_ref())?;
      state::delete(deps, env, info, authn, m)
    }
  }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg) -> StdResult<QueryResponse> {
  let config = ContractConfig::load(deps.storage)?;
  let r: StdResult<msg::QueryAnswer> = match msg {
    msg::QueryMsg::Get(m) => {
      let authn = authenticate(deps, &config, m.permit.as_ref())?;
      state::get(deps, env, authn, m)
    }
  };
  r.and_then(|a| to_binary(&a))
}
/*
pub fn try_submit_net_worth(
  deps: DepsMut,
  name: String,
  worth: u64,
) -> Result<Response, CustomContractError> {
  let mut state = config(deps.storage).load()?;

  match state.state {
    msg::ContractState::Init => {
      state.player1 = Millionaire::new(name, worth);
      state.state = ContractState::Got1;
    }
    msg::ContractState::Got1 => {
      state.player2 = Millionaire::new(name, worth);
      state.state = ContractState::Done;
    }
    msg::ContractState::Done => {
      return Err(CustomContractError::AlreadyAddedBothMillionaires);
    }
  }

  config(deps.storage).save(&state)?;

  Ok(Response::new())
}

pub fn try_reset(deps: DepsMut) -> Result<Response, CustomContractError> {
  let mut state = config(deps.storage).load()?;

  state.state = ContractState::Init;
  config(deps.storage).save(&state)?;

  Ok(Response::new().add_attribute("action", "reset state"))
}

fn query_who_is_richer(deps: Deps) -> StdResult<RicherResponse> {
  let state = config_read(deps.storage).load()?;

  if state.state != ContractState::Done {
    return Err(StdError::generic_err(
      "Can't tell who is richer unless we get 2 data points!",
    ));
  }

  if state.player1 == state.player2 {
    let resp = RicherResponse {
      richer: "It's a tie!".to_string(),
    };

    return Ok(resp);
  }

  let richer = max(state.player1, state.player2);

  let resp = RicherResponse {
    // we use .clone() here because ...
    richer: richer.name().clone(),
  };

  Ok(resp)
}
*/
#[cfg(test)]
mod tests {
  use super::*;

  use cosmwasm_std::coins;
  use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
  /*
    #[test]
    fn proper_instantialization() {
      let mut deps = mock_dependencies();

      let msg = InstantiateMsg {};
      let info = mock_info("creator", &coins(1000, "earth"));

      // we can just call .unwrap() to assert this was a success
      let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
      assert_eq!(0, res.messages.len());

      // it worked, let's query the state
      let _ = query_who_is_richer(deps.as_ref()).unwrap_err();
    }

    #[test]
    fn solve_millionaire() {
      let mut deps = mock_dependencies();

      let msg = InstantiateMsg {};
      let info = mock_info("creator", &coins(2, "token"));
      let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

      let msg_player1 = ExecuteMsg::SubmitNetWorth {
        worth: 1,
        name: "alice".to_string(),
      };
      let msg_player2 = ExecuteMsg::SubmitNetWorth {
        worth: 2,
        name: "bob".to_string(),
      };

      let info = mock_info("creator", &[]);

      let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg_player1).unwrap();
      let _res = execute(deps.as_mut(), mock_env(), info, msg_player2).unwrap();

      // it worked, let's query the state
      let value = query_who_is_richer(deps.as_ref()).unwrap();

      assert_eq!(&value.richer, "bob")
    }

    #[test]
    fn test_reset_state() {
      let mut deps = mock_dependencies();

      let msg = InstantiateMsg {};
      let info = mock_info("creator", &coins(2, "token"));
      let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

      let msg_player1 = ExecuteMsg::SubmitNetWorth {
        worth: 1,
        name: "alice".to_string(),
      };

      let info = mock_info("creator", &[]);
      let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg_player1).unwrap();

      let reset_msg = ExecuteMsg::Reset {};
      let _res = execute(deps.as_mut(), mock_env(), info.clone(), reset_msg).unwrap();

      let msg_player2 = ExecuteMsg::SubmitNetWorth {
        worth: 2,
        name: "bob".to_string(),
      };
      let msg_player3 = ExecuteMsg::SubmitNetWorth {
        worth: 3,
        name: "carol".to_string(),
      };

      let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg_player2).unwrap();
      let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg_player3).unwrap();

      // it worked, let's query the state
      let value = query_who_is_richer(deps.as_ref()).unwrap();

      assert_eq!(&value.richer, "carol")
    }
  */
}
