use crate::{
    api::{CallTypeApi, ManagedTypeApi},
    contract_base::SendRawWrapper,
    types::{
        EgldOrEsdtTokenPayment, EgldOrMultiEsdtPayment, EgldPayment, EsdtTokenPayment,
        ManagedAddress, MultiEsdtPayment,
    },
};

use super::{FunctionCall, TxFrom};

/// Temporary structure for returning a normalized transfer.
pub struct PaymentConversionResult<Api>
where
    Api: ManagedTypeApi,
{
    pub to: ManagedAddress<Api>,
    pub egld_payment: EgldPayment<Api>,
    pub fc: FunctionCall<Api>,
}

pub trait TxPayment<Api>
where
    Api: CallTypeApi,
{
    fn is_no_payment(&self) -> bool;

    fn convert_tx_data<From>(
        self,
        from: &From,
        to: ManagedAddress<Api>,
        fc: FunctionCall<Api>,
    ) -> PaymentConversionResult<Api>
    where
        From: TxFrom<Api>;

    fn perform_transfer_execute(
        self,
        to: &ManagedAddress<Api>,
        gas_limit: u64,
        fc: FunctionCall<Api>,
    );
}

impl<Api> TxPayment<Api> for ()
where
    Api: CallTypeApi,
{
    fn is_no_payment(&self) -> bool {
        true
    }

    fn convert_tx_data<From>(
        self,
        _from: &From,
        to: ManagedAddress<Api>,
        fc: FunctionCall<Api>,
    ) -> PaymentConversionResult<Api>
    where
        From: TxFrom<Api>,
    {
        PaymentConversionResult {
            to,
            egld_payment: EgldPayment::no_payment(),
            fc,
        }
    }

    fn perform_transfer_execute(
        self,
        to: &ManagedAddress<Api>,
        gas_limit: u64,
        fc: FunctionCall<Api>,
    ) {
        EgldPayment::no_payment().perform_transfer_execute(to, gas_limit, fc);
    }
}

impl<Api> TxPayment<Api> for EgldPayment<Api>
where
    Api: CallTypeApi,
{
    fn is_no_payment(&self) -> bool {
        self.value == 0u32
    }

    fn convert_tx_data<From>(
        self,
        _from: &From,
        to: ManagedAddress<Api>,
        fc: FunctionCall<Api>,
    ) -> PaymentConversionResult<Api>
    where
        From: TxFrom<Api>,
    {
        PaymentConversionResult {
            to,
            egld_payment: self,
            fc,
        }
    }

    fn perform_transfer_execute(
        self,
        to: &ManagedAddress<Api>,
        gas_limit: u64,
        fc: FunctionCall<Api>,
    ) {
        let _ = SendRawWrapper::<Api>::new().direct_egld_execute(
            to,
            &self.value,
            gas_limit,
            &fc.function_name,
            &fc.arg_buffer,
        );
    }
}

impl<Api> TxPayment<Api> for EsdtTokenPayment<Api>
where
    Api: CallTypeApi,
{
    fn is_no_payment(&self) -> bool {
        self.amount == 0u32
    }

    fn convert_tx_data<From>(
        self,
        from: &From,
        to: ManagedAddress<Api>,
        fc: FunctionCall<Api>,
    ) -> PaymentConversionResult<Api>
    where
        From: TxFrom<Api>,
    {
        if self.token_nonce == 0 {
            convert_tx_data_fungible(self, to, fc)
        } else {
            convert_tx_data_nft(self, from.resolve_address(), to, fc)
        }
    }

    fn perform_transfer_execute(
        self,
        to: &ManagedAddress<Api>,
        gas_limit: u64,
        fc: FunctionCall<Api>,
    ) {
        MultiEsdtPayment::from_single_item(self).perform_transfer_execute(to, gas_limit, fc)
    }
}

impl<Api> TxPayment<Api> for MultiEsdtPayment<Api>
where
    Api: CallTypeApi,
{
    fn is_no_payment(&self) -> bool {
        self.is_empty()
    }

    fn convert_tx_data<From>(
        self,
        from: &From,
        to: ManagedAddress<Api>,
        fc: FunctionCall<Api>,
    ) -> PaymentConversionResult<Api>
    where
        From: TxFrom<Api>,
    {
        match self.len() {
            0 => ().convert_tx_data(from, to, fc),
            1 => self.get(0).convert_tx_data(from, to, fc),
            _ => convert_tx_data_multi(self, from.resolve_address(), to, fc),
        }
    }

    fn perform_transfer_execute(
        self,
        to: &ManagedAddress<Api>,
        gas_limit: u64,
        fc: FunctionCall<Api>,
    ) {
        let _ = SendRawWrapper::<Api>::new().multi_esdt_transfer_execute(
            to,
            &self,
            gas_limit,
            &fc.function_name,
            &fc.arg_buffer,
        );
    }
}

