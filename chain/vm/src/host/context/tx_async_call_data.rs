use crate::{
    builtin_functions::BuiltinFunctionContainer,
    host::context::{TxInput, TxResult},
    types::{top_encode_u64, VMAddress, H256},
};

use num_bigint::BigUint;

use super::{CallType, CallbackPayments, Promise, TxFunctionName};

#[derive(Debug, Clone)]
pub struct AsyncCallTxData {
    pub from: VMAddress,
    pub to: VMAddress,
    pub call_value: BigUint,
    pub endpoint_name: TxFunctionName,
    pub arguments: Vec<Vec<u8>>,
    pub tx_hash: H256,
}

pub fn async_call_tx_input(async_call: &AsyncCallTxData, call_type: CallType) -> TxInput {
    TxInput {
        from: async_call.from.clone(),
        to: async_call.to.clone(),
        egld_value: async_call.call_value.clone(),
        esdt_values: Vec::new(),
        func_name: async_call.endpoint_name.clone(),
        args: async_call.arguments.clone(),
        call_type,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: async_call.tx_hash.clone(),
        ..Default::default()
    }
}

fn result_status_bytes(result_status: u64) -> Vec<u8> {
    if result_status == 0 {
        vec![0x00]
    } else {
        top_encode_u64(result_status)
    }
}

fn real_recipient(
    async_data: &AsyncCallTxData,
    builtin_functions: &BuiltinFunctionContainer,
) -> VMAddress {
    let tx_input = async_call_tx_input(async_data, CallType::AsyncCall);
    let transfers = builtin_functions.extract_token_transfers(&tx_input);
    transfers.real_recipient
}

pub fn async_callback_tx_input(
    async_data: &AsyncCallTxData,
    async_result: &TxResult,
    builtin_functions: &BuiltinFunctionContainer,
) -> TxInput {
    let mut args: Vec<Vec<u8>> = vec![result_status_bytes(async_result.result_status.as_u64())];
    if async_result.result_status.is_success() {
        args.extend_from_slice(async_result.result_values.as_slice());
    } else {
        args.push(async_result.result_message.clone().into_bytes());
    }
    let callback_payments =
        extract_callback_payments(&async_data.from, async_result, builtin_functions);
    TxInput {
        from: real_recipient(async_data, builtin_functions),
        to: async_data.from.clone(),
        egld_value: 0u32.into(),
        esdt_values: Vec::new(),
        func_name: TxFunctionName::CALLBACK,
        args,
        call_type: CallType::AsyncCallback,
        gas_limit: 1000,
        gas_price: 0,
        tx_hash: async_data.tx_hash.clone(),
        callback_payments,
        ..Default::default()
    }
}

fn extract_callback_payments(
    callback_contract_address: &VMAddress,
    async_result: &TxResult,
    builtin_functions: &BuiltinFunctionContainer,
) -> CallbackPayments {
    let mut callback_payments = CallbackPayments::default();
    for async_call in &async_result.all_calls {
        let tx_input = async_call_tx_input(async_call, CallType::AsyncCall);
        let token_transfers = builtin_functions.extract_token_transfers(&tx_input);
        if &token_transfers.real_recipient == callback_contract_address {
            if !token_transfers.is_empty() {
                callback_payments.esdt_values = token_transfers.transfers;
            } else {
                callback_payments
                    .egld_value
                    .clone_from(&async_call.call_value);
            }
            break;
        }
    }
    callback_payments
}

pub fn async_promise_callback_tx_input(
    promise: &Promise,
    async_result: &TxResult,
    builtin_functions: &BuiltinFunctionContainer,
) -> TxInput {
    let callback_name = if async_result.result_status.is_success() {
        promise.success_callback.clone()
    } else {
        promise.error_callback.clone()
    };

    let mut callback_input =
        async_callback_tx_input(&promise.call, async_result, builtin_functions);
    callback_input.func_name = callback_name;
    callback_input.promise_callback_closure_data = Some(promise.callback_closure_data.clone());
    callback_input
}

pub fn merge_results(mut original: TxResult, mut new: TxResult) -> TxResult {
    if original.result_status.is_success() {
        original.result_values.append(&mut new.result_values);
        original.result_logs.append(&mut new.result_logs);
        original.result_message = new.result_message;
        original
    } else {
        new
    }
}
