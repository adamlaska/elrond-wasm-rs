use core::ops::Deref;

use crate::{
    contract_base::{SendRawWrapper, TransferExecuteFailed},
    types::{BigUint, ManagedAddress, ManagedRef, MultiEsdtPayment, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxPayment};

/// Indicates that a payment object contains a multi-ESDT payment.
pub trait TxPaymentMultiEsdt<Env>: TxPayment<Env>
where
    Env: TxEnv,
{
}

impl<Env> TxPaymentMultiEsdt<Env> for MultiEsdtPayment<Env::Api> where Env: TxEnv {}
impl<Env> TxPaymentMultiEsdt<Env> for &MultiEsdtPayment<Env::Api> where Env: TxEnv {}
impl<Env> TxPaymentMultiEsdt<Env> for ManagedRef<'_, Env::Api, MultiEsdtPayment<Env::Api>> where
    Env: TxEnv
{
}

impl<Env> TxPayment<Env> for &MultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
{
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.is_empty()
    }

    fn perform_transfer_execute_fallible(
        self,
        _env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        SendRawWrapper::<Env::Api>::new().multi_egld_or_esdt_transfer_execute_fallible(
            to,
            self.as_multi_egld_or_esdt_payment(),
            gas_limit,
            &fc.function_name,
            &fc.arg_buffer,
        )
    }

    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        match self.len() {
            0 => ().with_normalized(env, from, to, fc, f),
            1 => self.get(0).as_refs().with_normalized(env, from, to, fc, f),
            _ => to.with_address_ref(env, |to_addr| {
                let fc_conv = fc.convert_to_multi_transfer_esdt_call(
                    to_addr,
                    self.as_multi_egld_or_esdt_payment(),
                );
                f(&from.resolve_address(env), &*BigUint::zero_ref(), fc_conv)
            }),
        }
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: self.as_multi_egld_or_esdt_payment().clone(),
        }
    }
}

impl<Env> TxPayment<Env> for ManagedRef<'_, Env::Api, MultiEsdtPayment<Env::Api>>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.deref().is_empty()
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        self.deref()
            .perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        self.deref().with_normalized(env, from, to, fc, f)
    }

    fn into_full_payment_data(self, env: &Env) -> FullPaymentData<Env::Api> {
        self.deref().into_full_payment_data(env)
    }
}

impl<Env> TxPayment<Env> for MultiEsdtPayment<Env::Api>
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        self.is_empty()
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        (&self).perform_transfer_execute_fallible(env, to, gas_limit, fc)
    }

    #[inline]
    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        (&self).with_normalized(env, from, to, fc, f)
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData {
            egld: None,
            multi_esdt: self.into_multi_egld_or_esdt_payment(),
        }
    }
}
