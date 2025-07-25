#![no_std]
#![allow(clippy::type_complexity)]

mod common;
pub mod forwarder_proxy;
pub mod fwd_call_async;
pub mod fwd_call_promise_direct;
pub mod fwd_call_promises;
pub mod fwd_call_promises_bt;
pub mod fwd_call_sync;
pub mod fwd_call_sync_bt;
pub mod fwd_call_sync_bt_legacy;
pub mod fwd_call_transf_exec;
pub mod fwd_change_owner;
pub mod fwd_deploy;
pub mod fwd_dynamic;
pub mod fwd_esdt;
pub mod fwd_nft;
pub mod fwd_roles;
pub mod fwd_sft;
pub mod fwd_storage;
pub mod fwd_upgrade;
pub mod vault_proxy;
pub mod vault_upgrade_proxy;

multiversx_sc::imports!();

/// Test contract for investigating contract calls.
#[multiversx_sc::contract]
pub trait Forwarder:
    fwd_call_sync::ForwarderSyncCallModule
    + fwd_call_async::ForwarderAsyncCallModule
    + fwd_call_transf_exec::ForwarderTransferExecuteModule
    + fwd_change_owner::ChangeOwnerModule
    + fwd_deploy::DeployContractModule
    + fwd_upgrade::UpgradeContractModule
    + fwd_esdt::ForwarderEsdtModule
    + fwd_sft::ForwarderSftModule
    + fwd_nft::ForwarderNftModule
    + fwd_roles::ForwarderRolesModule
    + fwd_dynamic::ForwarderDynamicModule
    + fwd_storage::ForwarderStorageModule
    + common::CommonModule
    + fwd_call_promises::CallPromisesModule
    + fwd_call_promise_direct::CallPromisesDirectModule
    + fwd_call_sync_bt_legacy::BackTransfersLegacyModule
    + fwd_call_sync_bt::BackTransfersModule
    + fwd_call_promises_bt::CallPromisesBackTransfersModule
{
    #[init]
    fn init(&self) {}

    #[endpoint]
    fn send_egld(&self, to: &ManagedAddress, amount: &BigUint) {
        self.tx().to(to).egld(amount).transfer();
    }
}
