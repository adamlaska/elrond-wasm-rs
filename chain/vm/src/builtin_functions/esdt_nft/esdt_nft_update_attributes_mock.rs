use multiversx_chain_core::types::ReturnCode;

use crate::{
    chain_core::builtin_func_names::ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME,
    host::context::{BlockchainUpdate, TxCache, TxInput, TxLog, TxResult},
    host::runtime::{RuntimeInstanceCallLambda, RuntimeRef},
    types::{top_decode_u64, top_encode_u64},
};

use super::super::builtin_func_trait::BuiltinFunction;

pub struct ESDTNftUpdateAttributes;

impl BuiltinFunction for ESDTNftUpdateAttributes {
    fn name(&self) -> &str {
        ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME
    }

    fn execute<F>(
        &self,
        tx_input: TxInput,
        tx_cache: TxCache,
        _runtime: &RuntimeRef,
        _f: F,
    ) -> (TxResult, BlockchainUpdate)
    where
        F: RuntimeInstanceCallLambda,
    {
        if tx_input.args.len() != 3 {
            let err_result = TxResult::from_vm_error("ESDTNFTUpdateAttributes expects 3 arguments");
            return (err_result, BlockchainUpdate::empty());
        }

        let token_identifier = tx_input.args[0].as_slice();
        let nonce = top_decode_u64(tx_input.args[1].as_slice());
        let attributes_bytes = tx_input.args[2].as_slice();

        tx_cache.with_account_mut(&tx_input.from, |account| {
            account
                .esdt
                .update_attributes(token_identifier, nonce, attributes_bytes.to_vec());
        });

        let esdt_nft_create_log = TxLog {
            address: tx_input.from,
            endpoint: ESDT_NFT_UPDATE_ATTRIBUTES_FUNC_NAME.into(),
            topics: vec![
                token_identifier.to_vec(),
                top_encode_u64(nonce),
                Vec::new(), // value = 0
                attributes_bytes.to_vec(),
            ],
            data: vec![],
        };

        let tx_result = TxResult {
            result_status: ReturnCode::Success,
            result_logs: vec![esdt_nft_create_log],
            ..Default::default()
        };

        (tx_result, tx_cache.into_blockchain_updates())
    }
}
