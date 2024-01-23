// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                            9
// Async Callback:                       1
// Total number of exported functions:  11

#![no_std]
#![allow(internal_features)]
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    kitty_auction
    (
        init => init
        setKittyOwnershipContractAddress => set_kitty_ownership_contract_address_endpoint
        createAndAuctionGenZeroKitty => create_and_auction_gen_zero_kitty
        isUpForAuction => is_up_for_auction
        getAuctionStatus => get_auction_status
        getCurrentWinningBid => get_current_winning_bid
        createSaleAuction => create_sale_auction
        createSiringAuction => create_siring_auction
        bid => bid
        endAuction => end_auction
    )
}

multiversx_sc_wasm_adapter::async_callback! { kitty_auction }
