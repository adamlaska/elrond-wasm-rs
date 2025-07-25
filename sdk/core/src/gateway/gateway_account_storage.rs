use crate::data::account_storage::AccountStorageResponse;
use anyhow::anyhow;
use multiversx_chain_core::std::Bech32Address;
use std::collections::HashMap;

use super::{GatewayRequest, GatewayRequestType, ACCOUNT_ENDPOINT, KEYS_ENDPOINT};

/// Retrieves an account storage from the network.
pub struct GetAccountStorageRequest<'a> {
    pub address: &'a Bech32Address,
}

impl<'a> GetAccountStorageRequest<'a> {
    pub fn new(address: &'a Bech32Address) -> Self {
        Self { address }
    }
}

impl GatewayRequest for GetAccountStorageRequest<'_> {
    type Payload = ();
    type DecodedJson = AccountStorageResponse;
    type Result = HashMap<String, String>;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("{ACCOUNT_ENDPOINT}/{}/{KEYS_ENDPOINT}", self.address.bech32)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.pairs),
        }
    }
}
