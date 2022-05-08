use cosmwasm_std::{
    to_binary, Addr, Coin, CosmosMsg, Deps, StdError, StdResult,Uint128, WasmMsg};
use cw20::Cw20ExecuteMsg;
use schemars::JsonSchema;
use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AnchorMsg {
    DepositStable {},
    RedeemStable {},
}

pub fn anchor_deposit_msg<T: Clone + fmt::Debug + PartialEq + JsonSchema>(
    _deps: Deps,
    anchor_money_market_address: Addr,
    amount: Coin,
) -> StdResult<CosmosMsg<T>> {
    if amount.denom != "uusd" {
        return Err(StdError::generic_err(
            "Wrong currency. Only UST (denom: uusd) is supported.",
        ));
    }

    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: anchor_money_market_address.to_string(),
        msg: to_binary(&AnchorMsg::DepositStable {})?,
        funds: vec![amount],
    }))
}

pub fn anchor_withdraw_msg<T: Clone + fmt::Debug + PartialEq + JsonSchema>(
    aust_address: Addr,
    anchor_money_market_address: Addr,
    amount: Uint128,
) -> StdResult<CosmosMsg<T>> {
    Ok(CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: aust_address.to_string(),
        msg: to_binary(&Cw20ExecuteMsg::Send {
            contract: anchor_money_market_address.to_string(),
            amount,
            msg: to_binary(&AnchorMsg::RedeemStable {})?,
        })?,
        funds: vec![],
    }))
}