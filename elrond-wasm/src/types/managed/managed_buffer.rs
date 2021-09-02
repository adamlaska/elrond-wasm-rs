use super::ManagedType;
use crate::api::InvalidSliceError;
use crate::{
    api::{Handle, ManagedTypeApi},
    types::BoxedBytes,
};
use alloc::string::String;
use elrond_codec::{
    DecodeError, EncodeError, NestedDecode, NestedDecodeInput, NestedEncode, NestedEncodeOutput,
    TopDecode, TopDecodeInput, TopEncode, TopEncodeOutput, TryStaticCast,
};

/// A byte buffer managed by an external API.
#[derive(Debug)]
pub struct ManagedBuffer<M: ManagedTypeApi> {
    pub(crate) handle: Handle,
    pub(crate) api: M,
}

impl<M: ManagedTypeApi> ManagedType<M> for ManagedBuffer<M> {
    #[inline]
    fn from_raw_handle(api: M, handle: Handle) -> Self {
        ManagedBuffer { handle, api }
    }

    #[doc(hidden)]
    fn get_raw_handle(&self) -> Handle {
        self.handle
    }

    #[inline]
    fn type_manager(&self) -> M {
        self.api.clone()
    }
}

impl<M: ManagedTypeApi> ManagedBuffer<M> {
    #[inline]
    pub fn new_empty(api: M) -> Self {
        ManagedBuffer {
            handle: api.mb_new_empty(),
            api,
        }
    }

    #[inline(always)]
    pub fn new_from_bytes(api: M, bytes: &[u8]) -> Self {
        ManagedBuffer {
            handle: api.mb_new_from_bytes(bytes),
            api,
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.api.mb_len(self.handle)
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn to_boxed_bytes(&self) -> BoxedBytes {
        self.api.mb_to_boxed_bytes(self.handle)
    }

    /// TODO: investigate the impact of using `Result<(), ()>` on the wasm output.
    #[inline]
    pub fn load_slice(
        &self,
        starting_position: usize,
        dest_slice: &mut [u8],
    ) -> Result<(), InvalidSliceError> {
        self.api
            .mb_load_slice(self.handle, starting_position, dest_slice)
    }

    pub fn copy_slice(
        &self,
        starting_position: usize,
        slice_len: usize,
    ) -> Option<ManagedBuffer<M>> {
        let result_handle = self.api.mb_new_empty();
        let err_result =
            self.api
                .mb_copy_slice(self.handle, starting_position, slice_len, result_handle);
        if err_result.is_ok() {
            Some(ManagedBuffer::from_raw_handle(
                self.api.clone(),
                result_handle,
            ))
        } else {
            None
        }
    }

    #[inline]
    pub fn overwrite(&mut self, value: &[u8]) {
        self.api.mb_overwrite(self.handle, value);
    }

    #[inline]
    pub fn append(&mut self, other: &ManagedBuffer<M>) {
        self.api.mb_append(self.handle, other.handle);
    }

    #[inline(always)]
    pub fn append_bytes(&mut self, slice: &[u8]) {
        self.api.mb_append_bytes(self.handle, slice);
    }

    /// Utility function: helps serialize lengths (or any other value of type usize) easier.
    #[inline(never)]
    pub fn append_u32_be(&mut self, item: u32) {
        self.api
            .mb_append_bytes(self.handle, &item.to_be_bytes()[..]);
    }
}

impl<M: ManagedTypeApi> Clone for ManagedBuffer<M> {
    fn clone(&self) -> Self {
        let clone_handle = self.api.mb_new_empty();
        self.api.mb_append(clone_handle, self.handle);
        ManagedBuffer {
            handle: clone_handle,
            api: self.api.clone(),
        }
    }
}

impl<M: ManagedTypeApi> PartialEq for ManagedBuffer<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.api.mb_eq(self.handle, other.handle)
    }
}

impl<M: ManagedTypeApi> Eq for ManagedBuffer<M> {}

impl<M: ManagedTypeApi> TryStaticCast for ManagedBuffer<M> {}

impl<M: ManagedTypeApi> TopEncode for ManagedBuffer<M> {
    #[inline]
    fn top_encode<O: TopEncodeOutput>(&self, output: O) -> Result<(), EncodeError> {
        output.set_specialized(self, |else_output| {
            else_output.set_slice_u8(self.to_boxed_bytes().as_slice());
            Ok(())
        })
    }
}

impl<M: ManagedTypeApi> NestedEncode for ManagedBuffer<M> {
    fn dep_encode<O: NestedEncodeOutput>(&self, dest: &mut O) -> Result<(), EncodeError> {
        dest.push_specialized((), self, |else_output| {
            self.to_boxed_bytes().dep_encode(else_output)
        })
    }
}

impl<M: ManagedTypeApi> TopDecode for ManagedBuffer<M> {
    fn top_decode<I: TopDecodeInput>(input: I) -> Result<Self, DecodeError> {
        input.into_specialized(|_| Err(DecodeError::UNSUPPORTED_OPERATION))
    }
}

impl<M: ManagedTypeApi> NestedDecode for ManagedBuffer<M> {
    fn dep_decode<I: NestedDecodeInput>(input: &mut I) -> Result<Self, DecodeError> {
        input.read_specialized((), |_| Err(DecodeError::UNSUPPORTED_OPERATION))
    }

    fn dep_decode_or_exit<I: NestedDecodeInput, ExitCtx: Clone>(
        input: &mut I,
        c: ExitCtx,
        exit: fn(ExitCtx, DecodeError) -> !,
    ) -> Self {
        input.read_specialized_or_exit((), c, exit, |_, c| {
            exit(c, DecodeError::UNSUPPORTED_OPERATION)
        })
    }
}

impl<M: ManagedTypeApi> crate::abi::TypeAbi for ManagedBuffer<M> {
    fn type_name() -> String {
        "bytes".into()
    }
}