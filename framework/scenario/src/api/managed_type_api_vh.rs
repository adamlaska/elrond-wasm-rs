mod big_float_api_vh;
mod big_int_api_vh;
mod elliptic_curve_api_vh;
mod managed_buffer_api_vh;
mod managed_map_api_vh;

use multiversx_sc::api::{HandleConstraints, ManagedTypeApi, ManagedTypeApiImpl};

use crate::api::{VMHooksApi, VMHooksApiBackend};

use super::i32_to_bool;

impl<VHB: VMHooksApiBackend> ManagedTypeApi for VMHooksApi<VHB> {
    type ManagedTypeApiImpl = Self;

    fn managed_type_impl() -> Self::ManagedTypeApiImpl {
        Self::api_impl()
    }
}

impl<VHB: VMHooksApiBackend> ManagedTypeApiImpl for VMHooksApi<VHB> {
    fn mb_to_big_int_unsigned(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        big_int_handle: Self::BigIntHandle,
    ) {
        self.with_vm_hooks_ctx_2(&buffer_handle, &big_int_handle, |vh| {
            vh.mbuffer_to_big_int_unsigned(
                buffer_handle.get_raw_handle_unchecked(),
                big_int_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn mb_to_big_int_signed(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        big_int_handle: Self::BigIntHandle,
    ) {
        self.with_vm_hooks_ctx_2(&buffer_handle, &big_int_handle, |vh| {
            vh.mbuffer_to_big_int_signed(
                buffer_handle.get_raw_handle_unchecked(),
                big_int_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn mb_from_big_int_unsigned(
        &self,
        big_int_handle: Self::BigIntHandle,
        buffer_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_2(&buffer_handle, &big_int_handle, |vh| {
            vh.mbuffer_from_big_int_unsigned(
                buffer_handle.get_raw_handle_unchecked(),
                big_int_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn mb_from_big_int_signed(
        &self,
        big_int_handle: Self::BigIntHandle,
        buffer_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_2(&buffer_handle, &big_int_handle, |vh| {
            vh.mbuffer_from_big_int_signed(
                buffer_handle.get_raw_handle_unchecked(),
                big_int_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn mb_to_small_int_unsigned(&self, buffer_handle: Self::ManagedBufferHandle) -> i64 {
        self.with_vm_hooks_ctx_1(&buffer_handle, |vh| {
            vh.mbuffer_to_small_int_unsigned(buffer_handle.get_raw_handle_unchecked())
        })
    }

    fn mb_to_small_int_signed(&self, buffer_handle: Self::ManagedBufferHandle) -> i64 {
        self.with_vm_hooks_ctx_1(&buffer_handle, |vh| {
            vh.mbuffer_to_small_int_signed(buffer_handle.get_raw_handle_unchecked())
        })
    }

    fn mb_from_small_int_unsigned(&self, buffer_handle: Self::ManagedBufferHandle, value: i64) {
        self.with_vm_hooks_ctx_1(&buffer_handle, |vh| {
            vh.mbuffer_from_small_int_unsigned(buffer_handle.get_raw_handle_unchecked(), value)
        })
    }

    fn mb_from_small_int_signed(&self, buffer_handle: Self::ManagedBufferHandle, value: i64) {
        self.with_vm_hooks_ctx_1(&buffer_handle, |vh| {
            vh.mbuffer_from_small_int_signed(buffer_handle.get_raw_handle_unchecked(), value)
        })
    }

    fn mb_to_big_float(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        big_float_handle: Self::BigFloatHandle,
    ) {
        self.with_vm_hooks_ctx_2(&buffer_handle, &big_float_handle, |vh| {
            vh.mbuffer_to_big_float(
                buffer_handle.get_raw_handle_unchecked(),
                big_float_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn mb_from_big_float(
        &self,
        big_float_handle: Self::BigFloatHandle,
        buffer_handle: Self::ManagedBufferHandle,
    ) {
        self.with_vm_hooks_ctx_2(&buffer_handle, &big_float_handle, |vh| {
            vh.mbuffer_from_big_float(
                buffer_handle.get_raw_handle_unchecked(),
                big_float_handle.get_raw_handle_unchecked(),
            )
        });
    }

    fn validate_token_identifier(&self, token_id_handle: Self::ManagedBufferHandle) -> bool {
        let result = self.with_vm_hooks_ctx_1(&token_id_handle, |vh| {
            vh.validate_token_identifier(token_id_handle.get_raw_handle_unchecked())
        });
        i32_to_bool(result)
    }
}