impl<Api> TxPayment<Api> for EgldOrEsdtTokenPayment<Api>
where
    Api: CallTypeApi,
{
    fn is_no_payment(&self) -> bool {
        self.amount == 0u32
    }

    fn convert_tx_data<From>(
        self,
        from: &From,
        to: ManagedAddress<Api>,
        fc: FunctionCall<Api>,
    ) -> PaymentConversionResult<Api>
    where
        From: TxFrom<Api>,
    {
        self.map_egld_or_esdt(
            (to, fc),
            |(to, fc), amount| EgldPayment::from(amount).convert_tx_data(from, to, fc),
            |(to, fc), esdt_payment| esdt_payment.convert_tx_data(from, to, fc),
        )
    }

    fn perform_transfer_execute(
        self,
        to: &ManagedAddress<Api>,
        gas_limit: u64,
        fc: FunctionCall<Api>,
    ) {
        self.map_egld_or_esdt(
            (to, fc),
            |(to, fc), amount| {
                EgldPayment::from(amount).perform_transfer_execute(to, gas_limit, fc)
            },
            |(to, fc), esdt_payment| esdt_payment.perform_transfer_execute(to, gas_limit, fc),
        )
    }
}

impl<Api> TxPayment<Api> for EgldOrMultiEsdtPayment<Api>
where
    Api: CallTypeApi,
{
    fn is_no_payment(&self) -> bool {
        self.is_empty()
    }

    fn convert_tx_data<From>(
        self,
        from: &From,
        to: ManagedAddress<Api>,
        fc: FunctionCall<Api>,
    ) -> PaymentConversionResult<Api>
    where
        From: TxFrom<Api>,
    {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                EgldPayment::from(egld_amount).convert_tx_data(from, to, fc)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.convert_tx_data(from, to, fc)
            },
        }
    }

    fn perform_transfer_execute(
        self,
        to: &ManagedAddress<Api>,
        gas_limit: u64,
        fc: FunctionCall<Api>,
    ) {
        match self {
            EgldOrMultiEsdtPayment::Egld(egld_amount) => {
                EgldPayment::from(egld_amount).perform_transfer_execute(to, gas_limit, fc)
            },
            EgldOrMultiEsdtPayment::MultiEsdt(multi_esdt_payment) => {
                multi_esdt_payment.perform_transfer_execute(to, gas_limit, fc)
            },
        }
    }
}

fn convert_tx_data_fungible<Api>(
    payment: EsdtTokenPayment<Api>,
    to: ManagedAddress<Api>,
    fc: FunctionCall<Api>,
) -> PaymentConversionResult<Api>
where
    Api: ManagedTypeApi,
{
    PaymentConversionResult {
        to,
        egld_payment: EgldPayment::no_payment(),
        fc: fc.convert_to_single_transfer_fungible_call(payment),
    }
}

fn convert_tx_data_nft<Api>(
    payment: EsdtTokenPayment<Api>,
    from: ManagedAddress<Api>,
    to: ManagedAddress<Api>,
    fc: FunctionCall<Api>,
) -> PaymentConversionResult<Api>
where
    Api: ManagedTypeApi,
{
    PaymentConversionResult {
        to: from,
        egld_payment: EgldPayment::no_payment(),
        fc: fc.convert_to_single_transfer_nft_call(&to, payment),
    }
}

fn convert_tx_data_multi<Api>(
    payment: MultiEsdtPayment<Api>,
    from: ManagedAddress<Api>,
    to: ManagedAddress<Api>,
    fc: FunctionCall<Api>,
) -> PaymentConversionResult<Api>
where
    Api: ManagedTypeApi,
{
    PaymentConversionResult {
        to: from,
        egld_payment: EgldPayment::no_payment(),
        fc: fc.convert_to_multi_transfer_esdt_call(&to, payment),
    }
}
