// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           16
// Async Callback (empty):               1
// Total number of exported functions:  18

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    formatted_message_features
    (
        init => init
        static_message => static_message
        dynamic_message => dynamic_message
        dynamic_message_hex => dynamic_message_hex
        dynamic_message_multiple => dynamic_message_multiple
        dynamic_message_ascii => dynamic_message_ascii
        decode_error_message => decode_error_message
        print_message => print_message
        print_message_hex => print_message_hex
        print_message_binary => print_message_binary
        print_message_codec => print_message_codec
        format_message_one_part => format_message_one_part
        format_message_multiple_parts => format_message_multiple_parts
        format_message_big_int => format_message_big_int
        format_message_i64 => format_message_i64
        format_message_managed_buffer => format_message_managed_buffer
        format_message_managed_buffer_hex => format_message_managed_buffer_hex
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
