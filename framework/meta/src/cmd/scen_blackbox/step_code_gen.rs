#![allow(unused_imports)] // TODO: Remove after all code is implemented

use std::collections::BTreeMap;

use multiversx_sc_scenario::scenario::model::{
    Account, AddressKey, AddressValue, BigUintValue, BlockInfo, BytesKey, BytesValue, CheckAccount,
    CheckAccounts, CheckStateStep, CheckStorage, CheckStorageDetails, CheckValue, NewAddress,
    ScCallStep, ScDeployStep, ScQueryStep, SetStateStep, Step, TxCall, TxDeploy, TxESDT, TxExpect,
    TxQuery,
};
use multiversx_sc_scenario::scenario_format::serde_raw::ValueSubTree;

use super::{
    num_format,
    scenario_loader::scenario_to_function_name,
    test_gen::{ConstGroup, TestGenerator},
};

impl<'a> TestGenerator<'a> {
    /// Generates code for a single step
    pub fn generate_step_code(&mut self, step: &Step) {
        match step {
            Step::ExternalSteps(step_data) => {
                self.generate_external_steps(&step_data.path, step_data.comment.as_deref());
            }
            Step::SetState(set_state) => {
                self.generate_set_state(set_state);
            }
            Step::ScDeploy(sc_deploy) => {
                self.generate_sc_deploy(sc_deploy);
            }
            Step::ScCall(sc_call) => {
                self.generate_sc_call(sc_call);
            }
            Step::ScQuery(sc_query) => {
                self.generate_sc_query(sc_query);
            }
            Step::CheckState(check_state) => {
                self.generate_check_state(check_state.comment.as_deref(), &check_state.accounts);
            }
            Step::Transfer(_transfer) => {
                self.step_writeln("    // TODO: Transfer step");
            }
            Step::ValidatorReward(_) => {
                self.step_writeln("    // TODO: ValidatorReward step");
            }
            Step::DumpState(_) => {
                self.step_writeln("    // TODO: DumpState step");
            }
        }
    }

