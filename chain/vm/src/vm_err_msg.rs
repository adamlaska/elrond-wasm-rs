pub const FUNCTION_NOT_FOUND: &str = "invalid function (not found)";
pub const NOT_ENOUGH_GAS: &str = "not enough gas";

pub const NON_PAYABLE_FUNC_EGLD: &str = "function does not accept EGLD payment";
pub const NON_PAYABLE_FUNC_ESDT: &str = "function does not accept ESDT payment";

pub const ARG_OUT_OF_RANGE: &str = "argument out of range";
pub const BIG_INT_BITWISE_OPERATION_NEGATIVE: &str =
    "bitwise operations only allowed on positive integers";
pub const DIVISION_BY_0: &str = "division by 0";
pub const BAD_BOUNDS_LOWER: &str = "bad bounds (lower)";
pub const EXPONENT_IS_POSITIVE: &str = "exponent must be negative";
pub const NUMBER_IS_NOT_NORMAL: &str =
    "number is not normal. It is either infinite, NaN or subnormal";
pub const CANNOT_COMPARE_VALUES: &str = "values are not comparable";
pub const WRITE_RESERVED: &str = "cannot write to storage under reserved key";
pub const WRITE_READONLY: &str = "cannot write on read only mode";

pub const ERROR_SIGNALLED_BY_SMARTCONTRACT: &str = "error signalled by smartcontract";

pub const ERROR_NO_CALLBACK_CLOSURE: &str =
    "no callback for closure, cannot call callback directly";

pub const ERROR_BYTES_EXCEED_INT64: &str = "bytes cannot be parsed as int64";
pub const ERROR_BYTES_EXCEED_UINT64: &str = "bytes cannot be parsed as uint64";

pub const PROMISES_TOKENIZE_FAILED: &str = "tokenize failed";

pub const CRYPTO_INVALID_SIGNATURE: &str = "invalid signature";
pub const CRYPTO_ED25519_ERROR: &str = "ed25519 verify error";
