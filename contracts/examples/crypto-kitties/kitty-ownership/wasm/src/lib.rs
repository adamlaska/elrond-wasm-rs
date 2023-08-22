// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           21
// Async Callback:                       1
// Total number of exported functions:  23

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    kitty_ownership
    (
        init => init
        setGeneScienceContractAddress => set_gene_science_contract_address_endpoint
        setKittyAuctionContractAddress => set_kitty_auction_contract_address_endpoint
        claim => claim
        totalSupply => total_supply
        balanceOf => balance_of
        ownerOf => owner_of
        approve => approve
        transfer => transfer
        transfer_from => transfer_from
        tokensOfOwner => tokens_of_owner
        allowAuctioning => allow_auctioning
        approveSiringAndReturnKitty => approve_siring_and_return_kitty
        createGenZeroKitty => create_gen_zero_kitty
        getKittyById => get_kitty_by_id_endpoint
        isReadyToBreed => is_ready_to_breed
        isPregnant => is_pregnant
        canBreedWith => can_breed_with
        approveSiring => approve_siring
        breedWith => breed_with
        giveBirth => give_birth
        birthFee => birth_fee
    )
}

multiversx_sc_wasm_adapter::async_callback! { kitty_ownership }
