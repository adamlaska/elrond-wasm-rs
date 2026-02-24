use multiversx_sc::abi::TypeNames;
use multiversx_sc_scenario::scenario::model::BytesValue;

use super::{num_format, test_gen::TestGenerator};

impl<'a> TestGenerator<'a> {
    /// Formats an argument value based on ABI type info and raw bytes.
    ///
    /// Uses ABI type information to generate idiomatic Rust literals where possible.
    /// For types whose ABI name is ambiguous (e.g. time types all map to "u64"),
    /// the Rust type name is checked instead.
    /// Falls back to `ScenarioValueRaw::new` for unrecognized types.
    pub(super) fn format_arg_value(&mut self, type_names: &TypeNames, arg: &BytesValue) -> String {
        match type_names.specific_or_abi() {
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

    /// Formats a BigUint value for use as a payment amount.
    pub(super) fn format_biguint_value(
        value: &multiversx_sc_scenario::num_bigint::BigUint,
    ) -> String {
        let bytes = value.to_bytes_be();
        num_format::format_unsigned(&bytes, "BigUint")
    }

    /// Formats a 32-byte H256 value as a named constant.
    /// Generates a `const H256_N: H256 = H256::from_hex("...");` declaration.
    pub(super) fn format_h256(&mut self, arg: &BytesValue) -> String {
        let hex_str = hex::encode(&arg.value);
        self.consts.get_or_create_h256(&hex_str)
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
    pub(super) fn format_byte_array(&mut self, arg: &BytesValue, size: usize) -> String {
        let hex_str = hex::encode(&arg.value);
        self.consts.get_or_create_byte_array(&hex_str, size)
    }

    /// Formats a list of arguments, using ABI type info when available.
    ///
    /// Handles `variadic<T>` and `multi<A,B,...>` types:
    /// - A `variadic<T>` input consumes all remaining scenario arguments, wrapping them
    ///   in `MultiValueVec::from(vec![...])`.
    /// - If the inner type is `multi<A,B,...>`, arguments are taken in groups matching
    ///   the number of multi fields, each group wrapped in `MultiValueN::new(...)`.
    /// - For all other (scalar) types, delegates to `format_arg_value`.
    pub(super) fn format_args(
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
                let type_names = TypeNames::from_abi(inner.to_string());
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
                    let type_names =
                        TypeNames::from_abi(field_types.get(j).cloned().unwrap_or_default());
                    fields.push(self.format_arg_value(&type_names, arg));
                }
                items.push(format!("{}::new({})", multi_struct, fields.join(", ")));
            }

            format!("MultiValueVec::from(vec![{}])", items.join(", "))
        } else {
            // Simple variadic (not multi)
            let type_names = TypeNames::from_abi(inner_type.to_string());

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
}
