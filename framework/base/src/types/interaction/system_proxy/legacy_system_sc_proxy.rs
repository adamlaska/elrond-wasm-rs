use core::marker::PhantomData;

// use super::properties::*;

use crate::{
    api::{CallTypeApi, SendApi},
    types::{
        BigUint, ContractCall, ContractCallNoPayment, ContractCallWithEgld, ESDTSystemSCAddress,
        EsdtLocalRole, EsdtTokenType, ManagedAddress, ManagedBuffer, TokenIdentifier,
    },
};

use super::token_properties::*;

const ISSUE_FUNGIBLE_ENDPOINT_NAME: &str = "issue";
const ISSUE_NON_FUNGIBLE_ENDPOINT_NAME: &str = "issueNonFungible";
const ISSUE_SEMI_FUNGIBLE_ENDPOINT_NAME: &str = "issueSemiFungible";
const REGISTER_META_ESDT_ENDPOINT_NAME: &str = "registerMetaESDT";
const ISSUE_AND_SET_ALL_ROLES_ENDPOINT_NAME: &str = "registerAndSetAllRoles";
const REGISTER_DYNAMIC_ESDT_ENDPOINT_NAME: &str = "registerDynamic";
const REGISTER_AND_SET_ALL_ROLES_DYNAMIC_ESDT_ENDPOINT_NAME: &str = "registerAndSetAllRolesDynamic";

/// Proxy for the ESDT system smart contract.
/// Unlike other contract proxies, this one has a fixed address,
/// so the proxy object doesn't really contain any data, it is more of a placeholder.
#[deprecated(
    since = "0.49.0",
    note = "There is a new `ESDTSystemSCProxy`, which uses the new proxy model."
)]
pub struct ESDTSystemSmartContractProxy<SA>
where
    SA: SendApi + 'static,
{
    _phantom: PhantomData<SA>,
}

impl<SA> ESDTSystemSmartContractProxy<SA>
where
    SA: SendApi + 'static,
{
    /// Constructor.
    /// TODO: consider moving this to a new Proxy constructor trait (bonus: better proxy constructor syntax).
    pub fn new_proxy_obj() -> Self {
        ESDTSystemSmartContractProxy {
            _phantom: PhantomData,
        }
    }
}

