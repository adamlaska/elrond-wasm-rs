use std::collections::HashMap;

/// Grouping for generated constants. Variants are ordered by desired output order.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConstGroup {
    CodePath,
    Address,
    TokenId,
    Hash,
    ByteArray,
}

/// Data about a generated constant: its group, type, and initialization expression.
pub struct ConstData {
    pub const_group: ConstGroup,
    pub const_type: String,
    pub initialization: String,
}

/// Holds all constant-related state: lookup maps, counters, and the constant registry.
#[derive(Default)]
pub struct ConstState {
    /// Maps address value to constant name (for TestAddress/TestSCAddress)
    pub test_address_map: HashMap<String, String>,
    /// Maps hex address to constant name
    pub hex_address_map: HashMap<String, String>,
    /// Counter for hex address constants
    pub hex_address_counter: usize,
    /// Maps code path expression to constant name
    pub code_path_map: HashMap<String, String>,
    /// Maps token identifier to constant name
    pub token_id_map: HashMap<String, String>,
    /// Maps H256 hex value to constant name
    pub h256_map: HashMap<String, String>,
    /// Counter for H256 constants
    pub h256_counter: usize,
    /// Maps byte array hex value to constant name (for arrayN<u8> types)
    pub hex_array_map: HashMap<String, String>,
    /// Counter for byte array constants, per size
    pub hex_array_counter: HashMap<usize, usize>,
    /// Map from constant name to its data (type and initialization)
    pub const_map: HashMap<String, ConstData>,
}

impl ConstState {
    /// Registers a new constant declaration.
    pub fn add_const(
        &mut self,
        name: String,
        const_group: ConstGroup,
        const_type: String,
        initialization: String,
    ) {
        self.const_map.insert(
            name,
            ConstData {
                const_group,
                const_type,
                initialization,
            },
        );
    }

    /// Renders all registered constants, sorted by group, then by type name, then by const name.
    pub fn render_constants(&self) -> String {
        let mut entries: Vec<_> = self.const_map.iter().collect();
        entries.sort_by(|a, b| {
            a.1.const_group
                .cmp(&b.1.const_group)
                .then(a.1.const_type.cmp(&b.1.const_type))
                .then(a.0.cmp(b.0))
        });

        let mut buf = String::new();
        for (name, data) in entries {
            buf.push_str(&format!(
                "const {}: {} = {};\n",
                name, data.const_type, data.initialization
            ));
        }
        buf
    }

    /// Derives a constant name from a code path expression
    /// Example: "mxsc:../output/adder.mxsc.json" -> "ADDER_CODE_PATH"
    fn derive_code_path_const_name(code_path_expr: &str) -> String {
        // Extract the filename from the path
        let path_str = code_path_expr
            .strip_prefix("mxsc:")
            .unwrap_or(code_path_expr);
        let filename = path_str.rsplit('/').next().unwrap_or(path_str);

        // Remove .mxsc.json extension
        let contract_name = filename
            .strip_suffix(".mxsc.json")
            .unwrap_or(filename)
            .replace('-', "_");

        format!("{}_CODE_PATH", contract_name.to_uppercase())
    }

    /// Formats a code path expression, generating a constant if needed
    pub fn format_code_path(&mut self, code_path_expr: &str) -> String {
        // Check if we already have a constant for this path
        if let Some(const_name) = self.code_path_map.get(code_path_expr) {
            return const_name.clone();
        }

        // Generate a new constant
        let const_name = Self::derive_code_path_const_name(code_path_expr);

        // Extract the actual path (strip mxsc: prefix)
        let path_value = code_path_expr
            .strip_prefix("mxsc:")
            .unwrap_or(code_path_expr);
        // Remove leading ../ if present to make it relative to contract root
        let path_value = path_value.strip_prefix("../").unwrap_or(path_value);

        // Register the constant declaration
        self.add_const(
            const_name.clone(),
            ConstGroup::CodePath,
            "MxscPath".to_string(),
            format!("MxscPath::new(\"{}\")", path_value),
        );

        // Store in map for future use
        self.code_path_map
            .insert(code_path_expr.to_string(), const_name.clone());

        const_name
    }
}