    fn generate_external_steps(&mut self, path: &str, comment: Option<&str>) {
        if let Some(comment_text) = comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        let scenario_name = std::path::Path::new(path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(path);

        let steps_function_name = format!("{}_steps", scenario_to_function_name(scenario_name));

        self.step_writeln(format!("    {}(world);", steps_function_name));
        self.step_writeln("");
    }

    fn generate_set_state(&mut self, set_state: &SetStateStep) {
        if let Some(comment_text) = &set_state.comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        // Generate current block info
        if let Some(block_info) = set_state.current_block_info.as_ref() {
            self.generate_block_info(block_info);
        }

        // Generate account setup
        for (address_key, account) in &set_state.accounts {
            let address_expr = self.format_address(&address_key.original);

            // Check if we need to set anything
            let has_nonce = account
                .nonce
                .as_ref()
                .map(|v| !Self::is_default_value(&v.original))
                .unwrap_or(false);
            let has_balance = account
                .balance
                .as_ref()
                .map(|v| !Self::is_default_value(&v.original))
                .unwrap_or(false);
            let has_esdt = !account.esdt.is_empty();

            if has_nonce || has_balance || has_esdt {
                self.step_write(format!("    world.account({})", address_expr));

                if has_nonce {
                    if let Some(nonce) = &account.nonce {
                        self.step_writeln(format!(
                            ".nonce({})",
                            Self::format_nonce_value(&nonce.original)
                        ));
                        self.step_write("        ");
                    }
                }

                if has_balance {
                    if let Some(balance) = &account.balance {
                        self.step_writeln(format!(
                            ".balance({})",
                            Self::format_balance_value(&balance.original)
                        ));
                        self.step_write("        ");
                    }
                }

                for (token_key, esdt) in &account.esdt {
                    let token_const = self.format_token_id_from_key(token_key);
                    self.generate_esdt_balance_calls(&token_const, esdt);
                }

                self.step_writeln(";");
            }
        }

        // Store new addresses for later use in deploy steps
        for new_addr in &set_state.new_addresses {
            let creator_key = new_addr.creator_address.original.to_concatenated_string();
            let new_address_key = new_addr.new_address.original.to_concatenated_string();
            self.new_address_map.insert(creator_key, new_address_key);
        }

        self.step_writeln("");
    }

    /// Generates `world.current_block().block_timestamp_millis(...)` and similar block info setters.
    fn generate_block_info(&mut self, block_info: &BlockInfo) {
        // blockTimestampMs takes priority over blockTimestamp
        if let Some(ref ts_ms) = block_info.block_timestamp_ms {
            let value = num_format::format_unsigned(&ts_ms.value.to_be_bytes(), "u64");
            self.step_writeln(format!(
                "    world.current_block().block_timestamp_millis(TimestampMillis::new({}));",
                value
            ));
        } else if let Some(ref ts) = block_info.block_timestamp {
            let value = num_format::format_unsigned(&ts.value.to_be_bytes(), "u64");
            self.step_writeln(format!(
                "    world.current_block().block_timestamp_seconds(TimestampSeconds::new({}));",
                value
            ));
        }

        if let Some(ref nonce) = block_info.block_nonce {
            self.step_writeln(format!(
                "    world.current_block().block_nonce({}u64);",
                nonce.value
            ));
        }

        if let Some(ref round) = block_info.block_round {
            self.step_writeln(format!(
                "    world.current_block().block_round({}u64);",
                round.value
            ));
        }

        if let Some(ref epoch) = block_info.block_epoch {
            self.step_writeln(format!(
                "    world.current_block().block_epoch({}u64);",
                epoch.value
            ));
        }
    }

    fn generate_sc_deploy(&mut self, sc_deploy: &ScDeployStep) {
        let tx = &sc_deploy.tx;

        if let Some(comment_text) = &sc_deploy.comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        self.step_writeln("    world");
        self.step_writeln("        .tx()");

        if let Some(id_val) = &sc_deploy.tx_id {
            self.step_writeln(format!("        .id(\"{}\")", id_val));
        }

        let from_addr = self.format_address_value(&tx.from);
        self.step_writeln(format!("        .from({})", from_addr));
        self.step_write("        ");

        let proxy_type = self.generate_proxy_type();
        self.step_writeln(format!(".typed({})", proxy_type));
        self.step_write("        ");

        let inputs = self.find_constructor_inputs();
        let formatted_args = self.format_args(&tx.arguments, inputs.as_deref());
        self.step_write(".init(");
        for (i, formatted) in formatted_args.iter().enumerate() {
            if i > 0 {
                self.step_write(", ");
            }
            self.step_write(formatted);
        }
        self.step_writeln(")");
        self.step_write("        ");

        // Generate EGLD payment for deploy (no ESDT support on deploy)
        self.generate_egld_payment(&tx.egld_value);

        // Generate code path from the contract_code field
        let code_path_expr = tx.contract_code.original.to_concatenated_string();
        let code_path_const = self.format_code_path(&code_path_expr);
        self.step_writeln(format!(".code({})", code_path_const));
        self.step_write("        ");

        // Add new_address if we have a prediction from setState
        let from_address = tx.from.original.to_concatenated_string();
        if let Some(new_address) = self.new_address_map.get(&from_address).cloned() {
            // Format as TestSCAddress::new("name") if it's sc:name
            let address_expr = self.format_address(&new_address);
            self.step_writeln(format!(".new_address({})", address_expr));
            self.step_write("        ");
        }

        self.generate_expect_error(sc_deploy.expect.as_ref());

        self.step_writeln(".run();");
        self.step_writeln("");
    }

    fn generate_sc_call(&mut self, sc_call: &ScCallStep) {
        let tx = &sc_call.tx;

        if let Some(comment_text) = &sc_call.comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        self.step_writeln("    world");
        self.step_writeln("        .tx()");

        if let Some(id_val) = &sc_call.tx_id {
            self.step_writeln(format!("        .id(\"{}\")", id_val));
        }

        let from_addr = self.format_address_value(&tx.from);
        self.step_writeln(format!("        .from({})", from_addr));

        let to_addr = self.format_address_value(&tx.to);
        self.step_writeln(format!("        .to({})", to_addr));
        self.step_write("        ");

        let proxy_type = self.generate_proxy_type();
        self.step_writeln(format!(".typed({})", proxy_type));
        self.step_write("        ");

        // Map the endpoint name from scenario to Rust method name
        let inputs = self.find_endpoint_inputs(&tx.function);
        let formatted_args = self.format_args(&tx.arguments, inputs.as_deref());
        let rust_method_name = self.map_endpoint_name(&tx.function);
        self.step_write(format!(".{}(", rust_method_name));
        for (i, formatted) in formatted_args.iter().enumerate() {
            if i > 0 {
                self.step_write(", ");
            }
            self.step_write(formatted);
        }
        self.step_writeln(")");
        self.step_write("        ");

        // Generate payments
        self.generate_payments(&tx.egld_value, &tx.esdt_value);

        self.generate_expect_error(sc_call.expect.as_ref());

        self.step_writeln(".run();");
        self.step_writeln("");
    }

    /// Generates `.with_result(ExpectError(status, "message"))` when the expected status is non-zero.
    fn generate_expect_error(&mut self, expect: Option<&TxExpect>) {
        let Some(expect_val) = expect else {
            return;
        };

        // Extract status code; skip if it's "*" or 0
        let status_code = match &expect_val.status {
            CheckValue::Equal(u64_val) => u64_val.value,
            CheckValue::Star => return,
        };

        if status_code == 0 {
            return;
        }

        // Extract message string
        let message = match &expect_val.message {
            CheckValue::Equal(bytes_val) => {
                match &bytes_val.original {
                    ValueSubTree::Str(s) => {
                        // Strip "str:" prefix if present
                        s.strip_prefix("str:").unwrap_or(s).to_string()
                    }
                    _ => String::new(),
                }
            }
            CheckValue::Star => String::new(),
        };

        self.step_writeln(format!(
            ".with_result(ExpectError({}, \"{}\"))",
            status_code,
            Self::escape_string(&message)
        ));
        self.step_write("        ");
    }

    /// Generates `.payment(...)` calls for EGLD and ESDT transfers.
    fn generate_payments(&mut self, egld_value: &BigUintValue, esdt_value: &[TxESDT]) {
        use multiversx_sc_scenario::num_bigint::BigUint;

        if !esdt_value.is_empty() {
            // ESDT payments (may include EGLD-000000)
            for esdt in esdt_value {
                self.generate_esdt_payment(esdt);
            }
        } else if egld_value.value > BigUint::from(0u32) {
            // Plain EGLD payment
            self.generate_egld_payment(egld_value);
        }
    }

    /// Generates a `.payment(Payment::try_new(...).unwrap())` call for an EGLD value.
    fn generate_egld_payment(&mut self, egld_value: &BigUintValue) {
        use multiversx_sc_scenario::num_bigint::BigUint;

        if egld_value.value > BigUint::from(0u32) {
            let amount = Self::format_biguint_value(&egld_value.value);
            self.step_writeln(format!(
                ".payment(Payment::try_new(TestTokenId::EGLD_000000, 0, {}).unwrap())",
                amount
            ));
            self.step_write("        ");
        }
    }

    /// Generates a `.payment(Payment::try_new(...).unwrap())` call for an ESDT transfer.
    fn generate_esdt_payment(&mut self, esdt: &TxESDT) {
        let nonce = esdt.nonce.value;
        let amount = Self::format_biguint_value(&esdt.esdt_value.value);

        if esdt.is_egld() {
            self.step_writeln(format!(
                ".payment(Payment::try_new(TestTokenId::EGLD_000000, {}, {}).unwrap())",
                nonce, amount
            ));
        } else {
            let token_const = self.format_token_id(&esdt.esdt_token_identifier);
            self.step_writeln(format!(
                ".payment(Payment::try_new({}, {}, {}).unwrap())",
                token_const, nonce, amount
            ));
        }
        self.step_write("        ");
    }

    /// Formats a token identifier from a BytesValue into a constant reference.
    /// Generates a `TestTokenId` constant if one doesn't already exist.
    fn format_token_id(&mut self, token_id: &BytesValue) -> String {
        let original_str = match &token_id.original {
            ValueSubTree::Str(s) => s.as_str(),
            _ => return format!("ScenarioValueRaw::new(\"{:?}\")", token_id.value),
        };

        self.format_token_id_str(original_str)
    }

    /// Formats a token identifier from a BytesKey (used in setState ESDT maps).
    fn format_token_id_from_key(&mut self, key: &BytesKey) -> String {
        self.format_token_id_str(&key.original)
    }

    /// Core token ID formatting logic, shared by `format_token_id` and `format_token_id_from_key`.
    fn format_token_id_str(&mut self, original_str: &str) -> String {
        // Check if we already have a constant for this token
        if let Some(const_name) = self.token_id_map.get(original_str) {
            return const_name.clone();
        }

        // Strip "str:" prefix if present
        let name = original_str.strip_prefix("str:").unwrap_or(original_str);

        // Use the built-in constant for EGLD-000000
        if name == "EGLD-000000" {
            let const_name = "TestTokenId::EGLD_000000".to_string();
            self.token_id_map
                .insert(original_str.to_string(), const_name.clone());
            return const_name;
        }

        // Generate constant name: "TOK-123456" -> "TOK_123456"
        let const_name = name.to_uppercase().replace('-', "_");

        self.add_const(
            const_name.clone(),
            ConstGroup::TokenId,
            "TestTokenId".to_string(),
            format!("TestTokenId::new(\"{}\")", name),
        );

        self.token_id_map
            .insert(original_str.to_string(), const_name.clone());

        const_name
    }

    /// Generates `.esdt_balance(token, amount)` or `.esdt_nft_balance(token, nonce, amount, ())`
    /// calls depending on whether the ESDT instances have non-zero nonces.
    fn generate_esdt_balance_calls(
        &mut self,
        token_const: &str,
        esdt: &multiversx_sc_scenario::scenario::model::Esdt,
    ) {
        use multiversx_sc_scenario::scenario::model::Esdt;
        match esdt {
            Esdt::Short(biguint_val) => {
                let amount = Self::format_biguint_value(&biguint_val.value);
                self.step_writeln(format!(".esdt_balance({}, {})", token_const, amount));
                self.step_write("        ");
            }
            Esdt::Full(esdt_obj) => {
                for instance in &esdt_obj.instances {
                    let nonce = instance.nonce.as_ref().map_or(0, |n| n.value);
                    let amount = instance
                        .balance
                        .as_ref()
                        .map(|b| Self::format_biguint_value(&b.value))
                        .unwrap_or_else(|| "0u64".to_string());

                    if nonce == 0 {
                        self.step_writeln(format!(".esdt_balance({}, {})", token_const, amount));
                    } else {
                        self.step_writeln(format!(
                            ".esdt_nft_balance({}, {}, {}, ())",
                            token_const, nonce, amount
                        ));
                    }
                    self.step_write("        ");
                }
            }
        }
    }

    /// Formats a 32-byte H256 value as a named constant.
    /// Generates a `const H256_N: H256 = H256::from_hex("...");` declaration.
    fn format_h256(&mut self, arg: &BytesValue) -> String {
        let hex_str = hex::encode(&arg.value);

        // Check if we already have a constant for this value
        if let Some(const_name) = self.h256_map.get(&hex_str) {
            return const_name.clone();
        }

        self.h256_counter += 1;
        let const_name = format!("H256_{}", self.h256_counter);

        self.add_const(
            const_name.clone(),
            ConstGroup::Hash,
            "H256".to_string(),
            format!("H256::from_hex(\"{}\")", hex_str),
        );

        self.h256_map.insert(hex_str, const_name.clone());

        const_name
    }

    /// Parses an ABI type name like `"array32<u8>"` and returns the array size.
    fn parse_array_type(abi_type: &str) -> Option<usize> {
        let rest = abi_type.strip_prefix("array")?;
        let size_str = rest.strip_suffix("<u8>")?;
        size_str.parse::<usize>().ok()
    }

    /// Formats a fixed-size byte array as a named constant using `hex!()`.
    /// Generates `const HEX_{size}_{N}: [u8; {size}] = hex!("...");`
    /// and returns `&HEX_{size}_{N}`.
    fn format_byte_array(&mut self, arg: &BytesValue, size: usize) -> String {
        let hex_str = hex::encode(&arg.value);

        // Check if we already have a constant for this value
        if let Some(const_name) = self.hex_array_map.get(&hex_str) {
            return format!("&{}", const_name);
        }

        let counter = self.hex_array_counter.entry(size).or_insert(0);
        *counter += 1;
        let const_name = format!("HEX_{}_{}", size, counter);

        self.add_const(
            const_name.clone(),
            ConstGroup::ByteArray,
            format!("[u8; {}]", size),
            format!("hex!(\"{}\")", hex_str),
        );

        self.hex_array_map.insert(hex_str, const_name.clone());

        format!("&{}", const_name)
    }

    /// Formats a BigUint value for use as a payment amount.
    fn format_biguint_value(value: &multiversx_sc_scenario::num_bigint::BigUint) -> String {
        let bytes = value.to_bytes_be();
        num_format::format_unsigned(&bytes, "BigUint")
    }

    fn generate_sc_query(&mut self, sc_query: &ScQueryStep) {
        let tx = &sc_query.tx;
        let expect = sc_query.expect.as_ref();

        if let Some(comment_text) = &sc_query.comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        self.step_writeln("    world");
        self.step_writeln("        .query()");

        if let Some(id_val) = &sc_query.tx_id {
            self.step_writeln(format!("        .id(\"{}\")", id_val));
        }

        let to_addr = self.format_address_value(&tx.to);
        self.step_writeln(format!("        .to({})", to_addr));
        self.step_write("        ");

        let proxy_type = self.generate_proxy_type();
        self.step_writeln(format!(".typed({})", proxy_type));
        self.step_write("        ");

        // Map the endpoint name from scenario to Rust method name
        let inputs = self.find_endpoint_inputs(&tx.function);
        let formatted_args = self.format_args(&tx.arguments, inputs.as_deref());
        let rust_method_name = self.map_endpoint_name(&tx.function);
        self.step_write(format!(".{}(", rust_method_name));
        for (i, formatted) in formatted_args.iter().enumerate() {
            if i > 0 {
                self.step_write(", ");
            }
            self.step_write(formatted);
        }
        self.step_writeln(")");
        self.step_write("        ");

        // Add returns if we have expected output
        if let Some(expect_val) = expect {
            if let CheckValue::Equal(ref out_values) = expect_val.out {
                self.step_write(".returns(ExpectValue(");
                for (i, out) in out_values.iter().enumerate() {
                    if i > 0 {
                        self.step_write(", ");
                    }
                    self.step_write(Self::format_check_value(out));
                }
                self.step_writeln("))");
                self.step_write("        ");
            }
        }

        self.step_writeln(".run();");
        self.step_writeln("");
    }

    fn generate_check_state(&mut self, comment: Option<&str>, accounts: &CheckAccounts) {
        if let Some(comment_text) = comment {
            self.step_writeln(format!("    // {}", comment_text));
        }

        for (address_key, account) in &accounts.accounts {
            let address_expr = self.format_address(&address_key.original);

            // Check if we need to check storage
            if let CheckStorage::Equal(ref storage_details) = account.storage {
                if !storage_details.storages.is_empty() {
                    self.step_writeln(format!("    world.check_account({})", address_expr));

                    for (key, value) in &storage_details.storages {
                        let value_str = Self::format_check_value_for_storage(value);
                        self.step_writeln(format!(
                            "        .check_storage(\"{}\", \"{}\")",
                            key.original, value_str
                        ));
                    }

                    self.step_writeln("        ;");
                }
            }
        }

        self.step_writeln("");
    }

    pub(super) fn format_address(&mut self, addr: &str) -> String {
        // Remove quotes if present
        let clean = addr.trim_matches('"');

        // Handle address: and sc: prefixes
        if let Some(name) = clean.strip_prefix("address:") {
            // Check if we already have a constant for this address
            if let Some(const_name) = self.test_address_map.get(addr) {
                return const_name.clone();
            }
            // Generate new constant name
            let const_name = Self::test_address_to_const_name(name);
            self.add_const(
                const_name.clone(),
                ConstGroup::Address,
                "TestAddress".to_string(),
                format!("TestAddress::new(\"{}\")", name),
            );
            self.test_address_map
                .insert(addr.to_string(), const_name.clone());
            const_name
        } else if let Some(name) = clean.strip_prefix("sc:") {
            // Check if we already have a constant for this address
            if let Some(const_name) = self.test_address_map.get(addr) {
                return const_name.clone();
            }
            // Generate new constant name
            let const_name = Self::test_address_to_const_name(name);
            self.add_const(
                const_name.clone(),
                ConstGroup::Address,
                "TestSCAddress".to_string(),
                format!("TestSCAddress::new(\"{}\")", name),
            );
            self.test_address_map
                .insert(addr.to_string(), const_name.clone());
            const_name
        } else if clean.starts_with("0x") || clean.starts_with("0X") {
            // Hex address - check if we already have a constant for it
            if let Some(const_name) = self.hex_address_map.get(clean) {
                return const_name.clone();
            }
            // Generate new constant name
            self.hex_address_counter += 1;
            let const_name = format!("ADDRESS_HEX_{}", self.hex_address_counter);
            self.add_const(
                const_name.clone(),
                ConstGroup::Address,
                "Address".to_string(),
                format!("Address::from_hex(\"{}\")", clean),
            );
            self.hex_address_map
                .insert(clean.to_string(), const_name.clone());
            const_name
        } else if clean.len() == 64 && clean.chars().all(|c| c.is_ascii_hexdigit()) {
            // Hex address without 0x prefix - check if we already have a constant for it
            if let Some(const_name) = self.hex_address_map.get(clean) {
                return const_name.clone();
            }
            // Generate new constant name
            self.hex_address_counter += 1;
            let const_name = format!("ADDRESS_HEX_{}", self.hex_address_counter);
            self.add_const(
                const_name.clone(),
                ConstGroup::Address,
                "Address".to_string(),
                format!("Address::from_hex(\"{}\")", clean),
            );
            self.hex_address_map
                .insert(clean.to_string(), const_name.clone());
            const_name
        } else {
            // Raw address - wrap in ScenarioValueRaw
            format!("ScenarioValueRaw::new(\"{}\")", clean)
        }
    }

    pub(super) fn format_address_value(&mut self, value: &AddressValue) -> String {
        match &value.original {
            ValueSubTree::Str(s) => self.format_address(s),
            _ => {
                // Fallback for non-string addresses
                Self::format_value(&value.original)
            }
        }
    }

    fn format_value_subtree(value: &ValueSubTree) -> String {
        match value {
            ValueSubTree::Str(s) => {
                format!(
                    "ValueSubTree::Str(\"{}\".to_string())",
                    Self::escape_string(s)
                )
            }
            ValueSubTree::List(items) => {
                if items.is_empty() {
                    "ValueSubTree::List(vec![])".to_string()
                } else {
                    let formatted_items: Vec<String> =
                        items.iter().map(Self::format_value_subtree).collect();
                    format!("ValueSubTree::List(vec![{}])", formatted_items.join(", "))
                }
            }
            ValueSubTree::Map(map) => {
                if map.is_empty() {
                    "ValueSubTree::Map(BTreeMap::new())".to_string()
                } else {
                    let formatted_entries: Vec<String> = map
                        .iter()
                        .map(|(k, v)| {
                            format!(
                                "(\"{}\".to_string(), {})",
                                Self::escape_string(k),
                                Self::format_value_subtree(v)
                            )
                        })
                        .collect();
                    format!(
                        "ValueSubTree::Map(BTreeMap::from([{}]))",
                        formatted_entries.join(", ")
                    )
                }
            }
        }
    }

    fn format_value(value: &ValueSubTree) -> String {
        match value {
            ValueSubTree::Str(s) => {
                format!("ScenarioValueRaw::new(\"{}\")", Self::escape_string(s))
            }
            _ => {
                format!(
                    "ScenarioValueRaw::new({})",
                    Self::format_value_subtree(value)
                )
            }
        }
    }

    fn format_check_value(value: &CheckValue<BytesValue>) -> String {
        match value {
            CheckValue::Star => "ScenarioValueRaw::new(\"*\")".to_string(),
            CheckValue::Equal(v) => Self::format_value(&v.original),
        }
    }

    fn format_check_value_for_storage(value: &CheckValue<BytesValue>) -> String {
        match value {
            CheckValue::Star => "*".to_string(),
            CheckValue::Equal(v) => Self::format_value_as_string(&v.original),
        }
    }

    fn escape_string(s: &str) -> String {
        s.replace('\\', "\\\\").replace('"', "\\\"")
    }

    /// Formats an argument value based on ABI type info and raw bytes.
    ///
    /// Uses ABI type information to generate idiomatic Rust literals where possible.
    /// For types whose ABI name is ambiguous (e.g. time types all map to "u64"),
    /// the Rust type name is checked instead.
    /// Falls back to `ScenarioValueRaw::new` for unrecognized types.
    fn format_arg_value(
        &mut self,
        type_names: &multiversx_sc::abi::TypeNames,
        arg: &BytesValue,
    ) -> String {
        // Then match on ABI type name for all other known types.
        match type_names.abi.as_str() {
            "bool" => {
                let is_true = arg.value.len() == 1 && arg.value[0] == 1;
                if is_true {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            }
            "u8" | "u16" | "u32" | "u64" | "usize" | "BigUint" => {
                num_format::format_unsigned(&arg.value, &type_names.abi)
            }
            "i8" | "i16" | "i32" | "i64" | "isize" | "BigInt" => {
                num_format::format_signed(&arg.value, &type_names.abi)
            }
            "TokenIdentifier" | "EgldOrEsdtTokenIdentifier" | "TokenId" => {
                self.format_token_id(arg)
            }
            "H256" if arg.value.len() == 32 => self.format_h256(arg),
            "TimestampMillis" | "TimestampSeconds" | "DurationMillis" | "DurationSeconds" => {
                let inner = num_format::format_unsigned(&arg.value, "u64");
                format!("{}::new({})", type_names.abi, inner)
            }
            // TODO: add more type cases here
            _ => {
                if let Some(size) = Self::parse_array_type(&type_names.abi) {
                    if arg.value.len() == size {
                        return self.format_byte_array(arg, size);
                    }
                }
                Self::format_value(&arg.original)
            }
        }
    }

    /// Looks up the ABI inputs for an endpoint by its scenario name.
    fn find_endpoint_inputs(
        &self,
        endpoint_name: &str,
    ) -> Option<Vec<multiversx_sc::abi::InputAbi>> {
        self.abi
            .endpoints
            .iter()
            .find(|e| e.name == endpoint_name)
            .map(|e| e.inputs.clone())
    }

    /// Looks up the ABI inputs for the constructor.
    fn find_constructor_inputs(&self) -> Option<Vec<multiversx_sc::abi::InputAbi>> {
        self.abi.constructors.first().map(|e| e.inputs.clone())
    }

    /// Formats a list of arguments, using ABI type info when available.
    ///
    /// Handles `variadic<T>` and `multi<A,B,...>` types:
    /// - A `variadic<T>` input consumes all remaining scenario arguments, wrapping them
    ///   in `MultiValueVec::from(vec![...])`.
    /// - If the inner type is `multi<A,B,...>`, arguments are taken in groups matching
    ///   the number of multi fields, each group wrapped in `MultiValueN::new(...)`.
    /// - For all other (scalar) types, delegates to `format_arg_value`.
    fn format_args(
        &mut self,
        args: &[BytesValue],
        inputs: Option<&[multiversx_sc::abi::InputAbi]>,
    ) -> Vec<String> {
        let mut result = Vec::with_capacity(args.len());
        let mut arg_idx = 0;

        let input_count = inputs.map_or(0, |ins| ins.len());

        for input_idx in 0..input_count {
            if arg_idx >= args.len() {
                break;
            }

            let input = &inputs.unwrap()[input_idx];
            let abi_type = &input.type_names.abi;

            if let Some(inner) = abi_type
                .strip_prefix("variadic<")
                .and_then(|s| s.strip_suffix('>'))
            {
                // Variadic: consume all remaining args
                let remaining = &args[arg_idx..];
                let formatted = self.format_variadic(inner, remaining);
                result.push(formatted);
                arg_idx = args.len(); // all consumed
            } else if let Some(inner) = abi_type
                .strip_prefix("optional<")
                .and_then(|s| s.strip_suffix('>'))
            {
                // Optional: consume one arg if available
                let type_names = multiversx_sc::abi::TypeNames {
                    abi: inner.to_string(),
                    rust: String::new(),
                };
                let formatted = self.format_arg_value(&type_names, &args[arg_idx]);
                result.push(formatted);
                arg_idx += 1;
            } else {
                // Scalar type
                let formatted = self.format_arg_value(&input.type_names, &args[arg_idx]);
                result.push(formatted);
                arg_idx += 1;
            }
        }

        // Any remaining args without ABI info
        while arg_idx < args.len() {
            result.push(Self::format_value(&args[arg_idx].original));
            arg_idx += 1;
        }

        result
    }

    /// Formats a variadic argument: `MultiValueVec::from(vec![...])`.
    ///
    /// If `inner_type` is `multi<A,B,...>`, groups remaining args and wraps each
    /// group in `MultiValueN::new(...)`.
    fn format_variadic(&mut self, inner_type: &str, args: &[BytesValue]) -> String {
        if let Some(multi_inner) = inner_type
            .strip_prefix("multi<")
            .and_then(|s| s.strip_suffix('>'))
        {
            let field_types = Self::parse_multi_fields(multi_inner);
            let group_size = field_types.len();

            if group_size == 0 || args.is_empty() {
                return "MultiValueVec::from(vec![])".to_string();
            }

            let multi_struct = format!("MultiValue{}", group_size);
            let mut items = Vec::new();

            for chunk in args.chunks(group_size) {
                let mut fields = Vec::new();
                for (j, arg) in chunk.iter().enumerate() {
                    let type_names = multiversx_sc::abi::TypeNames {
                        abi: field_types.get(j).cloned().unwrap_or_default(),
                        rust: String::new(),
                    };
                    fields.push(self.format_arg_value(&type_names, arg));
                }
                items.push(format!("{}::new({})", multi_struct, fields.join(", ")));
            }

            format!("MultiValueVec::from(vec![{}])", items.join(", "))
        } else {
            // Simple variadic (not multi)
            let type_names = multiversx_sc::abi::TypeNames {
                abi: inner_type.to_string(),
                rust: String::new(),
            };

            if args.is_empty() {
                return "MultiValueVec::from(vec![])".to_string();
            }

            let items: Vec<String> = args
                .iter()
                .map(|arg| self.format_arg_value(&type_names, arg))
                .collect();

            format!("MultiValueVec::from(vec![{}])", items.join(", "))
        }
    }

    /// Parses the comma-separated fields inside `multi<A,B,...>`, respecting nested angle brackets.
    fn parse_multi_fields(s: &str) -> Vec<String> {
        let mut fields = Vec::new();
        let mut depth = 0;
        let mut current = String::new();

        for ch in s.chars() {
            match ch {
                '<' => {
                    depth += 1;
                    current.push(ch);
                }
                '>' => {
                    depth -= 1;
                    current.push(ch);
                }
                ',' if depth == 0 => {
                    fields.push(current.trim().to_string());
                    current = String::new();
                }
                _ => {
                    current.push(ch);
                }
            }
        }

        let trimmed = current.trim().to_string();
        if !trimmed.is_empty() {
            fields.push(trimmed);
        }

        fields
    }

    /// Maps an endpoint name from the scenario (usually camelCase) to the Rust method name (snake_case)
    /// by looking it up in the contract ABI.
    fn map_endpoint_name(&self, scenario_endpoint_name: &str) -> String {
        // Look up the endpoint in the ABI
        for endpoint in &self.abi.endpoints {
            if endpoint.name == scenario_endpoint_name {
                return endpoint.rust_method_name.clone();
            }
        }

        // If not found, return the original name (might be a special case or already in the correct format)
        scenario_endpoint_name.to_string()
    }

    fn generate_proxy_type(&self) -> String {
        // Convert crate name to CamelCase for the proxy struct name
        let struct_name = self
            .crate_name
            .split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<String>();

        format!("{}_proxy::{}Proxy", self.crate_name, struct_name)
    }

    fn format_value_as_string(value: &ValueSubTree) -> String {
        match value {
            ValueSubTree::Str(s) => s.clone(),
            ValueSubTree::List(items) => {
                let strs: Vec<String> = items.iter().map(Self::format_value_as_string).collect();
                strs.join("|")
            }
            ValueSubTree::Map(map) => {
                let strs: Vec<String> = map.values().map(Self::format_value_as_string).collect();
                strs.join("|")
            }
        }
    }

    fn format_nonce_value(value: &ValueSubTree) -> String {
        let num_str = match value {
            ValueSubTree::Str(s) => s.as_str(),
            _ => return format!("\"{}\"", Self::format_value_as_string(value)),
        };

        // Remove commas and underscores for parsing
        let cleaned = num_str.replace([',', '_'], "");

        // Nonces are always u64
        if cleaned.parse::<u64>().is_ok() {
            format!("{}u64", cleaned)
        } else {
            format!("\"{}\"", num_str)
        }
    }

    fn format_balance_value(value: &ValueSubTree) -> String {
        let num_str = match value {
            ValueSubTree::Str(s) => s.as_str(),
            _ => return format!("\"{}\"", Self::format_value_as_string(value)),
        };

        // Remove commas and underscores for parsing
        let cleaned = num_str.replace([',', '_'], "");

        // Try to parse as u128 and choose appropriate type
        if let Ok(num_u128) = cleaned.parse::<u128>() {
            if num_u128 <= u64::MAX as u128 {
                format!("{}u64", cleaned)
            } else {
                format!("{}u128", cleaned)
            }
        } else {
            // Fallback to string if not a valid number
            format!("\"{}\"", num_str)
        }
    }

    fn is_default_value(value: &ValueSubTree) -> bool {
        let val_str = format!("{:?}", value);
        val_str == "\"0\"" || val_str == "\"\"" || val_str.is_empty()
    }

    /// Converts a test address name (like "owner") to a constant name (like "OWNER_ADDRESS")
    fn test_address_to_const_name(name: &str) -> String {
        format!(
            "{}_ADDRESS",
            name.to_uppercase().replace(['-', '.', ' '], "_")
        )
    }
}
