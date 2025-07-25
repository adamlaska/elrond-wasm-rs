mod blockchain_api;
mod call_value_api;
mod composite_api;
mod crypto_api;
mod endpoint_arg_api;
mod endpoint_finish_api;
mod error_api;
mod external_view;
mod log_api;
pub(crate) mod managed_types;
mod print_api;
mod send_api;
mod storage_api;
pub mod uncallable;
mod vm_api;

pub use blockchain_api::*;
pub use call_value_api::*;
pub use composite_api::*;
pub use crypto_api::*;
pub use endpoint_arg_api::*;
pub use endpoint_finish_api::*;
pub use error_api::*;
pub use external_view::*;
pub use log_api::*;
pub use managed_types::*;
pub use print_api::*;
pub use send_api::*;
pub use storage_api::*;
pub use vm_api::VMApi;

// Backwards compatibility.
pub use crate::chain_core::builtin_func_names::*;

/// Utility function.
pub(crate) fn quick_signal_error<Api: ErrorApi>(message: &str) -> ! {
    Api::error_api_impl().signal_error(message.as_bytes());
}
