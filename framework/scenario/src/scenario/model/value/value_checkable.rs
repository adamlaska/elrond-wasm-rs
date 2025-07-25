use multiversx_chain_vm::host::context::TxFunctionName;

use super::{value_set_big_uint::*, BytesValue, CheckValue, U64Value};
use num_bigint::BigUint;

pub trait Checkable<V> {
    fn check(&self, value: V) -> bool;
}

impl Checkable<&[u8]> for BytesValue {
    fn check(&self, value: &[u8]) -> bool {
        self.value.as_slice() == value
    }
}

impl Checkable<&str> for BytesValue {
    fn check(&self, value: &str) -> bool {
        self.check(value.as_bytes())
    }
}

impl Checkable<&Vec<u8>> for BytesValue {
    fn check(&self, value: &Vec<u8>) -> bool {
        &self.value == value
    }
}

impl Checkable<&TxFunctionName> for BytesValue {
    fn check(&self, value: &TxFunctionName) -> bool {
        self.value.as_slice() == value.as_str().as_bytes()
    }
}

impl Checkable<&BigUint> for BigUintValue {
    fn check(&self, value: &BigUint) -> bool {
        &self.value == value
    }
}

impl Checkable<u64> for U64Value {
    fn check(&self, value: u64) -> bool {
        self.value == value
    }
}

impl<V, T> Checkable<V> for CheckValue<T>
where
    T: Checkable<V> + Default,
{
    fn check(&self, value: V) -> bool {
        match self {
            CheckValue::Star => true,
            CheckValue::Equal(eq) => eq.check(value),
        }
    }
}

impl Checkable<&[Vec<u8>]> for Vec<CheckValue<BytesValue>> {
    fn check(&self, values: &[Vec<u8>]) -> bool {
        if self.len() != values.len() {
            return false;
        }
        for (i, cv) in self.iter().enumerate() {
            if !cv.check(values[i].as_slice()) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        scenario::model::{BytesValue, CheckValue, Checkable, U64Value},
        scenario_format::serde_raw::ValueSubTree,
    };

    #[test]
    fn check_bytes() {
        let bv = BytesValue {
            value: b"abc".to_vec(),
            original: ValueSubTree::Str("abc".to_string()),
        };
        assert!(bv.check(&b"abc"[..]));

        let cb_eq = CheckValue::Equal(bv);
        assert!(cb_eq.check(&b"abc"[..]));
        assert!(!cb_eq.check(&b"abd"[..]));

        let cb_star: CheckValue<BytesValue> = CheckValue::Star;
        assert!(cb_star.check(&b"anything_really"[..]));
    }

    #[test]
    fn check_u64() {
        let u64v = U64Value {
            value: 123,
            original: ValueSubTree::Str("123".to_string()),
        };
        assert!(u64v.check(123u64));

        let cb_eq = CheckValue::Equal(u64v);
        assert!(cb_eq.check(123u64));

        let cb_star: CheckValue<U64Value> = CheckValue::Star;
        assert!(cb_star.check(1234567890));
    }
}