impl<SA> ESDTSystemSmartContractProxy<SA>
where
    SA: CallTypeApi + 'static,
{
    /// Produces a contract call to the ESDT system SC,
    /// which causes it to issue a new fungible ESDT token.
    pub fn issue_fungible(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        initial_supply: &BigUint<SA>,
        properties: FungibleTokenProperties,
    ) -> ContractCallWithEgld<SA, ()> {
        self.issue(
            issue_cost,
            EsdtTokenType::Fungible,
            token_display_name,
            token_ticker,
            initial_supply,
            TokenProperties {
                num_decimals: properties.num_decimals,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_transfer_create_role: false,
                can_mint: properties.can_mint,
                can_burn: properties.can_burn,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
        )
    }

    /// Produces a contract call to the ESDT system SC,
    /// which causes it to issue a new non-fungible ESDT token.
    pub fn issue_non_fungible(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        properties: NonFungibleTokenProperties,
    ) -> ContractCallWithEgld<SA, ()> {
        let zero = BigUint::zero();
        self.issue(
            issue_cost,
            EsdtTokenType::NonFungible,
            token_display_name,
            token_ticker,
            &zero,
            TokenProperties {
                num_decimals: 0,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_transfer_create_role: properties.can_transfer_create_role,
                can_mint: false,
                can_burn: false,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
        )
    }

    /// Produces a contract call to the ESDT system SC,
    /// which causes it to issue a new semi-fungible ESDT token.
    pub fn issue_semi_fungible(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        properties: SemiFungibleTokenProperties,
    ) -> ContractCallWithEgld<SA, ()> {
        let zero = BigUint::zero();
        self.issue(
            issue_cost,
            EsdtTokenType::SemiFungible,
            token_display_name,
            token_ticker,
            &zero,
            TokenProperties {
                num_decimals: 0,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_transfer_create_role: properties.can_transfer_create_role,
                can_mint: false,
                can_burn: false,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
        )
    }

    /// Produces a contract call to the ESDT system SC,
    /// which causes it to register a new Meta ESDT token.
    pub fn register_meta_esdt(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        properties: MetaTokenProperties,
    ) -> ContractCallWithEgld<SA, ()> {
        let zero = BigUint::zero();
        self.issue(
            issue_cost,
            EsdtTokenType::MetaFungible,
            token_display_name,
            token_ticker,
            &zero,
            TokenProperties {
                num_decimals: properties.num_decimals,
                can_freeze: properties.can_freeze,
                can_wipe: properties.can_wipe,
                can_pause: properties.can_pause,
                can_transfer_create_role: properties.can_transfer_create_role,
                can_mint: false,
                can_burn: false,
                can_change_owner: properties.can_change_owner,
                can_upgrade: properties.can_upgrade,
                can_add_special_roles: properties.can_add_special_roles,
            },
        )
    }

    pub fn issue_and_set_all_roles(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: ManagedBuffer<SA>,
        token_ticker: ManagedBuffer<SA>,
        token_type: EsdtTokenType,
        num_decimals: usize,
    ) -> ContractCallWithEgld<SA, ()> {
        let esdt_system_sc_address = self.esdt_system_sc_address();

        let token_type_name = match token_type {
            EsdtTokenType::Fungible => "FNG",
            EsdtTokenType::NonFungible
            | EsdtTokenType::NonFungibleV2
            | EsdtTokenType::DynamicNFT => "NFT",
            EsdtTokenType::SemiFungible | EsdtTokenType::DynamicSFT => "SFT",
            EsdtTokenType::MetaFungible | EsdtTokenType::DynamicMeta => "META",
            EsdtTokenType::Invalid => "",
        };

        let endpoint = match token_type {
            EsdtTokenType::Fungible
            | EsdtTokenType::NonFungible
            | EsdtTokenType::NonFungibleV2
            | EsdtTokenType::SemiFungible
            | EsdtTokenType::MetaFungible => ISSUE_AND_SET_ALL_ROLES_ENDPOINT_NAME,

            EsdtTokenType::DynamicNFT | EsdtTokenType::DynamicSFT | EsdtTokenType::DynamicMeta => {
                REGISTER_AND_SET_ALL_ROLES_DYNAMIC_ESDT_ENDPOINT_NAME
            }

            EsdtTokenType::Invalid => "",
        };

        let mut contract_call =
            ContractCallWithEgld::new(esdt_system_sc_address, endpoint, issue_cost)
                .argument(&token_display_name)
                .argument(&token_ticker)
                .argument(&token_type_name);

        if token_type != EsdtTokenType::DynamicNFT {
            contract_call = contract_call.argument(&num_decimals);
        }

        contract_call
    }

    /// Issues dynamic ESDT tokens
    pub fn issue_dynamic(
        self,
        issue_cost: BigUint<SA>,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        token_type: EsdtTokenType,
        num_decimals: usize,
    ) -> ContractCallWithEgld<SA, ()> {
        let esdt_system_sc_address = self.esdt_system_sc_address();

        let endpoint_name = match token_type {
            EsdtTokenType::DynamicNFT | EsdtTokenType::DynamicSFT | EsdtTokenType::DynamicMeta => {
                REGISTER_DYNAMIC_ESDT_ENDPOINT_NAME
            }
            _ => "",
        };

        let token_type_name = match token_type {
            EsdtTokenType::DynamicNFT => "NFT",
            EsdtTokenType::DynamicSFT => "SFT",
            EsdtTokenType::DynamicMeta => "META",
            _ => "",
        };

        let mut contract_call =
            ContractCallWithEgld::new(esdt_system_sc_address, endpoint_name, issue_cost);

        contract_call.proxy_arg(token_display_name);
        contract_call.proxy_arg(token_ticker);
        contract_call.proxy_arg(&token_type_name);

        if token_type != EsdtTokenType::DynamicNFT {
            contract_call.proxy_arg(&num_decimals);
        }

        contract_call
    }

    /// Deduplicates code from all the possible issue functions
    fn issue(
        self,
        issue_cost: BigUint<SA>,
        token_type: EsdtTokenType,
        token_display_name: &ManagedBuffer<SA>,
        token_ticker: &ManagedBuffer<SA>,
        initial_supply: &BigUint<SA>,
        properties: TokenProperties,
    ) -> ContractCallWithEgld<SA, ()> {
        let esdt_system_sc_address = self.esdt_system_sc_address();

        let endpoint_name = match token_type {
            EsdtTokenType::Fungible => ISSUE_FUNGIBLE_ENDPOINT_NAME,
            EsdtTokenType::NonFungible => ISSUE_NON_FUNGIBLE_ENDPOINT_NAME,
            EsdtTokenType::SemiFungible => ISSUE_SEMI_FUNGIBLE_ENDPOINT_NAME,
            EsdtTokenType::MetaFungible => REGISTER_META_ESDT_ENDPOINT_NAME,
            _ => "",
        };

        let mut contract_call =
            ContractCallWithEgld::new(esdt_system_sc_address, endpoint_name, issue_cost);

        contract_call.proxy_arg(token_display_name);
        contract_call.proxy_arg(token_ticker);

        if token_type == EsdtTokenType::Fungible {
            contract_call.proxy_arg(initial_supply);
            contract_call.proxy_arg(&properties.num_decimals);
        } else if token_type == EsdtTokenType::MetaFungible {
            contract_call.proxy_arg(&properties.num_decimals);
        }

        let mut token_prop_args = TokenPropertyArguments {
            can_freeze: Some(properties.can_freeze),
            can_wipe: Some(properties.can_wipe),
            can_pause: Some(properties.can_pause),
            can_change_owner: Some(properties.can_change_owner),
            can_upgrade: Some(properties.can_upgrade),
            can_add_special_roles: Some(properties.can_add_special_roles),
            ..TokenPropertyArguments::default()
        };

        if token_type == EsdtTokenType::Fungible {
            token_prop_args.can_mint = Some(properties.can_mint);
            token_prop_args.can_burn = Some(properties.can_burn);
        } else {
            token_prop_args.can_transfer_create_role = Some(properties.can_transfer_create_role);
        }

        append_token_property_arguments(&mut contract_call, &token_prop_args);

        contract_call
    }

    /// Produces a contract call to the ESDT system SC,
    /// which causes it to mint more fungible ESDT tokens.
    /// It will fail if the SC is not the owner of the token.
    pub fn mint(
        self,
        token_identifier: &TokenIdentifier<SA>,
        amount: &BigUint<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("mint")
            .argument(token_identifier)
            .argument(amount)
    }

    /// Produces a contract call to the ESDT system SC,
    /// which causes it to burn fungible ESDT tokens owned by the SC.
    pub fn burn(
        self,
        token_identifier: &TokenIdentifier<SA>,
        amount: &BigUint<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("ESDTBurn")
            .argument(token_identifier)
            .argument(amount)
    }

    /// The manager of an ESDT token may choose to suspend all transactions of the token,
    /// except minting, freezing/unfreezing and wiping.
    pub fn pause(self, token_identifier: &TokenIdentifier<SA>) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("pause")
            .argument(token_identifier)
    }

    /// The reverse operation of `pause`.
    pub fn unpause(self, token_identifier: &TokenIdentifier<SA>) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("unPause")
            .argument(token_identifier)
    }

    /// The manager of an ESDT token may freeze the tokens held by a specific account.
    /// As a consequence, no tokens may be transferred to or from the frozen account.
    /// Freezing and unfreezing the tokens of an account are operations designed to help token managers to comply with regulations.
    pub fn freeze(
        self,
        token_identifier: &TokenIdentifier<SA>,
        address: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("freeze")
            .argument(token_identifier)
            .argument(address)
    }

    /// The reverse operation of `freeze`, unfreezing, will allow further transfers to and from the account.
    pub fn unfreeze(
        self,
        token_identifier: &TokenIdentifier<SA>,
        address: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("unFreeze")
            .argument(token_identifier)
            .argument(address)
    }

    /// The manager of an ESDT token may wipe out all the tokens held by a frozen account.
    /// This operation is similar to burning the tokens, but the account must have been frozen beforehand,
    /// and it must be done by the token manager.
    /// Wiping the tokens of an account is an operation designed to help token managers to comply with regulations.
    pub fn wipe(
        self,
        token_identifier: &TokenIdentifier<SA>,
        address: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("wipe")
            .argument(token_identifier)
            .argument(address)
    }

    /// The manager of an ESDT token may freeze the NFT held by a specific Account.
    /// As a consequence, no NFT can be transferred to or from the frozen Account.
    /// Freezing and unfreezing a single NFT of an Account are operations designed to help token managers to comply with regulations.
    pub fn freeze_nft(
        self,
        token_identifier: &TokenIdentifier<SA>,
        nft_nonce: u64,
        address: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("freezeSingleNFT")
            .argument(token_identifier)
            .argument(&nft_nonce)
            .argument(address)
    }

    /// The reverse operation of `freeze`, unfreezing, will allow further transfers to and from the account.
    pub fn unfreeze_nft(
        self,
        token_identifier: &TokenIdentifier<SA>,
        nft_nonce: u64,
        address: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("unFreezeSingleNFT")
            .argument(token_identifier)
            .argument(&nft_nonce)
            .argument(address)
    }

    /// The manager of an ESDT token may wipe out a single NFT held by a frozen Account.
    /// This operation is similar to burning the quantity, but the Account must have been frozen beforehand,
    /// and it must be done by the token manager.
    /// Wiping the tokens of an Account is an operation designed to help token managers to comply with regulations.
    pub fn wipe_nft(
        self,
        token_identifier: &TokenIdentifier<SA>,
        nft_nonce: u64,
        address: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("wipeSingleNFT")
            .argument(token_identifier)
            .argument(&nft_nonce)
            .argument(address)
    }

    /// This function converts an SFT to a metaESDT by adding decimals to its structure in the metachain ESDT System SC.
    /// This function as almost all in case of ESDT can be called only by the owner.
    pub fn change_sft_to_meta_esdt(
        self,
        token_identifier: &TokenIdentifier<SA>,
        num_decimals: usize,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("changeSFTToMetaESDT")
            .argument(&token_identifier)
            .argument(&num_decimals)
    }

    /// This function can be called only if canSetSpecialRoles was set to true.
    /// The metachain system SC will evaluate the arguments and call “ESDTSetRole@tokenId@listOfRoles” for the given address.
    /// This will be actually a cross shard call.
    /// This function as almost all in case of ESDT can be called only by the owner.
    pub fn set_special_roles<RoleIter: Iterator<Item = EsdtLocalRole>>(
        self,
        address: &ManagedAddress<SA>,
        token_identifier: &TokenIdentifier<SA>,
        roles_iter: RoleIter,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self
            .esdt_system_sc_call_no_args("setSpecialRole")
            .argument(token_identifier)
            .argument(address);
        for role in roles_iter {
            if role != EsdtLocalRole::None {
                contract_call.push_raw_argument(role.as_role_name());
            }
        }

        contract_call
    }

    /// This function can be called only if canSetSpecialRoles was set to true.
    /// The metachain system SC will evaluate the arguments and call “ESDTUnsetRole@tokenId@listOfRoles” for the given address.
    /// This will be actually a cross shard call.
    /// This function as almost all in case of ESDT can be called only by the owner.
    pub fn unset_special_roles<RoleIter: Iterator<Item = EsdtLocalRole>>(
        self,
        address: &ManagedAddress<SA>,
        token_identifier: &TokenIdentifier<SA>,
        roles_iter: RoleIter,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self
            .esdt_system_sc_call_no_args("unSetSpecialRole")
            .argument(token_identifier)
            .argument(address);
        for role in roles_iter {
            if role != EsdtLocalRole::None {
                contract_call.push_raw_argument(role.as_role_name());
            }
        }

        contract_call
    }

    pub fn transfer_ownership(
        self,
        token_identifier: &TokenIdentifier<SA>,
        new_owner: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("transferOwnership")
            .argument(token_identifier)
            .argument(new_owner)
    }

    pub fn transfer_nft_create_role(
        self,
        token_identifier: &TokenIdentifier<SA>,
        old_creator: &ManagedAddress<SA>,
        new_creator: &ManagedAddress<SA>,
    ) -> ContractCallNoPayment<SA, ()> {
        self.esdt_system_sc_call_no_args("transferNFTCreateRole")
            .argument(token_identifier)
            .argument(old_creator)
            .argument(new_creator)
    }

    pub fn control_changes(
        self,
        token_identifier: &TokenIdentifier<SA>,
        property_arguments: &TokenPropertyArguments,
    ) -> ContractCallNoPayment<SA, ()> {
        let mut contract_call = self
            .esdt_system_sc_call_no_args("controlChanges")
            .argument(token_identifier);
        append_token_property_arguments(&mut contract_call, property_arguments);
        contract_call
    }

    pub fn esdt_system_sc_address(&self) -> ManagedAddress<SA> {
        ESDTSystemSCAddress.to_managed_address()
    }

    fn esdt_system_sc_call_no_args(
        self,
        endpoint_name: &'static str,
    ) -> ContractCallNoPayment<SA, ()> {
        let esdt_system_sc_address = self.esdt_system_sc_address();
        ContractCallNoPayment::new(esdt_system_sc_address, endpoint_name)
    }
}

const TRUE_STR: &str = "true";
const FALSE_STR: &str = "false";

fn bool_name_bytes(b: bool) -> &'static str {
    if b {
        TRUE_STR
    } else {
        FALSE_STR
    }
}

