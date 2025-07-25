use multiversx_chain_vm::schedule::gas_schedule_toml_by_version;

use convert_case::{Case, Casing};

use crate::{get_file_path, parse_toml_sections};

pub fn generate_file_content(toml_version: u16) {
    let content = gas_schedule_toml_by_version(toml_version);
    let rust_code = generate_structs(content);
    let output_file = get_file_path();

    std::fs::write(&output_file, rust_code).unwrap();
    println!("Generated Rust structs written to {:#?}", output_file);
}

fn generate_structs(toml_content: &str) -> String {
    let mut output = String::new();

    // auto generated tag
    output.push_str(
        "// Code generated by gas schedule generator. DO NOT EDIT.

// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// !!!!!!!!!!!!!!!!!!!!!! AUTO-GENERATED FILE !!!!!!!!!!!!!!!!!!!!!!
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
\n",
    );

    // add header
    output.push_str("use serde::{Deserialize, Serialize};\n\n");

    let sections = parse_toml_sections(toml_content);

    for (section_name, section_entries) in &sections {
        output.push_str(&generate_section_struct(section_name, section_entries));
        output.push('\n');
    }

    output
}

fn generate_section_struct(section_name: &str, entries: &Vec<(String, String)>) -> String {
    let mut output = String::new();

    output.push_str("#[derive(Serialize, Deserialize, Clone, Debug, Default, PartialEq)]\n");
    output.push_str("#[serde(default)]\n");
    output.push_str(&format!("pub struct {} {{\n", section_name));

    for (entry_key, _) in entries {
        let field_name = entry_key.to_case(Case::Snake);

        output.push_str(&format!("    #[serde(rename = \"{}\")]\n", entry_key));
        output.push_str(&format!("    pub {}: u64,\n", field_name));
    }

    output.push_str("}\n");

    output
}
