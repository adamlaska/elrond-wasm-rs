use super::*;

/// Indicates how smart contract method implementations will be auto-generated based on their annotations.
#[derive(Clone, Debug)]
pub enum AutoImpl {
    Event { identifier: String },
    StorageGetter { identifier: String },
    StorageSetter { identifier: String },
    StorageMapper { identifier: String },
    StorageMapperFromAddress { identifier: String },
    StorageIsEmpty { identifier: String },
    StorageClear { identifier: String },
    ProxyGetter,
}
#[derive(Clone, Debug)]
pub enum MethodImpl {
    /// Implementation auto-generated by the framework. There can (obviously) be only one per method.
    Generated(AutoImpl),

    /// Methods where the developer has provided an explicit implementation.
    Explicit(syn::Block),

    /// Methods that have no implementation and are not annotated as such.
    /// They are not allowed in contracts and modules, but they are used in call proxies.
    NoImplementation,
}

impl MethodImpl {
    pub fn is_no_implementation(&self) -> bool {
        matches!(self, MethodImpl::NoImplementation)
    }
}

/// Models any method argument from a contract, module or callable proxy trait.
#[derive(Clone, Debug)]
pub struct Method {
    pub docs: Vec<String>,
    pub public_role: PublicRole,
    pub name: syn::Ident,
    pub generics: syn::Generics,
    pub unprocessed_attributes: Vec<syn::Attribute>,
    pub method_args: Vec<MethodArgument>,
    pub title: Option<String>,
    pub output_names: Vec<String>,
    pub label_names: Vec<String>,
    pub return_type: syn::ReturnType,
    pub implementation: MethodImpl,
}

impl Method {
    pub fn payment_token_arg(&self) -> Option<MethodArgument> {
        self.method_args
            .iter()
            .find(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentToken))
            .cloned()
    }

    pub fn payment_amount_arg(&self) -> Option<MethodArgument> {
        self.method_args
            .iter()
            .find(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentAmount))
            .cloned()
    }

    pub fn payment_nonce_arg(&self) -> Option<MethodArgument> {
        self.method_args
            .iter()
            .find(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentNonce))
            .cloned()
    }

    pub fn payment_multi_arg(&self) -> Option<MethodArgument> {
        self.method_args
            .iter()
            .find(|&arg| matches!(arg.metadata.payment, ArgPaymentMetadata::PaymentMulti))
            .cloned()
    }

    pub fn is_payable(&self) -> bool {
        match &self.public_role {
            PublicRole::Init(init_metadata) => init_metadata.payable.is_payable(),
            PublicRole::Upgrade(upgrade_metadata) => upgrade_metadata.payable.is_payable(),
            PublicRole::Endpoint(endpoint_metadata) => endpoint_metadata.payable.is_payable(),
            PublicRole::Callback(_) | PublicRole::CallbackRaw | PublicRole::CallbackPromise(_) => {
                true
            }
            PublicRole::Private => false,
        }
    }

    pub fn payable_metadata(&self) -> MethodPayableMetadata {
        match &self.public_role {
            PublicRole::Init(init_metadata) => init_metadata.payable.clone(),
            PublicRole::Upgrade(upgrade_metadata) => upgrade_metadata.payable.clone(),
            PublicRole::Endpoint(endpoint_metadata) => endpoint_metadata.payable.clone(),
            PublicRole::Callback(_) | PublicRole::CallbackRaw | PublicRole::CallbackPromise(_) => {
                MethodPayableMetadata::AnyToken
            }
            PublicRole::Private => MethodPayableMetadata::NotPayable,
        }
    }

    pub fn is_allow_multiple_var_args(&self) -> bool {
        match &self.public_role {
            PublicRole::Init(init_metadata) => init_metadata.allow_multiple_var_args,
            PublicRole::Upgrade(upgrade_metadata) => upgrade_metadata.allow_multiple_var_args,
            PublicRole::Endpoint(endpoint_metadata) => endpoint_metadata.allow_multiple_var_args,
            PublicRole::Callback(callback_metadata)
            | PublicRole::CallbackPromise(callback_metadata) => {
                callback_metadata.allow_multiple_var_args
            }
            PublicRole::CallbackRaw => true,
            PublicRole::Private => false,
        }
    }
}
