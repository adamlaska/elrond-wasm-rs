use crate::data::account::{Account, AccountResponse};
use anyhow::anyhow;
use multiversx_chain_core::std::Bech32Address;

use super::ACCOUNT_ENDPOINT;
use super::{GatewayRequest, GatewayRequestType};

/// Retrieves an account info from the network (nonce, balance).
pub struct GetAccountRequest<'a> {
    pub address: &'a Bech32Address,
}

impl<'a> GetAccountRequest<'a> {
    pub fn new(address: &'a Bech32Address) -> Self {
        Self { address }
    }
}

impl GatewayRequest for GetAccountRequest<'_> {
    type Payload = ();
    type DecodedJson = AccountResponse;
    type Result = Account;

    fn request_type(&self) -> GatewayRequestType {
        GatewayRequestType::Get
    }

    fn get_endpoint(&self) -> String {
        format!("{ACCOUNT_ENDPOINT}/{}", self.address.bech32)
    }

    fn process_json(&self, decoded: Self::DecodedJson) -> anyhow::Result<Self::Result> {
        match decoded.data {
            None => Err(anyhow!("{}", decoded.error)),
            Some(b) => Ok(b.account),
        }
    }
}
