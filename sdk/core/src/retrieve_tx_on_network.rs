use crate::{
    data::transaction::{Events, TransactionOnNetwork},
    gateway::{GetTxInfo, GetTxProcessStatus},
    utils::base64_encode,
};
use log::info;
use multiversx_chain_core::types::ReturnCode;

use crate::gateway::GatewayAsyncService;

const INITIAL_BACKOFF_DELAY: u64 = 1400;
const MAX_RETRIES: usize = 8;
const MAX_BACKOFF_DELAY: u64 = 6000;
const LOG_IDENTIFIER_SIGNAL_ERROR: &str = "signalError";

/// Retrieves a transaction from the network.
pub async fn retrieve_tx_on_network<GatewayProxy: GatewayAsyncService>(
    proxy: &GatewayProxy,
    tx_hash: String,
) -> (TransactionOnNetwork, ReturnCode) {
    let mut retries = 0;
    let mut backoff_delay = INITIAL_BACKOFF_DELAY;
    let start_time = proxy.now();

    loop {
        match proxy.request(GetTxProcessStatus::new(&tx_hash)).await {
            Ok((status, reason)) => {
                // checks if transaction status is final
                match status.as_str() {
                    "success" => {
                        // retrieve transaction info with results
                        let transaction_info_with_results = proxy
                            .request(GetTxInfo::new(&tx_hash).with_results())
                            .await
                            .unwrap();

                        info!(
                            "Transaction retrieved successfully, with status {}: {:#?}",
                            status, transaction_info_with_results
                        );
                        return (transaction_info_with_results, ReturnCode::Success);
                    },
                    "fail" => {
                        let (error_code, error_message) = parse_reason(&reason);
                        let mut transaction_info_with_results: TransactionOnNetwork = proxy
                            .request(GetTxInfo::new(&tx_hash).with_results())
                            .await
                            .unwrap();
                        replace_with_error_message(
                            &mut transaction_info_with_results,
                            error_message,
                        );

                        info!(
                            "Transaction retrieved successfully, with status {}: {:#?}",
                            status, transaction_info_with_results
                        );
                        return (transaction_info_with_results, error_code);
                    },
                    _ => {
                        continue;
                    },
                }
            },
            Err(err) => {
                retries += 1;
                if retries >= MAX_RETRIES {
                    info!("Transaction failed, max retries exceeded: {}", err);
                    println!("Transaction failed, max retries exceeded: {}", err);
                    break;
                }

                let backoff_time = backoff_delay.min(MAX_BACKOFF_DELAY);
                proxy.sleep(backoff_time).await;
                backoff_delay *= 2; // exponential backoff
            },
        }
    }

    // retries have been exhausted
    println!(
            "Fetching transaction failed and retries exhausted, returning default transaction. Total elapsed time: {:?}s",
            proxy.elapsed_seconds(&start_time)
        );
    (TransactionOnNetwork::default(), ReturnCode::UserError)
}

pub fn parse_reason(reason: &str) -> (ReturnCode, String) {
    if reason.is_empty() {
        return (ReturnCode::UserError, String::new());
    }

    let (code, mut message) = find_code_and_message(reason);

    match code {
        Some(return_code) => {
            if message.is_empty() {
                message = ReturnCode::message(return_code).to_owned();
            }

            (return_code, base64_encode(message))
        },
        None => {
            if message.is_empty() {
                message = extract_message_from_string_reason(reason);
            }
            let return_code = ReturnCode::from_message(&message).unwrap_or(ReturnCode::UserError);

            (return_code, base64_encode(message))
        },
    }
}

pub fn find_code_and_message(reason: &str) -> (Option<ReturnCode>, String) {
    let mut error_code: Option<ReturnCode> = None;
    let mut error_message: String = String::new();
    let parts: Vec<&str> = reason.split('@').filter(|part| !part.is_empty()).collect();

    for part in &parts {
        if let Ok(code) = u64::from_str_radix(part, 16) {
            if error_code.is_none() {
                error_code = ReturnCode::from_u64(code);
            }
        } else if let Ok(hex_decode_error_message) = hex::decode(part) {
            if let Ok(str) = String::from_utf8(hex_decode_error_message.clone()) {
                error_message = str;
            }
        }
    }

    (error_code, error_message)
}

pub fn extract_message_from_string_reason(reason: &str) -> String {
    let contract_error: Vec<&str> = reason.split('[').filter(|part| !part.is_empty()).collect();
    if contract_error.len() == 3 {
        let message: Vec<&str> = contract_error[1].split(']').collect();
        return message[0].to_string();
    }

    return contract_error.last().unwrap_or(&"").split(']').collect();
}

pub fn replace_with_error_message(tx: &mut TransactionOnNetwork, error_message: String) {
    if error_message.is_empty() {
        return;
    }

    if let Some(event) = find_log(tx) {
        if let Some(event_topics) = event.topics.as_mut() {
            if event_topics.len() == 2 {
                event_topics[1] = error_message;
            }
        }
    }
}

fn find_log(tx: &mut TransactionOnNetwork) -> Option<&mut Events> {
    if let Some(logs) = tx.logs.as_mut() {
        logs.events
            .iter_mut()
            .find(|event| event.identifier == LOG_IDENTIFIER_SIGNAL_ERROR)
    } else {
        None
    }
}
