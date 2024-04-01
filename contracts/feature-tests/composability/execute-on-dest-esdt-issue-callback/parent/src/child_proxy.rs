// Code generated by the multiversx-sc proxy generator. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

#![allow(dead_code)]
#![allow(clippy::all)]

use multiversx_sc::proxy_imports::*;

pub struct ChildProxy;

impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for ChildProxy
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    type TxProxyMethods = ChildProxyMethods<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {
        ChildProxyMethods { wrapped_tx: tx }
    }
}

pub struct ChildProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}

#[rustfmt::skip]
impl<Env, From, Gas> ChildProxyMethods<Env, From, (), Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    Gas: TxGas<Env>,
{
    pub fn init(
        self,
    ) -> TxProxyDeploy<Env, From, Gas, ()> {
        self.wrapped_tx
            .raw_deploy()
            .original_result()
    }
}

#[rustfmt::skip]
impl<Env, From, To, Gas> ChildProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
}

#[rustfmt::skip]
impl<Env, From, To, Gas> ChildProxyMethods<Env, From, To, Gas>
where
    Env: TxEnv,
    Env::Api: VMApi,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{
    pub fn issue_wrapped_egld<
        Arg0: CodecInto<ManagedBuffer<Env::Api>>,
        Arg1: CodecInto<ManagedBuffer<Env::Api>>,
        Arg2: CodecInto<BigUint<Env::Api>>,
    >(
        self,
        token_display_name: Arg0,
        token_ticker: Arg1,
        initial_supply: Arg2,
    ) -> TxProxyCall<Env, From, To, Gas, ()> {
        self.wrapped_tx
            .raw_call("issueWrappedEgld")
            .argument(&token_display_name)
            .argument(&token_ticker)
            .argument(&initial_supply)
            .original_result()
    }

    pub fn wrapped_egld_token_identifier(
        self,
    ) -> TxProxyCall<Env, From, To, Gas, TokenIdentifier<Env::Api>> {
        self.wrapped_tx
            .raw_call("getWrappedEgldTokenIdentifier")
            .original_result()
    }
}
