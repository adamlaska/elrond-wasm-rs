use std::cell::RefCell;

use crate::api::{VMHooksApi, VMHooksBackendType};
use multiversx_chain_vm::tx_mock::TxStaticVars;
use multiversx_sc::{
    api::{
        use_raw_handle, HandleConstraints, HandleTypeInfo, RawHandle, StaticVarApi,
        StaticVarApiImpl,
    },
    types::LockableStaticBuffer,
};

impl<const BACKEND_TYPE: VMHooksBackendType> StaticVarApi for VMHooksApi<BACKEND_TYPE> {
    type StaticVarApiImpl = StaticApiStaticVarImpl;

    fn static_var_api_impl() -> Self::StaticVarApiImpl {
        StaticApiStaticVarImpl
    }
}

thread_local!(
    static LOCKABLE_STATIC_BUFFER_CELL: RefCell<LockableStaticBuffer> = RefCell::new(LockableStaticBuffer::new())
);

thread_local!(
    static STATIC_VARS_CELL: RefCell<TxStaticVars> = RefCell::new(TxStaticVars::default())
);

pub struct StaticApiStaticVarImpl;

impl HandleTypeInfo for StaticApiStaticVarImpl {
    type ManagedBufferHandle = RawHandle;
    type BigIntHandle = RawHandle;
    type BigFloatHandle = RawHandle;
    type EllipticCurveHandle = RawHandle;
    type ManagedMapHandle = RawHandle;
}

impl StaticVarApiImpl for StaticApiStaticVarImpl {
    fn with_lockable_static_buffer<R, F: FnOnce(&mut LockableStaticBuffer) -> R>(&self, f: F) -> R {
        LOCKABLE_STATIC_BUFFER_CELL.with(|lockable_static_buffer_cell| {
            let mut lockable_static_buffer = lockable_static_buffer_cell.borrow_mut();
            f(&mut lockable_static_buffer)
        })
    }

    fn set_external_view_target_address_handle(&self, handle: RawHandle) {
        STATIC_VARS_CELL.with(|static_vars_cell| {
            static_vars_cell
                .borrow_mut()
                .external_view_target_address_handle = handle.get_raw_handle();
        });
    }

    fn get_external_view_target_address_handle(&self) -> RawHandle {
        STATIC_VARS_CELL.with(|static_vars_cell| {
            static_vars_cell
                .borrow()
                .external_view_target_address_handle
        })
    }

    fn next_handle(&self) -> RawHandle {
        STATIC_VARS_CELL.with(|static_vars_cell| {
            let mut ref_tx_static_vars = static_vars_cell.borrow_mut();
            let new_handle = ref_tx_static_vars.next_handle;
            ref_tx_static_vars.next_handle -= 1;
            new_handle
        })
    }

    fn set_num_arguments(&self, num_arguments: i32) {
        STATIC_VARS_CELL.with(|static_vars_cell| {
            static_vars_cell.borrow_mut().num_arguments = num_arguments;
        })
    }

    fn get_num_arguments(&self) -> i32 {
        STATIC_VARS_CELL.with(|static_vars_cell| static_vars_cell.borrow().num_arguments)
    }

    fn set_call_value_egld_handle(&self, handle: RawHandle) {
        STATIC_VARS_CELL.with(|static_vars_cell| {
            static_vars_cell.borrow_mut().call_value_egld_handle = handle.get_raw_handle();
        })
    }

    fn get_call_value_egld_handle(&self) -> RawHandle {
        STATIC_VARS_CELL.with(|static_vars_cell| {
            use_raw_handle(static_vars_cell.borrow().call_value_egld_handle)
        })
    }

    fn set_call_value_multi_esdt_handle(&self, handle: RawHandle) {
        STATIC_VARS_CELL.with(|static_vars_cell| {
            static_vars_cell.borrow_mut().call_value_multi_esdt_handle = handle.get_raw_handle();
        })
    }

    fn get_call_value_multi_esdt_handle(&self) -> RawHandle {
        STATIC_VARS_CELL.with(|static_vars_cell| {
            use_raw_handle(static_vars_cell.borrow().call_value_multi_esdt_handle)
        })
    }
}
