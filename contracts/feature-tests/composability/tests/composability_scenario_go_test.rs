use multiversx_sc_scenario::imports::*;

fn world() -> ScenarioWorld {
    ScenarioWorld::vm_go()
}

#[test]
fn builtin_func_delete_user_name_go() {
    world().run("scenarios/builtin_func_delete_user_name.scen.json");
}

#[test]
fn builtin_func_set_user_name_go() {
    world().run("scenarios/builtin_func_set_user_name.scen.json");
}

#[test]
fn forw_queue_async_go() {
    world().run("scenarios/forw_queue_async.scen.json");
}

#[test]
fn forw_raw_async_accept_egld_go() {
    world().run("scenarios/forw_raw_async_accept_egld.scen.json");
}

#[test]
fn forw_raw_async_accept_esdt_go() {
    world().run("scenarios/forw_raw_async_accept_esdt.scen.json");
}

#[test]
fn forw_raw_async_echo_go() {
    world().run("scenarios/forw_raw_async_echo.scen.json");
}

#[test]
fn forw_raw_async_send_and_retrieve_multi_transfer_funds_go() {
    world().run("scenarios/forw_raw_async_send_and_retrieve_multi_transfer_funds.scen.json");
}

#[test]
fn forw_raw_builtin_nft_local_mint_via_async_call_go() {
    world().run("scenarios/forw_raw_builtin_nft_local_mint_via_async_call.scen.json");
}

#[test]
fn forw_raw_builtin_nft_local_mint_via_sync_call_go() {
    world().run("scenarios/forw_raw_builtin_nft_local_mint_via_sync_call.scen.json");
}

#[test]
fn forw_raw_call_async_retrieve_multi_transfer_go() {
    world().run("scenarios/forw_raw_call_async_retrieve_multi_transfer.scen.json");
}

#[test]
fn forw_raw_contract_deploy_go() {
    world().run("scenarios/forw_raw_contract_deploy.scen.json");
}

#[test]
fn forw_raw_contract_upgrade_go() {
    world().run("scenarios/forw_raw_contract_upgrade.scen.json");
}

#[test]
fn forw_raw_contract_upgrade_self_go() {
    world().run("scenarios/forw_raw_contract_upgrade_self.scen.json");
}

#[test]
fn forw_raw_direct_egld_go() {
    world().run("scenarios/forw_raw_direct_egld.scen.json");
}

#[test]
fn forw_raw_direct_esdt_go() {
    world().run("scenarios/forw_raw_direct_esdt.scen.json");
}

#[test]
fn forw_raw_direct_multi_esdt_go() {
    world().run("scenarios/forw_raw_direct_multi_esdt.scen.json");
}

#[test]
fn forw_raw_init_async_go() {
    world().run("scenarios/forw_raw_init_async.scen.json");
}

#[test]
fn forw_raw_init_sync_accept_egld_go() {
    world().run("scenarios/forw_raw_init_sync_accept_egld.scen.json");
}

#[test]
fn forw_raw_init_sync_echo_go() {
    world().run("scenarios/forw_raw_init_sync_echo.scen.json");
}

#[test]
fn forw_raw_sync_echo_go() {
    world().run("scenarios/forw_raw_sync_echo.scen.json");
}

#[test]
fn forw_raw_sync_echo_caller_go() {
    world().run("scenarios/forw_raw_sync_echo_caller.scen.json");
}

