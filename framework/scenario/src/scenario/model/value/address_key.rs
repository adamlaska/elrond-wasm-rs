use multiversx_sc::{
    chain_core::std::Bech32Address,
    types::{Address, TestAddress, TestSCAddress},
};

use super::{value_from_slice, AddressValue};
use crate::scenario_format::{
    interpret_trait::{InterpretableFrom, InterpreterContext, IntoRaw},
    value_interpreter::interpret_string,
};
use std::{cmp::Ordering, fmt};

#[derive(Debug, Clone, Eq)]
pub struct AddressKey {
    pub value: Address,
    pub original: String,
}

impl Default for AddressKey {
    fn default() -> Self {
        Self {
            value: Address::zero(),
            original: Default::default(),
        }
    }
}

impl AddressKey {
    pub fn to_address(&self) -> Address {
        self.value.clone()
    }
}

impl Ord for AddressKey {
    fn cmp(&self, other: &Self) -> Ordering {
        self.original.cmp(&other.original)
    }
}

impl PartialOrd for AddressKey {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for AddressKey {
    fn eq(&self, other: &Self) -> bool {
        self.original == other.original
    }
}

impl fmt::Display for AddressKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.original.fmt(f)
    }
}

impl InterpretableFrom<&str> for AddressKey {
    fn interpret_from(from: &str, context: &InterpreterContext) -> Self {
        let bytes = interpret_string(from, context);
        AddressKey {
            value: value_from_slice(bytes.as_slice()),
            original: from.to_string(),
        }
    }
}

impl InterpretableFrom<String> for AddressKey {
    fn interpret_from(from: String, context: &InterpreterContext) -> Self {
        AddressKey::interpret_from(from.as_str(), context)
    }
}

impl IntoRaw<String> for AddressKey {
    fn into_raw(self) -> String {
        self.original
    }
}

impl From<&str> for AddressKey {
    fn from(from: &str) -> Self {
        Self::interpret_from(from, &InterpreterContext::default())
    }
}

impl From<String> for AddressKey {
    fn from(from: String) -> Self {
        Self::interpret_from(from, &InterpreterContext::default())
    }
}

impl From<&AddressValue> for AddressKey {
    fn from(from: &AddressValue) -> Self {
        AddressKey {
            value: from.to_address(),
            original: from.original.to_concatenated_string(),
        }
    }
}

impl From<AddressValue> for AddressKey {
    fn from(from: AddressValue) -> Self {
        AddressKey::from(&from)
    }
}

impl From<&Address> for AddressKey {
    fn from(from: &Address) -> Self {
        AddressKey {
            value: from.clone(),
            original: format!("0x{}", hex::encode(from)),
        }
    }
}

impl From<&Bech32Address> for AddressKey {
    fn from(from: &Bech32Address) -> Self {
        AddressKey {
            value: from.to_address().clone(),
            original: from.to_bech32_expr(),
        }
    }
}

impl From<Bech32Address> for AddressKey {
    fn from(from: Bech32Address) -> Self {
        AddressKey {
            original: from.to_bech32_expr(),
            value: from.into_address(),
        }
    }
}

impl From<TestAddress<'_>> for AddressKey {
    fn from(from: TestAddress) -> Self {
        AddressKey {
            value: from.eval_to_array().into(),
            original: from.eval_to_expr(),
        }
    }
}

impl From<TestSCAddress<'_>> for AddressKey {
    fn from(from: TestSCAddress) -> Self {
        AddressKey {
            value: from.eval_to_array().into(),
            original: from.eval_to_expr(),
        }
    }
}
