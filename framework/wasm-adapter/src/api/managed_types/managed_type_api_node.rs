use crate::api::VmApiImpl;
use multiversx_sc::api::{ManagedTypeApi, ManagedTypeApiImpl};

unsafe extern "C" {
    fn mBufferToBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferToBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntUnsigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;
    fn mBufferFromBigIntSigned(mBufferHandle: i32, bigIntHandle: i32) -> i32;

    fn mBufferToSmallIntUnsigned(mBufferHandle: i32) -> i64;
    fn mBufferToSmallIntSigned(mBufferHandle: i32) -> i64;
    fn mBufferFromSmallIntUnsigned(mBufferHandle: i32, value: i64);
    fn mBufferFromSmallIntSigned(mBufferHandle: i32, value: i64);

    fn mBufferToBigFloat(mBufferHandle: i32, bigFloatHandle: i32) -> i32;
    fn mBufferFromBigFloat(mBufferHandle: i32, bigFloatHandle: i32) -> i32;

    fn validateTokenIdentifier(token_id_handle: i32) -> i32;
}

impl ManagedTypeApi for VmApiImpl {
    type ManagedTypeApiImpl = VmApiImpl;

    fn managed_type_impl() -> Self {
        VmApiImpl {}
    }
}

impl ManagedTypeApiImpl for VmApiImpl {
    #[inline]
    fn mb_to_big_int_unsigned(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        big_int_handle: Self::BigIntHandle,
    ) {
        unsafe {
            mBufferToBigIntUnsigned(buffer_handle, big_int_handle);
        }
    }

    #[inline]
    fn mb_to_big_int_signed(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        big_int_handle: Self::BigIntHandle,
    ) {
        unsafe {
            mBufferToBigIntSigned(buffer_handle, big_int_handle);
        }
    }

    #[inline]
    fn mb_from_big_int_unsigned(
        &self,
        big_int_handle: Self::BigIntHandle,
        buffer_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            mBufferFromBigIntUnsigned(buffer_handle, big_int_handle);
        }
    }

    #[inline]
    fn mb_from_big_int_signed(
        &self,
        big_int_handle: Self::BigIntHandle,
        buffer_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            mBufferFromBigIntSigned(buffer_handle, big_int_handle);
        }
    }

    fn mb_to_small_int_unsigned(&self, buffer_handle: Self::ManagedBufferHandle) -> i64 {
        unsafe { mBufferToSmallIntUnsigned(buffer_handle) }
    }

    fn mb_to_small_int_signed(&self, buffer_handle: Self::ManagedBufferHandle) -> i64 {
        unsafe { mBufferToSmallIntSigned(buffer_handle) }
    }

    fn mb_from_small_int_unsigned(&self, buffer_handle: Self::ManagedBufferHandle, value: i64) {
        unsafe {
            mBufferFromSmallIntUnsigned(buffer_handle, value);
        }
    }

    fn mb_from_small_int_signed(&self, buffer_handle: Self::ManagedBufferHandle, value: i64) {
        unsafe {
            mBufferFromSmallIntSigned(buffer_handle, value);
        }
    }

    #[inline]
    fn mb_to_big_float(
        &self,
        buffer_handle: Self::ManagedBufferHandle,
        big_float_handle: Self::BigFloatHandle,
    ) {
        unsafe {
            mBufferToBigFloat(buffer_handle, big_float_handle);
        }
    }

    #[inline]
    fn mb_from_big_float(
        &self,
        big_float_handle: Self::BigFloatHandle,
        buffer_handle: Self::ManagedBufferHandle,
    ) {
        unsafe {
            mBufferFromBigFloat(buffer_handle, big_float_handle);
        }
    }

    #[inline]
    fn validate_token_identifier(&self, token_id_handle: Self::ManagedBufferHandle) -> bool {
        unsafe { validateTokenIdentifier(token_id_handle) != 0 }
    }
}
