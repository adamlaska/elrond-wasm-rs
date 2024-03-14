use std::{fs::File, io::Write};

use crate::cmd::contract::generate_snippets::snippet_gen_common::write_newline;

use super::proxy_naming::{proxy_methods_type_name, proxy_type_name};

const PREFIX_AUTO_GENERATED: &str = "////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////
";

const IMPORTS: &str = "#![allow(clippy::all)]

use multiversx_sc::imports::*;";

pub(crate) fn write_header(file: &mut File) {
    writeln!(file, "{PREFIX_AUTO_GENERATED}").unwrap();
    writeln!(file, r#"{IMPORTS}"#).unwrap();

    write_newline(file);
}

pub(crate) fn write_struct_template(file: &mut File, name: &String) {
    let proxy_type_name = proxy_type_name(name);
    writeln!(file, "pub struct {proxy_type_name};").unwrap();
    write_newline(file)
}

pub(crate) fn write_impl_for_tx_proxy(file: &mut File, name: &String) {
    let proxy_type_name = proxy_type_name(name);
    let proxy_methods_type_name = proxy_methods_type_name(name);
    writeln!(
        file,
        r#"impl<Env, From, To, Gas> TxProxyTrait<Env, From, To, Gas> for {proxy_type_name}
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    type TxProxyMethods = {proxy_methods_type_name}<Env, From, To, Gas>;

    fn proxy_methods(self, tx: Tx<Env, From, To, (), Gas, (), ()>) -> Self::TxProxyMethods {{
        {proxy_methods_type_name} {{ wrapped_tx: tx }}
    }}
}}"#
    )
    .unwrap();

    write_newline(file);
}

pub(crate) fn write_struct_tx_proxy_methods(file: &mut File, name: &String) {
    let proxy_methods_type_name = proxy_methods_type_name(name);
    writeln!(
        file,
        r#"pub struct {proxy_methods_type_name}<Env, From, To, Gas>
where
    Env: TxEnv,
    From: TxFrom<Env>,
    To: TxTo<Env>,
    Gas: TxGas<Env>,
{{
    wrapped_tx: Tx<Env, From, To, (), Gas, (), ()>,
}}"#
    )
    .unwrap();

    write_newline(file);
}
