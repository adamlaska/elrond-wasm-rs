use crate::{
    api::quick_signal_error,
    contract_base::TransferExecuteFailed,
    err_msg,
    types::{BigUint, Egld, ManagedAddress, TxFrom, TxToSpecified},
};

use super::{FullPaymentData, FunctionCall, TxEnv, TxNoPayment, TxPayment, TxPaymentEgldOnly};

impl<Env> TxPayment<Env> for ()
where
    Env: TxEnv,
{
    #[inline]
    fn is_no_payment(&self, _env: &Env) -> bool {
        true
    }

    #[inline]
    fn perform_transfer_execute_fallible(
        self,
        _env: &Env,
        _to: &ManagedAddress<Env::Api>,
        _gas_limit: u64,
        _fc: FunctionCall<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        quick_signal_error::<Env::Api>(err_msg::TRANSFER_EXECUTE_REQUIRES_PAYMENT)
    }

    #[inline]
    fn perform_transfer_fallible(
        self,
        _env: &Env,
        _to: &ManagedAddress<Env::Api>,
    ) -> Result<(), TransferExecuteFailed> {
        quick_signal_error::<Env::Api>(err_msg::TRANSFER_EXECUTE_REQUIRES_PAYMENT)
    }

    fn perform_transfer_execute_legacy(
        self,
        env: &Env,
        to: &ManagedAddress<Env::Api>,
        gas_limit: u64,
        fc: FunctionCall<Env::Api>,
    ) {
        Egld(BigUint::zero_ref()).perform_transfer_execute_legacy(env, to, gas_limit, fc);
    }

    #[inline]
    fn with_normalized<From, To, F, R>(
        self,
        env: &Env,
        _from: &From,
        to: To,
        fc: FunctionCall<Env::Api>,
        f: F,
    ) -> R
    where
        From: TxFrom<Env>,
        To: TxToSpecified<Env>,
        F: FnOnce(&ManagedAddress<Env::Api>, &BigUint<Env::Api>, FunctionCall<Env::Api>) -> R,
    {
        to.with_address_ref(env, |to_addr| f(to_addr, &*BigUint::zero_ref(), fc))
    }

    fn into_full_payment_data(self, _env: &Env) -> FullPaymentData<Env::Api> {
        FullPaymentData::default()
    }
}

impl<Env> TxNoPayment<Env> for () where Env: TxEnv {}

impl<Env> TxPaymentEgldOnly<Env> for () where Env: TxEnv {}