fn set_token_property<SA, CC>(contract_call: &mut CC, name: &str, value: bool)
where
    SA: CallTypeApi + 'static,
    CC: ContractCall<SA>,
{
    contract_call.push_raw_argument(name);
    contract_call.push_raw_argument(bool_name_bytes(value));
}

fn append_token_property_arguments<SA, CC>(
    contract_call: &mut CC,
    token_prop_args: &TokenPropertyArguments,
) where
    SA: CallTypeApi + 'static,
    CC: ContractCall<SA>,
{
    if let Some(can_freeze) = token_prop_args.can_freeze {
        set_token_property(contract_call, "canFreeze", can_freeze);
    }

    if let Some(can_wipe) = token_prop_args.can_wipe {
        set_token_property(contract_call, "canWipe", can_wipe);
    }

    if let Some(can_pause) = token_prop_args.can_pause {
        set_token_property(contract_call, "canPause", can_pause);
    }

    if let Some(can_transfer_create_role) = token_prop_args.can_transfer_create_role {
        set_token_property(
            contract_call,
            "canTransferNFTCreateRole",
            can_transfer_create_role,
        );
    }

    if let Some(can_mint) = token_prop_args.can_mint {
        set_token_property(contract_call, "canMint", can_mint);
    }

    if let Some(can_burn) = token_prop_args.can_burn {
        set_token_property(contract_call, "canBurn", can_burn);
    }

    if let Some(can_change_owner) = token_prop_args.can_change_owner {
        set_token_property(contract_call, "canChangeOwner", can_change_owner);
    }

    if let Some(can_upgrade) = token_prop_args.can_upgrade {
        set_token_property(contract_call, "canUpgrade", can_upgrade);
    }

    if let Some(can_add_special_roles) = token_prop_args.can_add_special_roles {
        set_token_property(contract_call, "canAddSpecialRoles", can_add_special_roles);
    }
}
