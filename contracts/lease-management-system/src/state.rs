use cosmwasm_std::{Addr, Uint128};
use cw0::Expiration;
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone, PartialEq)]
pub struct FlatInfo {
    pub renter: String,
    pub rentee: Option<String>,
    pub rent: Uint128,
    pub expires: Option<Expiration>,
}


pub const FLAT_LIST: Item<Vec<FlatInfo>>= Item::new("flatlist");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const DENOM: Item<String> = Item::new("denom");
pub const RENTER_TO_FLAT_ID: Map<&Addr,Vec<usize>>= Map::new("renter_to_flatid");