#[test]
fn forw_raw_sync_egld_go() {
    world().run("scenarios/forw_raw_sync_egld.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_sync_fallible_go() {
    world().run("scenarios/forw_raw_sync_fallible.scen.json");
}

#[test]
fn forw_raw_sync_readonly_go() {
    world().run("scenarios/forw_raw_sync_readonly.scen.json");
}

#[test]
fn forw_raw_sync_reject_go() {
    world().run("scenarios/forw_raw_sync_reject.scen.json");
}

#[test]
fn forw_raw_sync_same_context_go() {
    world().run("scenarios/forw_raw_sync_same_context.scen.json");
}

#[test]
fn forw_raw_sync_same_context_egld_go() {
    world().run("scenarios/forw_raw_sync_same_context_egld.scen.json");
}

#[test]
fn forw_raw_transf_exec_accept_egld_go() {
    world().run("scenarios/forw_raw_transf_exec_accept_egld.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_0_accept_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_0_accept.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_0_reject_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_0_reject.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_egld_accept_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_egld_accept.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_egld_reject_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_egld_reject.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_esdt_accept_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_esdt_accept.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_esdt_reject_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_esdt_reject.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_multi_egld_accept_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_multi_egld_accept.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_multi_egld_reject_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_multi_egld_reject.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_multi_esdt_accept_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_multi_esdt_accept.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_fallible_multi_esdt_reject_go() {
    world().run("scenarios/forw_raw_transf_exec_fallible_multi_esdt_reject.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transf_exec_reject_egld_go() {
    world().run("scenarios/forw_raw_transf_exec_reject_egld.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transfer_fallible_egld_go() {
    world().run("scenarios/forw_raw_transfer_fallible_egld.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transfer_fallible_multi_err_go() {
    world().run("scenarios/forw_raw_transfer_fallible_multi_err.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forw_raw_transfer_fallible_multi_ok_go() {
    world().run("scenarios/forw_raw_transfer_fallible_multi_ok.scen.json");
}

#[test]
fn forwarder_builtin_nft_add_quantity_go() {
    world().run("scenarios/forwarder_builtin_nft_add_quantity.scen.json");
}

#[test]
fn forwarder_builtin_nft_burn_go() {
    world().run("scenarios/forwarder_builtin_nft_burn.scen.json");
}

#[test]
fn forwarder_builtin_nft_create_go() {
    world().run("scenarios/forwarder_builtin_nft_create.scen.json");
}

#[test]
fn forwarder_builtin_nft_local_burn_go() {
    world().run("scenarios/forwarder_builtin_nft_local_burn.scen.json");
}

#[test]
fn forwarder_builtin_nft_local_mint_go() {
    world().run("scenarios/forwarder_builtin_nft_local_mint.scen.json");
}

#[test]
fn forwarder_call_async_accept_egld_go() {
    world().run("scenarios/forwarder_call_async_accept_egld.scen.json");
}

#[test]
fn forwarder_call_async_accept_esdt_go() {
    world().run("scenarios/forwarder_call_async_accept_esdt.scen.json");
}

#[test]
fn forwarder_call_async_accept_nft_go() {
    world().run("scenarios/forwarder_call_async_accept_nft.scen.json");
}

#[test]
fn forwarder_call_async_multi_transfer_go() {
    world().run("scenarios/forwarder_call_async_multi_transfer.scen.json");
}

#[test]
fn forwarder_call_async_multi_transfer_egld_accept_go() {
    world().run("scenarios/forwarder_call_async_multi_transfer_egld_accept.scen.json");
}

#[test]
#[ignore = "mandos bug"]
fn forwarder_call_async_multi_transfer_egld_reject_go() {
    world().run("scenarios/forwarder_call_async_multi_transfer_egld_reject.scen.json");
}

#[test]
fn forwarder_call_async_reject_egld_go() {
    world().run("scenarios/forwarder_call_async_reject_egld.scen.json");
}

#[test]
fn forwarder_call_async_reject_esdt_go() {
    world().run("scenarios/forwarder_call_async_reject_esdt.scen.json");
}

#[test]
fn forwarder_call_async_retrieve_egld_go() {
    world().run("scenarios/forwarder_call_async_retrieve_egld.scen.json");
}

#[test]
fn forwarder_call_async_retrieve_esdt_go() {
    world().run("scenarios/forwarder_call_async_retrieve_esdt.scen.json");
}

#[test]
fn forwarder_call_async_retrieve_nft_go() {
    world().run("scenarios/forwarder_call_async_retrieve_nft.scen.json");
}

#[test]
fn forwarder_call_sync_accept_egld_go() {
    world().run("scenarios/forwarder_call_sync_accept_egld.scen.json");
}

#[test]
fn forwarder_call_sync_accept_esdt_go() {
    world().run("scenarios/forwarder_call_sync_accept_esdt.scen.json");
}

#[test]
fn forwarder_call_sync_accept_multi_transfer_go() {
    world().run("scenarios/forwarder_call_sync_accept_multi_transfer.scen.json");
}

#[test]
fn forwarder_call_sync_accept_nft_go() {
    world().run("scenarios/forwarder_call_sync_accept_nft.scen.json");
}

#[test]
fn forwarder_call_sync_accept_then_read_egld_go() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_egld.scen.json");
}

#[test]
fn forwarder_call_sync_accept_then_read_esdt_go() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_esdt.scen.json");
}

#[test]
fn forwarder_call_sync_accept_then_read_nft_go() {
    world().run("scenarios/forwarder_call_sync_accept_then_read_nft.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forwarder_call_sync_fallible_multi_transfer_egld_accept_go() {
    world().run("scenarios/forwarder_call_sync_fallible_multi_transfer_egld_accept.scen.json");
}

#[test]
#[ignore = "requires Barnard"]
fn forwarder_call_sync_fallible_multi_transfer_egld_reject_go() {
    world().run("scenarios/forwarder_call_sync_fallible_multi_transfer_egld_reject.scen.json");
}

#[test]
fn forwarder_call_sync_multi_transfer_egld_accept_go() {
    world().run("scenarios/forwarder_call_sync_multi_transfer_egld_accept.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_bt_legacy_egld_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_bt_legacy_egld.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_bt_legacy_esdt_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_bt_legacy_esdt.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_bt_legacy_nft_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_bt_legacy_nft.scen.json");
}

#[test]
#[ignore = "TODO: Barnard fixes a bug with this"]
fn forwarder_call_sync_retrieve_bt_multi_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_bt_multi.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_bt_multi_egld_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_bt_multi_egld.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_bt_multi_esdt_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_bt_multi_esdt.scen.json");
}

#[test]
#[ignore = "TODO: Barnard"]
fn forwarder_call_sync_retrieve_bt_multi_twice_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_bt_multi_twice.scen.json");
}

#[test]
#[ignore = "TODO: Barnard"]
fn forwarder_call_sync_retrieve_bt_multi_twice_reset_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_bt_multi_twice_reset.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_egld_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_egld.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_esdt_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_esdt.scen.json");
}

#[test]
fn forwarder_call_sync_retrieve_nft_go() {
    world().run("scenarios/forwarder_call_sync_retrieve_nft.scen.json");
}

#[test]
fn forwarder_call_transf_exec_accept_return_values_go() {
    world().run("scenarios/forwarder_call_transf_exec_accept_return_values.scen.json");
}

#[test]
fn forwarder_call_transf_exec_egld_accept_go() {
    world().run("scenarios/forwarder_call_transf_exec_egld_accept.scen.json");
}

#[test]
fn forwarder_call_transf_exec_egld_accept_twice_go() {
    world().run("scenarios/forwarder_call_transf_exec_egld_accept_twice.scen.json");
}

#[test]
fn forwarder_call_transf_exec_multi_transfer_egld_accept_go() {
    world().run("scenarios/forwarder_call_transf_exec_multi_transfer_egld_accept.scen.json");
}

#[test]
#[ignore = "mandos bug"]
fn forwarder_call_transf_exec_multi_transfer_egld_reject_go() {
    world().run("scenarios/forwarder_call_transf_exec_multi_transfer_egld_reject.scen.json");
}

#[test]
fn forwarder_call_transf_exec_multi_transfer_esdt_accept_go() {
    world().run("scenarios/forwarder_call_transf_exec_multi_transfer_esdt_accept.scen.json");
}

#[test]
fn forwarder_call_transf_exec_multi_transfer_esdt_reject_go() {
    world().run("scenarios/forwarder_call_transf_exec_multi_transfer_esdt_reject.scen.json");
}

#[test]
fn forwarder_call_transf_exec_single_esdt_accept_go() {
    world().run("scenarios/forwarder_call_transf_exec_single_esdt_accept.scen.json");
}

#[test]
fn forwarder_call_transf_exec_single_esdt_accept_twice_go() {
    world().run("scenarios/forwarder_call_transf_exec_single_esdt_accept_twice.scen.json");
}

#[test]
fn forwarder_call_transf_exec_single_nft_accept_go() {
    world().run("scenarios/forwarder_call_transf_exec_single_nft_accept.scen.json");
}

#[test]
fn forwarder_call_transf_exec_single_nft_reject_go() {
    world().run("scenarios/forwarder_call_transf_exec_single_nft_reject.scen.json");
}

#[test]
fn forwarder_call_transf_exec_single_sft_twice_accept_go() {
    world().run("scenarios/forwarder_call_transf_exec_single_sft_twice_accept.scen.json");
}

#[test]
fn forwarder_contract_change_owner_go() {
    world().run("scenarios/forwarder_contract_change_owner.scen.json");
}

#[test]
fn forwarder_contract_deploy_go() {
    world().run("scenarios/forwarder_contract_deploy.scen.json");
}

#[test]
fn forwarder_contract_upgrade_go() {
    world().run("scenarios/forwarder_contract_upgrade.scen.json");
}

#[test]
fn forwarder_get_esdt_local_roles_go() {
    world().run("scenarios/forwarder_get_esdt_local_roles.scen.json");
}

#[test]
fn forwarder_get_esdt_token_data_go() {
    world().run("scenarios/forwarder_get_esdt_token_data.scen.json");
}

#[test]
fn forwarder_nft_add_uri_go() {
    world().run("scenarios/forwarder_nft_add_uri.scen.json");
}

#[test]
fn forwarder_nft_create_go() {
    world().run("scenarios/forwarder_nft_create.scen.json");
}

#[test]
fn forwarder_nft_create_and_send_go() {
    world().run("scenarios/forwarder_nft_create_and_send.scen.json");
}

#[test]
fn forwarder_nft_current_nonce_go() {
    world().run("scenarios/forwarder_nft_current_nonce.scen.json");
}

#[test]
fn forwarder_nft_decode_complex_attributes_go() {
    world().run("scenarios/forwarder_nft_decode_complex_attributes.scen.json");
}

#[test]
fn forwarder_nft_transfer_async_go() {
    world().run("scenarios/forwarder_nft_transfer_async.scen.json");
}

#[test]
fn forwarder_nft_transfer_exec_go() {
    world().run("scenarios/forwarder_nft_transfer_exec.scen.json");
}

#[test]
fn forwarder_nft_update_attributes_go() {
    world().run("scenarios/forwarder_nft_update_attributes.scen.json");
}

#[test]
fn forwarder_no_endpoint_go() {
    world().run("scenarios/forwarder_no_endpoint.scen.json");
}

#[test]
fn forwarder_retrieve_funds_with_accept_func_go() {
    world().run("scenarios/forwarder_retrieve_funds_with_accept_func.scen.json");
}

#[test]
fn forwarder_send_esdt_multi_transfer_go() {
    world().run("scenarios/forwarder_send_esdt_multi_transfer.scen.json");
}

#[test]
fn forwarder_sync_echo_go() {
    world().run("scenarios/forwarder_sync_echo.scen.json");
}

#[test]
fn forwarder_transfer_esdt_with_fees_go() {
    world().run("scenarios/forwarder_transfer_esdt_with_fees.scen.json");
}

#[test]
fn forwarder_validate_token_identifier_go() {
    world().run("scenarios/forwarder_validate_token_identifier.scen.json");
}

#[test]
fn promises_call_async_accept_egld_go() {
    world().run("scenarios/promises_call_async_accept_egld.scen.json");
}

#[test]
fn promises_call_async_accept_esdt_go() {
    world().run("scenarios/promises_call_async_accept_esdt.scen.json");
}

#[test]
#[ignore = "TODO"]
fn promises_call_async_retrieve_egld_go() {
    world().run("scenarios/promises_call_async_retrieve_egld.scen.json");
}

#[test]
#[ignore = "TODO"]
fn promises_call_async_retrieve_esdt_go() {
    world().run("scenarios/promises_call_async_retrieve_esdt.scen.json");
}

#[test]
#[ignore = "TODO"]
fn promises_call_callback_directly_go() {
    world().run("scenarios/promises_call_callback_directly.scen.json");
}

#[test]
fn promises_call_transfer_callback_egld_go() {
    world().run("scenarios/promises_call_transfer_callback_egld.scen.json");
}

#[test]
fn promises_call_transfer_callback_esdt_go() {
    world().run("scenarios/promises_call_transfer_callback_esdt.scen.json");
}

#[test]
#[ignore = "TODO"]
fn promises_multi_transfer_go() {
    world().run("scenarios/promises_multi_transfer.scen.json");
}

#[test]
fn promises_multi_transfer_err_go() {
    world().run("scenarios/promises_multi_transfer_err.scen.json");
}

#[test]
fn promises_single_transfer_go() {
    world().run("scenarios/promises_single_transfer.scen.json");
}

#[test]
#[ignore = "gas"]
fn promises_single_transfer_gas_1_go() {
    world().run("scenarios/promises_single_transfer_gas1.scen.json");
}

#[test]
#[ignore = "gas"]
fn promises_single_transfer_gas_2_go() {
    world().run("scenarios/promises_single_transfer_gas2.scen.json");
}

#[test]
fn proxy_test_init_go() {
    world().run("scenarios/proxy_test_init.scen.json");
}

#[test]
fn proxy_test_message_other_shard_go() {
    world().run("scenarios/proxy_test_message_otherShard.scen.json");
}

#[test]
fn proxy_test_message_other_shard_callback_go() {
    world().run("scenarios/proxy_test_message_otherShard_callback.scen.json");
}

#[test]
fn proxy_test_message_same_shard_go() {
    world().run("scenarios/proxy_test_message_sameShard.scen.json");
}

#[test]
fn proxy_test_message_same_shard_callback_go() {
    world().run("scenarios/proxy_test_message_sameShard_callback.scen.json");
}

#[test]
fn proxy_test_payment_other_shard_go() {
    world().run("scenarios/proxy_test_payment_otherShard.scen.json");
}

#[test]
fn proxy_test_payment_other_shard_callback_go() {
    world().run("scenarios/proxy_test_payment_otherShard_callback.scen.json");
}

#[test]
fn proxy_test_payment_same_shard_go() {
    world().run("scenarios/proxy_test_payment_sameShard.scen.json");
}

#[test]
fn proxy_test_payment_same_shard_callback_go() {
    world().run("scenarios/proxy_test_payment_sameShard_callback.scen.json");
}

#[test]
fn proxy_test_upgrade_go() {
    world().run("scenarios/proxy_test_upgrade.scen.json");
}

#[test]
fn recursive_caller_egld_1_go() {
    world().run("scenarios/recursive_caller_egld_1.scen.json");
}

#[test]
fn recursive_caller_esdt_1_go() {
    world().run("scenarios/recursive_caller_esdt_1.scen.json");
}

#[test]
fn send_egld_go() {
    world().run("scenarios/send_egld.scen.json");
}

#[test]
fn send_esdt_go() {
    world().run("scenarios/send_esdt.scen.json");
}

#[test]
fn send_esdt_to_nonexisting_account_go() {
    world().run("scenarios/send_esdt_to_nonexisting_account.scen.json");
}
