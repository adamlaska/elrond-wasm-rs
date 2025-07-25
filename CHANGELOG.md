# Change Log

This file contains a centralizes a trace of all published crate versions, with their changes in short.

## Versioning the crates

The `mx-sdk-rs` repo contains many crates, grouped into several families. Crates in these families always have the same version with one another.

For brevity, the changelog will only mention a short version of their name.

They are:
- `multiversx-sc`, in short `sc`, the smart contract framework, 7 crates + 3 for contracts/modules:
	- `multiversx-sc`
    - `multiversx-sc-derive`
    - `multiversx-sc-meta`
    - `multiversx-sc-meta-lib`
    - `multiversx-sc-scenario`
    - `multiversx-sc-snippets`
    - `multiversx-sc-wasm-adapter`
    - `multiversx-sc-modules` - *standard contract modules*
	- `multiversx-price-aggregator-sc` - *core contract*
	- `multiversx-wegld-swap-sc` - *core contract*
- `multiversx-sc-codec`, in short `codec`, the serializer/deserializer, 2 crates:
	- `multiversx-sc-codec`
	- `multiversx-sc-codec-derive`
- Chain crates, in short `chain`. Formerly it was only the VM (`vm`). 2 crates:
	- `multiversx-chain-core` - *a common crate for chain types, constants, flags*
	- `multiversx-chain-vm` - *a Rust VM implementation*
- `multiversx-chain-scenario-format`, in short `scenario-format`, scenario JSON serializer/deserializer, 1 crate.
- `multiversx-sdk`, in short `sdk`, allows communication with the chain(s), 3 crates:
	- `multiversx-sdk`
	- `multiversx-sdk-http`
	- `multiversx-sdk-dapp`


## [sc 0.59.1, codec 0.23.1, chain 0.16.1, sdk 0.11.1] - 2025-07-24
- Governance proxy improvements.
- Codec: added support for using u128.
- SDK/interactors:
	- Added logging for http requests and responses;
	- Fixed an issue with retrieving results from transactions with multi-transfer ESDT.
	- Fixed a VM query error handling issue.


## [sc 0.59.0, codec 0.23.0, chain 0.16.0, sdk 0.11.0] - 2025-07-03
- Support for Barnard features
	- `barnard` feature for smart contracts, can be enabled in the contract's `Cargo.toml` or `sc-config.toml`;
	- Blockchain API new features:
		- Code hash API;
		- Block info:
			- Timstamps in milliseconds: `get_block_timestamp_ms`,  `get_prev_block_timestamp_ms`, `epoch_start_block_timestamp_ms`;
			- Block round time: `get_block_round_time_ms`;
			- Epoch start info: `epoch_start_block_timestamp_ms`, `epoch_start_block_nonce`, `epoch_start_block_round`.
		- ESDT info:
			- Token type API supplied by the protocol (`get_esdt_token_type`);
			- `get_esdt_token_data` provides the token type supplied by the protocol;
			- `EsdtTokenType` updated with new ESDT types (meta & dynamic tokens).
	- New transaction mechanisms:
		- Fallible synchronous call;
		- Fallible transfer-execute;
		- Both are integrated in the unified syntax;
		- Simplified several scenarios by routing all through the fallible tx VM hooks.
	- Optimisations:
		- Multi-transfer call value including the direct EGLD is now provided by the VM directly.
		- Direct conversion between ManagedBuffer and i64 (small integer) now provided directly by the VM.
- Back transfers now support multi-transfers with EGLD properly
	- New `BackTransfer` structure contains back-transfers as a multi-transfer list;
	- It contains methods to filter and extract EGLD or single ESDT values;
	- New implementation of `ReturnsBackTransfers` and `ReturnsBackTransfersReset`, which work with this payment list;
	- `ReturnsBackTransfersEGLD` now supports multi-transfer;
	- Old implementations renamed to `*Legacy`.
- New proxies for system smart contracts:
	- Governance system SC;
	- Delegation system SC.
- Core crate updates:
	- Bech32Address:
		- Deduplicated and moved to the core crate, guarded by a `std` feature;
		- Support for custom HRP;
	- `BLSKey` and `BLSSignature` types, to help the interaction with the delegation contract.
- `sc-meta`:
	- Support for building contracts with `std` library;
	-  `test-gen` support for `#[should_panic]` annotation.
- Validator processing in the SDK, including parsing from pem.
- Event log name can now be empty or missing in declaration, the method name will be used in this case.
- Fixed a bug in mandos-rs, it was not handling a failing `scQuery` properly.
- Codec: improved multi-value length handling.


## [sc 0.58.0, codec 0.22.1, chain 0.15.0, sdk 0.10.0] - 2025-05-26
- Rust VM and debugger redesign:
	- VM major refactoring: runtime, execution, debugger, VM hooks handler;
	- Integration of the new executor interface: new instance, executor & VM hooks interfaces;
	- Early exit mechanism for VM hooks;
	- Integration of Wasmer 2.2 production code, via an adapter;
	- Integration of Wasmer 6, as an experimental alternative, but more stable in tests;
	- Mechanism for running blackbox and mandos-rs tests with compiled contracts (.wasm);
	- Mechanism for running the same test via the debugger as part of the Rust test suite, and via Wasmer as part of the Wasm tests;
	- Crude metering, as a proof-of-concept, will be refined in the future. Gas schedule can be configured.
	- New feature `compiled-sc-tests` to replace `run-go-tests`.
- Build system:
	- Opcode validator, as a post-build automated process. It detects and signals the usage of non-whitelisted WASM opcodes.
	- WASM target:
		- Default target is now `wasmv1-none` instead of `wasm32-unknown-unknown`. This is to allow upgrading to Rust 1.87, which uses LLVM 20 and normally emits bulk memory opcodes, which are currently unsupported on MultiversX. This change prevents these opcodes to be emitted.
		- A mechanism for overriding the default target, per contract, in `sc-config.toml`.
		- Target will be autoinstalled upon build, if missing.
- `sc-meta` new `test` argument: `-w` or `--wasm`, to run tests based on compiled smart contracts; replaces `--go`.
- Improved interactor error handling.
- Back-transfer object cloneable.
- Fixed typos.
- Updated dependencies.

## [sc 0.57.1, sdk 0.9.1] - 2025-04-04
- Retrieve token properties using `get_token_properties`;
- Fixed URIs for `esdt_metadata_recreate` and `esdt_metadata_update`;
- `sc-meta`:
  - Fixed `test --chain-simulator` used for running chain-simulator interactor tests;
  - Added extra checks for argument validity.
- Interactor:
  - Fixed `setStateOverwrite`;
  - Fixed `ReturnsTxHash` result handler.
- Enhanced `checkState` to allow partial key verification.

## [sc 0.57.0, codec 0.22.0, chain 0.14.0, sdk 0.9.0, scenario-format 0.23.1] - 2025-03-11
- Newer compiler support:
	- Dropped support for Rust compiler versions older than 1.83.
	- Support and optimizations for using Rust 1.85.
- `sc-meta`:
	- Windows support.
	- Removed the concept of a "main" contract configuration.
- Using `typenum`/`generic-array` instead of const generics/macros for:
	- ManagedVec payloads;
	- ManagedDecimal const decimals.
- ManagedDecimal - more arithmetic operator implementations for combinations of const + var decimals.
- ManagedVecItem can now be derived for enums with fields.
- Codec and ABI support for bitflags.
- Storage mappers:
	- New storage mapper: `TimelockMapper`;
	- Renamed source type and object.
- `ESDTTransferRole`:
	- Reintroduced role after being accidentally dropped;
	- Added a `token_has_transfer_role` method for checking if it is set on a token, as a workaround until Barnard release.
- Unified syntax - result handler for back transfers, which resets previous back transfers (`ReturnsBackTransfersReset`).
- SDK:
	- Chain simulator - set state overwrite support;
	- `Wallet` `get_shard` method.
- Debugger - improved mandos error messages.
- Dependencies upgraded.


## [sc 0.56.1, chain 0.13.1, sdk 0.8.2] - 2025-02-06
- Allow setting gas for callback for direct transfers.
- NestedEncode for interaction types: TestAddress, TestScAddress and TestTokenIdentifier.
- Bugfix: pretty representation for ManagedAddress when debugging.
- Upgrade dependency: ruplacer.

## [sc 0.56.0, chain 0.13.0, sdk 0.8.1] - 2025-01-23
- Rust VM support for readonly sync calls.
- `ManagedMapEncoded`, a map type that can use any key or value types that are serializable.
- `ManagedDecimal` implements `ManagedVecItem`.
- Bugfixes, improvements:
	- Fixed a bug regarding the ESDT roles VM hook;
	- Pretty representation for ManagedBuffer and other string-like types when debugging;
	- API fix of an issue that was preventing set state in chain simulator;
	- Snippets generator fixes involving the crate path and the upgrade result handler.

## [sc 0.55.0, codec 0.21.2, chain 0.12.0, sdk 0.8.0] - 2025-01-08
- Integrating Spica changes into the framework:
	- EGLD+ESDT multi-transfers are now possible:
		- changed the handling of call values: EGLD is treated almost the same as an ESDT in `all_transfers` and `multi_egld_or_esdt`, old ESDT methods are given some protection against unexpected scenarios
		- changed the tx unified syntax for sending EGLD+ESDT from contracts, interactors and tests;
		- support in the Rust VM.
	- New built-in functions in the `ESDTSystemSCProxy`: `ESDTModifyRoyalties`, `SDTSetNewURIs`, `ESDTModifyCreator`, `ESDTMetaDataRecreate`, `ESDTMetaDataUpdate`.
- Interactor support for "set state" on the chain simulator.
- Fixed ownership for ManagedVec iterators, specifically reference iterators only produce references to the items.
- Syntax cleanup:
	- `#[payable]` now allowed instead of `#[payable("*")]`;
	- `register_promise` allows callback, without calling a function on destination.
- Refactoring and optimizations:
	- Simplified the callback selector;
	- Performance improvements in ManagedVec iterators;
	- Removed some unnecessary bound checks in `multi_esdt`.

## [sc 0.54.6] - 2024-12-04
- `ManagedDecimal` bugfixes:
	- ABI/proxy bugfix;
	- Rescale bugfix.

## [sc 0.54.5] - 2024-11-28
- `ManagedVec` - deprecated `sort` and guarded it by the `alloc` feature, since it uses the allocator.
- `sc-meta`
	- versioning fix;
	- interactor generator fix.
- Interactors - fixed code metadata on deploy.

## [sc 0.54.4] - 2024-11-22
- `sc-meta`
	- `install debugger` CLI that prepares VSCode extension for debugging;
	- fixed a crash involving templates and installers.
- Deprecated `#[derive(TypeAbi)]` and added an additional warning in the macro.

## [sc 0.54.3] - 2024-11-18
- `#[storage_mapper_from_address]` fixes for: `FungibleTokenMapper`, `NonFungibleTokenMapper`, `TokenAttributesMapper`, `UniqueIdMapper`, `UserMapper`, `AddressToIdMapper`.

## [sc 0.54.2, codec 0.21.1, chain 0.11.1, sdk 0.7.1] - 2024-11-15
- Codec improvements:
	- `MultiValueX` - `TopDecodeMultiLength` implementation fix;
	- `ManagedVecItem` implemented for MultiValue2 and MultiValue3.
- `sc-meta snippets` improvements.

## [sc 0.54.1] - 2024-11-13
- `sc-meta` `cs` - ChainSimulator CLI, which provides handy functionality to:
	- install the chain simulator image in Docker;
	- start/stop the chain simulator;
	- quick testing using the `chain-simulator-tests` feature flag.
- Adder interactor cleanup, including in template.
- Interactor - `use_chain_simulator` builder method, for improved backwards compatibility.
- `MultiValueEncodedCounted` - a lazy multi-value encoding, but with known number of elements.

## [sc 0.54.0, sdk 0.7.0, chain 0.11.0] - 2024-11-06
- New crate, `multiversx-chain-core`, to be used in both framework and Rust VM. It contains common types, flags, and constants that refer to the protocol.
- Major SDK/interactor refactor:
	- Added support for Chain Simulator in interactors:
		- Added chain-simulator-specific endpoints: feed account, advance blocks
		- Added a system to set up accounts in the chain simulator from the interactor;
		- Support for advancing blocks in the interactor;
	- Split SDK crate into:
		- `multiversx-sdk` - only contains the specifications of the gateway API, without a mechanism to call the API;
		- `multiversx-sdk-http` - functionality to call the gateway via reqwest;
		- `multiversx-sdk-dapp` - functionality to call the gateway via wasm-bindgen, to be used in WebAssembly front-ends;
	- Major improvements in the retrieving of transactions and other blockchain data from the API, many bugs fixed;
	- Support for writing integration tests for interactors, using the Chain Simulator;
		- Also added support for test-related `chain-simulator-tests` feature flag in `sc-meta`;
	- Interactors on the front-end:
		- Interactor type made generic over the gateway API implementation, so that it can be used in both front-end and back-end, with no change in the code base;
		- Support for custom random number generation for the front-end;
	- Mechanism for fixing file paths in the interactor context;
	- Fixed an issue with the account tool;
	- Adjusted `sc-meta snippets` for the new syntax and the chain simulator support;
- Unified syntax:
	- `ReturnsHandledOrError` result handler, which can gracefully deal with failed transactions;
	- `ReturnsGasUsed` result handler;
	- `PassValue` result handler for providing a closure-like context for multi-transaction call/deploy;
	- More specific back transfer result handlers: `ReturnsBackTransfersEGLD`, `ReturnsBackTransfersMultiESDT`, `ReturnsBackTransfersSingleESDT`;
	- Fixed an issue with the update functionality not being general enough;
	- Deprecated `prepare_async()`, developers can now call `run()` directly, asynchronously;
- `sc-meta` improvements:
	- New mechanism for detecting and warning about storage writes in readonly endpoints, integrated into the build system;
	- Support for referencing the framework via git commit, branch, or tag, to make it easier to try out unreleased versions;
	- Support for `default-features`;
	- Better representation in console of the contract/lib folders, as well as better error messages.
	- Refactoring of the dependency handling logic.
- Fixed the debugger, following changes in the Rust debug tooling.
- `ManagedVec` `set` always consumes ownership. Preparations for a profound memory management cleanup.
- ABI:
	- `title` field and annotation;
	- Refactoring.

## [sc 0.53.2] - 2024-10-02
- `StakingModule` fix.

## [sc 0.53.1, sdk 0.6.1] - 2024-10-01
- Interactor: 
  - Allow signature to be empty in TransactionOnNetwork;
  - Allow return data to be empty in VMOutputApi.

## [sc 0.53.0 codec 0.21.0, vm 0.10.0, sdk 0.6.0, scenario-format 0.23.0] - 2024-09-04
- Unified syntax:
  -  Whitebox testing;
  -  Proxy fix for ManagedOption;
  -  TestTokenIdentifier syntactic sugar.
- New ResultHandler: `ReturnsLogs`.
- Interactor: 
  - Fix on API fetch process status;
  - Fix on ReturnsNewTokenIdentifier edge cases solved;
  - Fix on ESDTTransfer for transfer step;
  - Support for Keystore + password.
- Framework API support: EI 1.4 crypto functions.
- `sc-meta`:
  -  New `wallet` command: PEM and keystore generator and conversions;
  -  New `report` command: 
     -  Generate json or Markdown report based on size, path, allocator and panic messages;
     -  Compare reports;
     -  Convert reports.
- VecMapper update with index.
- Substitution list: AddressToIdMapper
- Dependencies updated.

## [sc 0.52.3] - 2024-08-06
- Pause module events.

## [sc 0.52.2] - 2024-08-01
- `ManagedBufferReadToEnd` extract data methods.

## [sc 0.52.1] - 2024-07-31
- `ManagedBufferReadToEnd` `TypeAbi` implementation.

## [sc 0.52.0, codec 0.20.1] - 2024-07-31
- ManagedBufferReadToEnd type, which flushed a nested data buffer.
- Fixed hex and binary formatters for byte slices.
- Added EI 1.4 and 1.5 configs.
- Dependency upgrades.

## [sc 0.51.1]
- `sc-meta upgrade` bugfix.

## [sc 0.51.0, codec 0.20.0, vm 0.9.0, sdk 0.5.0, scenario-format 0.22.3] - 2024-07-06
- Major refactoring of `multiversx-sc-meta`
	- Crate `multiversx-sc-meta` split in 2:
		1. `multiversx-sc-meta` remains the standalone tool. For backwards compatibility, it can still be used in contract meta crates, but a warning will be issued.
		2. `multiversx-sc-meta-lib` is the contract-only library to contract meta crates.
	- The refactoring came with few code changes, but dependencies were disentangled and cleaned up.
	- Account retrieval tool was merged into `sc-meta` standalone. Previously little known feature, it enables downloading the full state of an account and formatting it as a mandos set state step. Very useful for generating tests and investigating state.
	- `multiversx-sdk` was also refactored, especially the gateway proxy.
- A new code report is available in the `.mxsc.json` build output. The report analyzes the wasm code after build and always offers the following information:
	- `imports`: what VM hooks are used;
	- `eiCheck`: if the used imports comply with the environment interface (EI, allowed VM hooks);
	- `hasAllocator`: is it allocates on the heap;
	- `hasPanic`: whether it produces Rust panics and formats error messages using the standard Rust formatter (a source of code bloat).
- `ManagedDecimal` and `ManagedDecimalSigned`:
	- New types that encapulate a managed `BigUint` and `BigInt` respectively, but treat them as base 10 fixed point rational numbers.
	- Two flavors are allowed: the number of decimals is known at compile time (e.g. EGLD always has 18 decimals), or only at runtime.
		- Type `ConstDecimals` is able to resolve conversions at compile time, reducing code size and making encoding and decoding easier, since the number of decimals does not need to be encoded.
		- Regular `usize` number of decimals is resolved at runtime.
	- All basic arithmetic operations are implemented for these types, just like for the big integers.
- Implemented logarithms:
	- Natural logarithm `ln` for `ManagedDecimal`, `BigFloat`, and `BigInt`.
	- Base 2 logarithm `log2` for `ManagedDecimal`.
	- Precision is about 5 decimals, largely irrespective of input.
	- The operation is cheap, `ln` costs 44980 gas for managed decimals and 153772 for big floats, largely irrespective of input.
- Smart contract code on the front-end:
	- Framework and contract code, together with the Rust VM as a backend, can now be compiled to WebAssembly for front-end, using `wasm-bindgen`.
	- A few incompatible Rust VM features needed to be made optional for this to work.
- Reverted changes in `sc 0.50.6` (`diagnostic::on_unimplemented` & rustc 1.78 dependency).
- Bugfix: `sync_call_readonly` can now be used with proxies.


## [sc 0.50.6] - 2024-07-05
- Temporarily removed dependency to rustc 1.78, to ease transition from older versions. Will be re-enabled in 0.51.0.

## [sc 0.50.5] - 2024-06-21
- `#[storage_mapper_from_address] annotation.
- Added missing equality operator for test addresses (`TestAddress`, `TestSCAddress`).

## [sc 0.50.4] - 2024-06-06
- Compiler version requirement (1.78).
- Minor imports fix.

## [sc 0.50.3] - 2024-05-25
- Dependency update and fix. There was an issue with the `zip` dependency in sc-meta.

## [sc 0.50.2] - 2024-05-24
- Unified transaction syntax:
	- Better compilation error messages for malformed transactions;
	- Deprecated methods `async_call` and `async_call_promises`, which are kept for backwards compatibility, but causing confusion among developers;
	- Contract upgrade available in tests.
- `sc-meta` proxy compare option, which checks that proxies are up to date. Useful for CI.
- `TypeAbi` - removed `Unmanaged` associated type trait bounds, and implemented it for more types.
- Removed jitter from interactor transaction fetch.
- Fixed an issue in the snippets generator.

## [sc 0.50.1] - 2024-05-16
- `sc-meta all snippets` generates unified syntax.
- Proxy generator can reference multi-contract variant.
- Fixes:
	- `BoxedBytes` - fixed memory leak.
	- `ManagedVecItem` - allowing larger payloads (up to 128 bytes).

## [sc 0.50.0, codec 0.19.0, vm 0.8.4, sdk 0.4.1] - 2024-05-10
- Framework now runs on **stable** Rust. All unstable features were removed. The most important changes enabling this:
	- `CodecFrom` completely removed, `TypeAbiFrom` was used instead since 0.49.0.
	- `ManagedVecItem` payload redesigned.
	- Contract panic message mechanism improved.
- Unified syntax:
	- `NotPayable` marker type in proxies, which prevents callers to add payment to a non-payable endpoint.

## [sc 0.49.0, codec 0.18.8, sdk 0.4.0] - 2024-05-07
- Unified transaction syntax
	- new syntax for sending transactions from contracts
	- new syntax for integration tests: tx, set state, check state, etc.
	- new syntax for interactors
	- new proxies, generated from sc-meta
	- support for upgrade in new proxies
- Improved interactor tx result polling performance.

## [sc 0.48.1, codec 0.18.7] - 2024-04-30
- Simplified decoding of small numbers (i64/u64).
- Manual reset of the `StaticApi`, in order to free memory for long-running tasks.

## [sc 0.49.0-alpha.4, sdk 0.4.0-alpha.4] - 2024-04-23
Fourth pre-release, contains many interactor improvements, including improved tx polling.

## [sc 0.49.0-alpha.3] - 2024-04-13
Third pre-release of the unified syntax, includes backwards compatibility fixes and testing set state/check state.

## [sc 0.49.0-alpha.2] - 2024-04-09
Second pre-release of the unified syntax. Most features done, including fully featured interactors.
Still missing: set state/check state in tests.

## [sc 0.48.0] - 2024-04-09
- When serializing to a managed buffer, static buffer caching is disabled by default.
- `sc-meta:` - installers for wasm32 target and wasm-opt.
- Integrated traits for token management: `FixedSupplyToken`, `Mergeable`.

## [sc 0.48.0-alpha.1] - 2024-03-27 (actually alpha release of 0.49.0)
First pre-release of the unified syntax. Syntax not yet stabilized, should only be used for experimenting with various smart contracts.

## [sc 0.47.8] - 2024-03-22
- Test coverage functionality in sc-meta.
- Removed deprecation from legacy whitebox testing framework, since it is still used extensively.

## [sc 0.47.7] - 2024-03-15
- Template bugfix (concerning the interactor).

## [sc 0.47.6] - 2024-03-14
- Template naming bugfix, regarding numbers in the project name.
- Added the interactor to the adder template.

## [sc 0.47.5] - 2024-03-08
- Fixed an issue with `MapMapper` when reading from another contract.
- Got rid of nightly feature `maybe_uninit_uninit_array`/`maybe_uninit_array_assume_init`.

## [sc 0.47.4, vm 0.8.3] - 2024-02-08
- Post-build wasm report added to `.mxsc.json` file.
- Fixed a dependency issue involving ed25519-dalek (downgraded dependency).

## [sc 0.47.3, sdk 0.3.2] - 2024-02-06
- SDK: changed the way to retrieve the new deployed address after deploy/
- Support for reading from another contract for the following storage mappers: `AddressToIdMapper`, `BiDiMapper`, `LinkedListMapper`, `SetMapper`, `SingleValueMapper`, `UniqueIdMapper`, `UnorderedSetMapper`, `UserMapper`, `VecMapper`, `WhitelistMapper`.
- Additional methods to access data nodes directly in the `SetMapper` and `QueueMapper`.

## [sc 0.47.2, codec 0.18.6, vm 0.8.2, scenario-format 0.22.2] - 2024-02-02
- Scenario testing infrastructure:
	- The Rust VM can generate mock addresses, if not specified in advance.
	- The `sc:` syntax now generates addresses with VM type 0x0500, same as the latest version of mx-scenario-go.
	- Rust test support for checking `code_metadata`.
- Explicit discriminants supported for enums.
- Optimized `top_encode_number` function. It no longer contains branches or loops.
- Removed reliance on Rust nightly features `is_sorted` and `slice_partition_dedup`.

## [sc 0.47.1, codec 0.18.5, vm 0.8.1, scenario-format 0.22.1] - 2024-01-29
- Blockchain hooks: `get_code_metadata`, `is_builtin_function`.
- Support for `mxsc:` syntax in scenarios.
- Updated dependencies.

## [sc 0.47.0, codec 0.18.4, vm 0.8.0, scenario-format 0.22.0] - 2024-01-23
- Added support for the code metadata in the Rust VM and Rust scenarios backend.
- `sc-meta`:
	- New `mx-scenario-go` installer;
	- `--nocapture` flag added in `sc-meta test` CLI;
	- Framework version system refactor,
- `SetMapper` and `QueueMapper` can read from another contract.
- Fixed an edge case when generating enum encoding.

## [sc 0.46.1] - 2024-01-10
- Interactor: fixed parsing of newly issued token identifier.

## [sc 0.46.0] - 2024-01-05
- Promises callback memory allocator bugfix.
- Removed features: `promises`, `managed-map`, `back-transfers`.
- Removed `hashbrown` dependency from framework.
- Imports in output now sorted.

## [sc 0.45.2, codec 0.18.3, vm 0.7.1, scenario-format 0.21.1, sdk 0.3.1] - 2023-12-18
- Updated framework dependencies to the latest versions: syn, bitflags, wasmparser, base64, sha2, sha3, itertools, hmac, pem, pbkdf2, etc.
- `sc-meta` improvements:
	- `overflow-checks` field in `sc-config.toml`;
	- Upgrade: new `--no-check` flag, which disables the compile check after major version upgrades;
	- Template: `wasm` crates no longer copied for new versions; retroactively patched missing `multiversx.json` file for older versions.

## [sc 0.45.1, codec 0.18.2] - 2023-11-24
- Fixed sc-meta standalone install backwards compatibility.
- Better hygiene in codec derive.

## [sc 0.45.0, vm 0.7.0, scenario-format 0.21.0, sdk 0.3.0] - 2023-11-24
- Replicated VM 1.5 in the Rust VM. This includes support for:
	- promises,
	- back-transfers,
	- modified event logs.
- New endpoint annotation, `#[upgrade]`. Contract variants with upgrade endpoint, but without init now allowed.
- Build system:
	- `wasm` crates now fully generated based on data from `sc-config.toml` and root `Cargo.toml`.
	- Setting wasm target dir automatically, if not specified, based on workspace.

## [sc 0.44.0, vm 0.6.0] - 2023-11-03
- Back-transfer:
	- API support in framework (not yet implemented in the Rust VM);
	- Feature flag: `"back-transfers"`;
	- EI updated.
- ESDT attribute ABI annotation and generator.
- Multiple var-args disallowed, unless annotating endpoint with `#[allow_multiple_var_args]`.
- Build system updates:
	- `multicontract.toml` renamed to `sc-config.toml`;
	- `add-unlabelled` default true.
- New `FunctionCall` object & refactoring. Can be used as multi-value to pass contract call info to contracts.
- `AddressToId` storage mapper.


## [sc 0.43.5] - 2023-10-16
- Meta crate: removed external dependencies to `wasm2wat` and `wasm-objdump`, replaces with internal implementation.
- NFT subscription module.
- EsdtTokenData implements `ManagedVecItem`.
- Contract call `argument` method.
- `SendRawWrapper` made public.

## [sc 0.43.4] - 2023-09-18
- Bugfix in `sc-meta`: fixed `--locked argument` in `all` command.
- Template fix: added `multiversx.json` files.
- Testing framework: check NFT balances and attributes.

## [sc 0.43.3, vm 0.5.2] - 2023-09-08
- Added several new methods in the `SendWrapper`, which perform EGLD & ESDT transfers but don't do anything if the value is zero.
- Added the `DeleteUsername` builtin function to the VM.
- Minor fixes in API wrapper constructors.

## [sc 0.43.2] - 2023-08-18
- Template tool tag argument validation bugfix.

## [sc 0.43.1, vm 0.5.1] - 2023-08-18
- Template tool improvements:
	- Ability to specify for which framework version to download (based on git tag). The first allowed version is 0.43.0.
	- Ability to specify path where to create new contract.
	- Various bugfixes.
- VM implementation for `get_shard_of_address` VM hook.

## [sc 0.43.0, codec 0.18.1, vm 0.5.0] - 2023-08-16
- Fixed a rustc compatibility issue when building contracts. The meta crate looks at the rustc version when generating the wasm crate code:
	- pre-rustc-1.71;
	- between rustc-1.71 and rustc-1.73;
	- latest, after rustc-1.73. Also upgraded some dependencies, notably proc-macro2 "1.0.66" and ed25519-dalek "2.0.0".
- Initial version of the contract template tool in multiversx-sc-meta:
	- Ability to download and adapt template contracts, to kickstart contract development;
	- A template mechanism that is customizable on the framework side;
	- Available templates: adder, empty, crypto-zombies.
- The Rust debugger is now thread safe.
- Removed the `big-float` feature of multiversx-sc, because the functionality is already available on mainnet.
- Arguments `--target-dir-wasm`, `--target-dir-meta`, and `--target-dir-all` in the `multiversx-sc-meta` CLI.
- Fixed an issue with contract calls and ESDT transfers in the `StaticApi` environment.

## [sc 0.42.0, codec 0.18.0, vm 0.4.0, scenario-format 0.20.0, sdk 0.2.0] - 2023-07-15
- Multi-endpoints in multi-contracts:
	- It is now possible to have multiple versions of the same endpoint in different multi-contract variants.
	- We can also have multiple versions of the constructor.
- Major architectural redesign of the debugger:
	- The VM executor interface inserted between the smart contract API objects and the Rust VM. A new `VMHooksApi` is used to connect on the smart contract side. A `VMHooksDispatcher` object and `VMHooksHandler` interface provide the connection on the VM side.
	- The `VMHooksApi` comes in several flavors (backends):
		- The old `DebugApi` is now only used at runtime, on the VM context stack;
		- A new `StaticApi` provides support for managed types in a regular context, without needing to be initialized;
		- An additional `SingleTxApi` is useful for unit tests. Aside managed types, it also allows some basic context for tx inputs, results, storage and block info.
	- Removed almost all of the legacy functionality from the smart contract APIs.
- System SC mock.
	- It is now possible to issue tokens (fungible, SFT, NFT) in integration tests.
	- Setting roles is modelled.
	- It is, however, not fully mocked.
- Integration of blackbox and whitebox testing into one unified framework.
	- Whitebox testing was the modus operandi of the old testing framework.
	- Integration of whitebox functionality into the new testing framework allows easier migration in some specific cases.
	- Tested the new whitebox framework with the old tests by injecting it into the implementation of the old one.
- Interactors can now export a trace of their execution, thus producing integration tests.
	- Integrated tool for retrieving the initial states of the involved accounts from the blockchain.
	- Tight integration with the scenario testing infrastructure makes generating the trace straightforward;
	- The same format for the trace is used, as in the case of the integration tests.
- Interactors can now execute several steps (calls, deploys) in parallel.
- Redesigned the wrappers around the Rust and Go JSON scenario executors;
	- Also improved the  `sc-meta test-gen` tool for auto-generating these wrappers.
	- Using the `ScenarioRunner` interface to abstract away the various backends used to run tests.
- Redesigned syntax of both the testing and the interactor (snippets) frameworks.
	- While the codebases are separate (the latter is async Rust), the names and arguments of the methods are the same, and both use the scenario infrastructure.
	- Methods that allow chaining scenario steps, while also processing results;
	- Added several defaults in the syntax, for more concise code;
	- Deprecated the old testing framework;
	- Updated all contract interactors and blackbox tests with the new syntax;
	- Upgraded the snippets generator to produce new syntax.

## [sc 0.41.3, vm 0.3.3] - 2023-06-19
- Bugfix on `ManagedBufferCachedBuilder`, involving large inputs.
- Explicit enum ABI: `OperationCompletionStatus` is now properly described in the ABI as an enum that gets serialized by name instead of discriminant.

## [sc 0.41.2, codec 0.17.2, vm 0.3.2] - 2023-06-09
- Releasing a new version of the codec, without the dependency to `wee_alloc`.

## [sc 0.41.1, vm 0.3.1] - 2023-05-15
- Fixed an edge case for the token storage mappers (`FungibleTokenMapper`, `NonFungibleTokenMapper`).

## [sc 0.41.0, vm 0.3.0] - 2023-05-05
- Fixed compatibility with rustc v1.71.0.
- Allocator system:
	- Contracts can now choose their own allocator. This works in multi-contract contexts.
	- New allocators: `fail` (default), `static64k`, `leaking`.
	- Removed dependency to `wee_alloc`, but using it is still possible if the contract references it directly.
	- Contract call stack size is now configurable in `multicontract.toml`.
	- The 'panic with message' system now relies on managed buffers instead of on an allocator.
- Fixed BigUint bitwise operations in the debugger.
- When building contracts, an additional `.mxsc.json` file is created, which packs both the contract binary, the ABI, and some additional metadata.
- Refactor: reorganized the meta crate.
- Deprecated some legacy methods in the API wrappers.

## [sc 0.40.1, vm 0.2.1] - 2023-04-24
- Building contracts also triggers an EI check, which verifies compatibility with various VM versions. It currently only issues warnings.
- `ManagedVecItem` implementation for arrays.

## [sc 0.40.0, vm 0.2.0] - 2023-04-20
- Call value `egld_value` and `all_esdt_transfers` methods return `ManagedRef` instead of owned objects, because they are cached (to avoid accidental corruption of the underlying cache).

## [sc 0.39.8, vm 0.1.8] - 2023-03-29
- `multiversx-sc-meta` `test-gen` command: generates Rust integration tests based on scenarios present in the `scenarios` folder.
 - `UnorderedSetMapper` `swap_indexes` method.

## [sc 0.39.7, vm 0.1.7] - 2023-03-18
 - `TokenIdentifier` `ticker` method.
 - `ManagedBuffer` `concat` method.

## [sc 0.39.6, vm 0.1.6] - 2023-03-16
- `multiversx-sc-meta` improvements:
	- Bugfix: custom names in the main contract no longer crash the multi-contract build.
	- Bugfix: the `--mir` flag works correctly in `sc-meta all build`;
	- Multi-contract configs can now specify separate cargo features for individual contracts, for conditional compilation.

## [sc 0.39.5, vm 0.1.5] - 2023-02-06
- `multiversx-sc-meta` improvements:
	- Rust snippet generator fixes. The generator creates compilable code with appropriate argument types.
	- `local-deps` command: generates a report on the local dependencies of contract crates. Will explore indirect dependencies too.
	- Upgrade tool minor fix.

## [sc 0.39.4, vm 0.1.4] - 2023-01-26
- `multiversx-sc-meta` improvements:
	- `--locked` flag get passed to the build command, preserves dependencies in Cargo.lock.
	- `update` command updates Cargo.lock files without building the contracts.
- Backwards compatibility for running scenarios using the VM Go infrastructure.

## [sc 0.39.3, vm 0.1.3] - 2023-01-26
- `multiversx-sc-meta` improvements:
	- `upgrade` can handle crates as early as `0.28.0`;
	- `--ignore` flag for the `all` command: will ignore folders with given names, by default set to `target`;
	- `info` command, shows contracts and contract library crates with their respective framework versions;
	- `--mir` flag when building, also emits MIR files;
	- printing to console the build command.
- `BigUint` from `u128` conversion.

## [sc 0.39.2, vm 0.1.2] - 2023-01-19
- `multiversx-sc-meta` improvements:
	- `all` command that allows calling all contract meta crates in a folder;
	- `upgrade` also re-generates wasm crates after reaching 0.39.1.
- Cleaned up dependencies.

## [sc 0.39.1, codec 0.17.1, vm 0.1.1, scenario-format 0.19.1, sdk 0.1.1] - 2023-01-18
- `multiversx-sc-meta` can be installed as a standalone tool (`sc-meta`), and used to automatically upgrade contracts.
- Many dependencies updates across the repo.
- Updated readme files.

## [sc 0.39.0, codec 0.17.0, vm 0.1.0, scenario-format 0.19.0, sdk 0.1.0] - 2023-01-12
- All crates were renamed, in line with the MultiversX brand.
- New crate: `multiversx-chain-vm`, extracted from the old debug crate.
- New crate: `multiversx-sdk`, adapted from a solution proposed by the community.
- A `ScenarioWorld` facade, for contract tests.
- The meta crate supports `twiggy` post-processing, this is a tool to analyze contract size and investigate bloat in the binaries.
- Dropped crate: `elrond-wasm-output`. There is no equivalent crate, its job was passed to the individual `wasm` crates.
- `ManagedVec` supports sorting and deduplication.
- `migrateUserName` builtin function mock.

## [elrond-wasm 0.38.0, elrond-codec 0.16.0, mandos 0.18.0] - 2022-12-15
- `ContractCall` refactor. Building a contract call comes with harder compile-time constraints. This also reduces compiled code size.
- `ContractBase` supertrait can be now stated explicitly for contract and module traits.
- Debugger:
	- Callback payment is now set correctly.
	- Function names are represented internally as strings instead of bytes, which aids debugging.
- Removed the `ei-1-2` feature, which was guarding the newer VM functions. These functions are in the mainnet, so this feature is no longer needed.
- New utility functions: `self.send().esdt_local_burn_multi(...`, `self.blockchain().get_token_attributes(...)`.
- Updated all crates to Rust 2021.

## [elrond-wasm 0.37.0, elrond-codec 0.15.0] - 2022-12-09
- Multi-contract build system:
	- build system refactor;
	- `multicontract.toml` config system with labels,
	- eliminated monomorphization issue that was bloating some contracts;
	- build post-processing: `wasm2wat`, imports via `wasm-objdump`.
- Support for the new async call system (promises):
	- new APIs;
	- a new flavor of callbacks (`#[promises-callback]`);
	- callback optimizations.
- `elrond-codec` refactor: removed `TopEncodeNoErr`, `NestedEncodeNoErr` and `TypeInfo`
- System SC proxy: added support for `controlChanges` endpoint and transfer create role (from community).
- Module updates:
	- `MergedTokenInstances` module;
	- Governance module improvements;
	- `set_if_empty` for FungibleTokenMapper and NonFungibleTokenMapper.
- `IntoMultiValue` trait.
- Storage mapper improvements:
	- Storage mappers can read from another contract.
	- `BiDiMapper` improvements;
	- Fixed missing substitution rules for `FungibleTokenMapper`, `NonFungibleTokenMapper`, `UniqueIdMapper`, `BiDiMapper`, `WhitelistMapper`, `RandomnessSource`;
	- Added `take` and `replace` methods for `SingleValueMapper`;
	- Implemented `Extend` trait for `UnorderedSetMapper`.

## [elrond-wasm 0.36.1] - 2022-11-01
- Deprecated `ContractCall` `execute_on_dest_context_ignore_result` method, since it is currently redundant.

## [elrond-wasm 0.36.0, elrond-codec 0.14.0] - 2022-10-13
- `EsdtTokenPayment` legacy decode: objects encoded by older versions of the framework can now also be decoded, if flag `esdt-token-payment-legacy-decode` is active.
- Codec `NestedDecodeInput` new  `peek_into` method.
- `FungibleTokenMapper` caches the token identifier.

## [elrond-wasm 0.35.0, elrond-codec 0.13.0, mandos 0.17.0] - 2022-09-20
- Rust interactor snippet generator.
- Added some missing substitution rules in the contract preprocessor.
- Allow single zero byte when top-decoding Option::None.
- Ongoing operations module.
- Claim developer rewards module.
- `FromIterator` trait for `ManagedVec`.
- Mandos `"id"` accepted as synonym to `"txId"`.

## [elrond-wasm 0.34.1] - 2022-07-19
- `#[only_admin]` annotation
- Safer BigUint/BigInt conversions
- Added and published `price-aggregator` and `wegld-swap` core contracts.

## [elrond-wasm 0.34.0, elrond-codec 0.12.0, mandos 0.16.0, elrond-interact-snippets 0.1.0] - 2022-07-08
- Major refactor of the mandos-rs infrastructure.
	- High-level Mandos objects moved to elrond-wasm-debug;
	- The `mandos` crate no longer depends on `elrond-wasm-debug` (as originally intended and implemented);
	- Typed mandos contract call objects, for better call syntax.
	- More syntactic sugar for writing mandos calls.
- The first version of elrond-interact-snippets, which can be used to write short blockchain interactor programs.
	- The syntax relies on contract proxies to easily build calls.
	- Some of the infrastructure is shared with Mandos.
	- There is an example of such a interactor for the multisig contract.
- Refactor of managed type handles in all API traits. Eliminated undefined behavior when using the same handle in multiple contexts.
- Transfer role proxy module.
- NFT merge module.
- `#[only_user_account]` annotation. Only user accounts can call these endpoints.
- ABI - fixed missing event logs from modules.

## [elrond-wasm 0.33.1, mandos 0.15.1] - 2022-06-24
- CodecSelf for BigInt

## [elrond-wasm 0.33.0, mandos 0.15.0] - 2022-06-20
- Removed the data field for direct EGLD & ESDT transfers.
- Testing and debugging environment aligned with VM version 1.4.53.
- Call value and token data infrastructure additional cleanup.

## [elrond-wasm 0.32.0, mandos 0.14.0] - 2022-06-03
- VM new functionality added as part of the environment interface 1.2:
	- Fully managed functionality for elliptic curves (no allocator);
	- Fully managed cryptographic functions (no allocator);
	- More efficient printing of big ints and hex;
	- Functionality available by adding the `ei-1-2` flag to contracts.
- `BigFloat` functionality. Since the functionality is not yet deployed on mainnet, use flag `big-float` to use.
- Major refactoring of the call value mechanism:
	- `TokenIdentifier` now only refers to ESDT, for mixed EGLD+ESDT we have `EgldOrEsdtTokenIdentifier`.
	- `EsdtTokenPayment` now only refers to ESDT, for mixed EGLD+ESDT we have `EgldOrEsdtTokenPayment`.
	- Compact version for multi-transfer: `let [payment_a, payment_b, payment_c] = self.call_value().multi_esdt();`.
	- Explicit `single_esdt` vs. `single_fungible_esdt` vs. `egld_or_single_esdt` vs. `egld_or_single_fungible_esdt`.
	- Payment arguments are still supported, although discouraged. They always assume the EGLD+ESDT scenario.
- `ManagedOption` provides some minor optimization for specific use-cases. Mostly for use in the framework.
- Cleanup in the callback mechanism and in the `SendApi`.
- `SparseArray` implementation.
- `UniqueIdMapper` - efficient storage mapper for holding unique values.
- The ABI also contains events.
- New standard module: `StakingModule`.


## [elrond-wasm 0.31.1, mandos 0.13.1] - 2022-05-04
- Bugfix - formatter single char issue.

## [elrond-wasm 0.31.0, elrond-codec 0.11.0, mandos 0.13.0] - 2022-05-02
- Improved formatter. Strings can be formatted similarly to the standard Rust ones, but without allocator, using managed buffers. Macros `require!`, `sc_panic!`, `sc_format!`, `sc_print!` use it.
- Removed build flag `ei-1-1`, following mainnet updated and new VM endpoints being available. Among others, managed `sha256` and `keccak256` APIs can be used freely.
- `CodecFrom` and `CodecInto` traits to define equivalent encodings and conversions via codec.
- Generated smart contract proxies use the `CodecFrom`/`CodecInto` traits to accept a wider range of types.
- Mandos Rust testing framework v2, which uses contract proxies for composing calls and is capable of building and exporting mandos scenarios.
- Managed type handle management system in the contract, to reduce the number of API calls to the VM. General VM API refactor.
- Eliminated `#[var_args]` annotation. The framework can now distinguish between single-values and multi-values solely based on type.
- Contract cleans up return data after performing synchronous calls. Getting return data by range is no longer needed and the respective methods have been removed.
- Fixed behavior of blockchain API `get_esdt_token_data`.
- Git tag/commit info in ABI (fixed & reintroduced).

## [elrond-wasm 0.30.0, elrond-codec 0.10.0] - 2022-03-17
- Feature flags in `elrond-wasm`:
	- `alloc` allows contracts to use the heap allocator. It is not a hard restriction, there is still access to the implementations of the heap-allocated types, but they are not imported. Some methods are only available with this flag.
	- `ei-1-1` allows contracts to use VM endpoints that are not yet available on the mainnet.
- Fixes with async calls, smart contract deploy & upgrade.
- Refactoring regarding small number types in the API.
- Rust testing framework: Allow checking NFT balance without also checking attributes.
- View for `MapMapper`.

## [elrond-wasm 0.29.3] - 2022-03-03
- `ManagedVec` backwards compatible implementation for `set`.
- Implemented `ManagedVecItem` for `Option<T>`.

## [elrond-wasm 0.29.2] - 2022-03-01
- Disabled git tag/commit info in ABI due to issue in standard modules.

## [elrond-wasm 0.29.0] - 2022-03-01
- Cleaned up allocator from modules: `DnsModule`, `EsdtModule`, `FeaturesModule`, `PauseModule`, `UsersModule`.
- Crypto API managed wrapper over legacy VM endpoints.
- Managed multi-value types refactor and rename.
- `ManagedVec` - `remove`, `contains`, `find`.
- `ManagedVecItem` derive for simple enums.
- Feature `cb_closure_managed_deser` replaced by `cb_closure_unmanaged_deser`, managed implementation is now the default.
- Git tag/commit info in ABI.

## [elrond-wasm 0.28.0, elrond-codec 0.9.0, mandos 0.12.0] - 2022-02-22
- Major elrond-codec refactor:
	- Redesigned the error handling for single value encoding
	- Introduced multi-value encoding, which replaces the previous endpoint argument and result mechanisms
- Mandos improvements:
	- Multi-values: out, topics, ESDT uri
	- Logs "+" wildcard
- Builtin function mocks: `ESDTNFTUpdateAttributes`, `ESDTNFTAddURI`
- New storage mappers: `FungibleTokenMapper`, `NonFungibleTokenMapper`, `WhitelistMapper`
- Call value wrapper avoids using invalid token index in requests

## [elrond-wasm 0.27.4, elrond-codec 0.8.5] - 2022-02-02
- Backwards compatibility fix.

## [elrond-wasm 0.27.3] - 2022-01-31
- Backwards compatibility fix.
- Trailing commas are allowed in `sc_panic!`, `require!` and `sc_print!`.
- EsdtTokenData `decode_attributes_or_exit` for easier error handling.

## [elrond-wasm 0.27.2, elrond-codec 0.8.4] - 2022-01-27
- Added missing non-specialized decode implementations for managed types.

## [elrond-wasm 0.27.1] - 2022-01-27
- Deriving `PartialEq` now works on structs that contain managed types.

## [elrond-wasm 0.27.0] - 2022-01-25
- Fixed certain compilation error messages. The previous implementation of the macro preprocessor would have concealed the location of many issues.
- Changed implementation of `require!`:
	- `require!` no longer returns a `SCResult` type, when the condition is false it now stops the transaction immediately, via `signal_error`;
	- `require!` now accepts message formatting;
	- `require_old!` gives access to the old implementation.
- The Rust testing framework can now handle panics and async calls.
- ABI bugfix - an issue regarding nested types.
- `meta` crate build also attempts to call `wasm-opt` after building the contracts.
- Refactored `CodeMetadata` and added "payable by SC" field.
- Empty contract template.

## [elrond-wasm 0.26.0] - 2022-01-19
- Major VM API trait refactoring. All API methods can be accessed from a static context. Removed api instance variables from all objects.
- External view contracts
	- Annotating one or more endpoints with `#[external_view]` triggers the framework to create a second "external view" contract where all these endpoints are placed. This is primarily to reduce the main contract size.
	- General `meta` crate functionality refactor to allow multiple contract generation.
- `ManagedRef` type
	- Provided as a more efficient alternative to regular references to managed types
	- Has `Copy` semantics
	- `ManagedVec` iterators made safer by the proper use of lifetimes
	- `ManagedVec` `get_mut` offers a safe mutable reference, using lifetimes
	- Some initial optimizations in storage mappers
- First version of a message formatter based on `ManagedBuffer`s:
	- `sc_print!` macro
	- `sc_panic!` macro
- Random number generator wrapper over randomness source from the VM.

## [elrond-wasm 0.25.0] - 2021-12-14
- Rust testing framework - mandos generation fixes and some more getters
- Standard modules moved to `elrond-wasm-modules` crates

## [elrond-wasm 0.24.0] - 2021-12-07
- Rust testing framework
- Managed Crypto API - keccak256 and sha256
- New hook for ESDT local roles
- Only-owner module annotation

## [elrond-wasm 0.23.1, elrond-codec 0.8.3] - 2021-11-25
- `ArrayVec` serialization
- `ManagedAddress` additional conversions

## [elrond-wasm 0.23.0] - 2021-11-23
- Static access to API. Static thread-local context stack in the debugger.

## [elrond-wasm 0.22.11] - 2021-11-17
- Derive `ManagedVecItem` generics fix
- Constructor can reside in module

## [elrond-wasm 0.22.10] - 2021-11-12
- `ManagedMultiResultVec` push accepts multi result

## [elrond-wasm 0.22.9] - 2021-11-12
- `ManagedVarArgsEager` implementation
- `EsdtLocalRoleFlags`, no heap allocation in `get_esdt_local_roles`

## [elrond-wasm 0.22.8, elrond-codec 0.8.2] - 2021-11-12
- Optimized decode unsigned number from slice

## [elrond-wasm 0.22.7] - 2021-11-12
- Optimized decode unsigned number from slice
- Optimized blockchain API: managed get token nonce, get esdt balance
- `ManagedVecItem` for `ManagedByteArray`

## [elrond-wasm 0.22.6] - 2021-11-11
- Optimized decode u64 from `ManagedBuffer`
- `ManagedVecItem` in `derive_imports`

## [elrond-wasm 0.22.5] - 2021-11-11
- Implemented `ManagedVecItem` for `bool`.
- Substitution for `ManagedMultiResultVec::new()`.

## [elrond-wasm 0.22.4] - 2021-11-11
- Derive `ManagedVecItem`.
- Nested encode and decode from ManagedBuffers cached in a static singleton buffer.
- Implemented `ExactSizeIterator` for `ManagedVecIterator`.

## [elrond-wasm 0.22.3] - 2021-11-10
- Memory allocation optimisations.

## [elrond-wasm 0.22.2] - 2021-11-06
- Callback endpoint automatically created empty for contracts that have no callbacks. This is determined by the `meta` crate, based on the ABI of the contract and its modules.
- `UnorderedSetMapper`
- `IgnoreVarArgs` variadic argument type that ignores input

## [elrond-wasm 0.22.1] - 2021-11-04
- Made the generated code in `wasm/lib.rs` more compact with the use of macros.

## [elrond-wasm 0.22.0] - 2021-11-02
- Mechanism for generating contract endpoints based on ABI. Previously, all endpoints from all modules from a crate were automatically included, now they can be filtered based on what modules are used.
- Contract `meta` crates are now capable of building the respective contracts and the ABIs without relying on `erdpy`.
- Renamed feature `arwen-tests` to `mandos-go-tests`

## [elrond-wasm 0.21.2] - 2021-10-26
- Bugfix regarding contract upgrade args in `elrond-wasm-debug`

## [elrond-wasm 0.21.1, elrond-codec 0.8.1, mandos 0.11.1] - 2021-10-26
- Relative path improvements and fixes in `elrond-wasm-debug`:
	- mandos-rs `file:` syntax now actually loads files and correctly unifies equivalent paths
	- debugging now works seamlessly, without needing to temporarily change paths in the tests
- SC proxy - `register_meta_esdt`
- Debugger builtin function mocks check for ESDT roles
- ABI provides definitions for EsdtTokenPayment, EsdtTokenData, EsdtTokenType

## [elrond-wasm 0.21.0, elrond-codec 0.8.0, mandos 0.11.0] - 2021-10-22
- Mandos support for NFT syntax. Many more small improvements and some major refactoring.
- Major refactoring of the `elrond-wasm-debug` crate, which enables the debugger and the coverage tool. Many features added:
	- support for synchronous calls, also nested synchronous calls
	- support for NFT simple transfers
	- support for ESDT multitransfer (FT + NFT)
	- builtin functions mocked in the debugger: `ESDTLocalMint`, `ESDTLocalBurn`, `MultiESDTNFTTransfer`, `ESDTNFTTransfer`, `ESDTNFTCreate`, `ESDTNFTAddQuantity`, `ESDTNFTBurn`, `ESDTTransfer`, `ChangeOwnerAddress`, `SetUserName`
	- supports deploy/deploy from source/upgrade/upgrade from source from contracts
- `#[payment_multi]` annotation
- `ManagedRef` type, that allows easier handling of managed types
- ABI contains endpoint mutability flag (mutable/readonly)
- reverse iteration for `ManagedVec`

## [elrond-wasm 0.20.1] - 2021-10-05
- Added missing managed methods in blockchain API: `is_smart_contract`, `get_shard_of_address`, `get_balance`.
- Improved preprocessor substitutions: `ManagedAddress`, `TokenIdentifier`.

## [elrond-wasm 0.20.0, elrond-codec 0.7.0, mandos 0.10.0] - 2021-10-02
- Managed callback handling
- Managed async call result
- ManagedVec improvements, deserialization fix
- Better conversions between big numeric types
- Improved preprocessor substitutions: hidden generics for most managed types
- Build info in ABI - rustc version, framework version, crate version

## [elrond-wasm 0.19.1] - 2021-09-17
- Legacy Send API implementation fix

## [elrond-wasm 0.19.0, elrond-codec 0.6.0, mandos 0.9.0] - 2021-09-10
- Managed types used extensively. Because of this, the recommended Arwen minimum version is `v1.4.10`.
	- Redesigned parts of the elrond-codec, so as to allow custom type specializations. These specializations allow serializers and types to bypass the limitations of the codec traits to provide optimized implementations. Managed type serialization relies on this.
	- Redesigned existing managed types: `BigInt`, `BigUint`, `EllipticCurve`.
	- Added the `ManagedBuffer` type, which can be used to store anything on the VM side.
	- Support for complex operations using managed buffers, such as storing lists of elements in a managed buffer via the `ManagedVec` type.
	- There are `ManagedAddress`es now. They rely on another managed type, the `ManagedByteArray`, which is a fixed size managed structure.
	- `TokenIdentifier` is now a managed type.
	- Serializer based on a managed buffer.
	- Storage keys are now based on managed buffers.
	- All error messages generated by the framework are assembled using a managed buffer.
	- The blockchain API uses managed types for most interactions.
	- The contract call API uses managed types for most interactions.
	- The call value API supports multi transfer via managed `EsdtTokenPayment` objects.
	- Event logs are sent to the VM via managed types (`ManagedVec<ManagedBuffer>` for topics, `ManagedBuffer` for data).
	- Type conversion traits for managed types: `ManagedFrom` and `ManagedInto`.
	- There are now 2 types of `SCError`: `StaticSCError` for static messages and `ManagedSCError`, which is backed by a managed buffer.
	- Contract errors can now be triggered immediately, without the need to return them from an endpoint.
- Improved macro preprocessor: more complex patterns can now be substituted.
	- Generic API parameter needs not be specified every time.
	- Substitutions available for most managed types and storage mappers.
- Separated contract API into low-level VM API connectors and high-level utility objects to be used in the contracts.
- Mandos-rs improvements:
	- Self tests synchronized with mandos-go. Some missing features needed to be added to make them pass.
	- Support for ESDT tokens.
	- Support for ESDT multi-transfer.


## [elrond-wasm 0.18.2] - 2021-08-20
- Crypto API: `ripemd160` function, custom secp256k1 signature verification (`verify_custom_secp256k1`) and signature generation (`encode_secp256k1_der_signature`).

## [elrond-wasm 0.18.1] - 2021-08-05
- Added "safe" storage mappers, which serialize keys using nested encoding instead of top. The old respective mappers only kept for backwards compatibility, are now deprecated.

## [elrond-wasm 0.18.0, mandos 0.8.0] - 2021-07-28

- New math hooks exposed from Arwen:
	- `pow`, `log2`, `sqrt`
	- cryptography: elliptic curves
- `deploy_contract` now returns `Option<Address>`
- `deploy_from_source_contract` API
- Send API refactored for more consistency and ease of use.
- High level proxies can be used to deploy contracts.
- Mandos log syntax updated, to match Arwen.
- A better `#[only_owner]` annotation, which can be applied directly to endpoint methods. This annotation also shows up in the ABI.
- `elrond-wasm-derive` now an optional dependency of `elrond-wasm`. Use `#[elrond_wasm::contract]` instead of `#[elrond_wasm_derive::contract]` now. Same for proxies and modules.

## [elrond-wasm 0.17.4] - 2021-06-30
- conversions from big ints to small int: `BigUint::to_u64`, `BigInt::to_i64`

## [elrond-wasm 0.17.3] - 2021-06-11
- `SingleValueMapper` `set_if_empty` method

## [elrond-wasm 0.17.2] - 2021-06-04
- callbacks can now declared in modules only (manual forwarding from the main contract no longer required)

## [elrond-wasm 0.17.1] - 2021-06-04
- `legacy-nft-transfer` feature for interacting with older versions of Arwen

## [elrond-wasm 0.17.0] - 2021-05-28
- Integration tests can now call Arwen-Mandos (mandos-go)
- Send API refactoring and cleanup
	- ESDT builtin function calls no longer require explicit gas
	- sync calls and transfer-execute no longer require explicit gas
- `#[payment_nonce]` endpoint argument annotation
- `#[payable]` annotation no longer allowed without argument

## [elrond-wasm 0.16.2, mandos 0.7.2] - 2021-05-20
- New implementation for the `Try` trait for `SCResult`, in accordance to feature `try_trait_v2`
- Published DNS module, which helps contracts register usernames for themselves
- `ESDTLocalRole` more expressive type ABI

## [elrond-wasm 0.16.1, mandos 0.7.1] - 2021-05-18
- Improvements in mandos-rs: username, contract owner, nested async calls

## [elrond-wasm 0.16.0, mandos 0.7.0, elrond-codec 0.5.3] - 2021-05-14
### Major redesign of important framework components:
- The arguments to contract/module/proxy annotations are gone. All items are generated in the same Rust module. Both submodule inclusion and contract calls are now Rust-module-aware.
- Submodule imports are now expressed as supertraits instead of the module getter annotated methods. Note: explicitly specifying the Rust module is required, in order for the framework to fetch generated types and functions from that module.
- Each contract now generates its own callable proxy to ease calling it. Caller contracts do no longer need to define a call interface, they can import it from the crate of the contract they want to call. Callable proxies contain the methods from the main contract, as well as from all the modules. Note: calling a contract requires the caller to specify the Rust module where it resides.
- We no longer have a separate syntax/parser/code generation for call proxies. They are just contracts with no implementations and annotated with `#[elrond_wasm_derive::proxy]` instead of `#[elrond_wasm_derive::contract]`.
- BigUint and BigInt are now associated types instead of generics in all API traits. Contracts need to specify them as `Self::BigUint` instead of just `BigUint`. Although more verbose, this might be more intuitive for the developer.
- `ContractCall`s, `AsyncCall`s and all other call & transfer result types now contain a reference to the Send API. This also means the `execute_on_dest_context` method no longer requires an api argument.
- `execute_on_dest_context` can now deserialize the call results automatically and provide them to the calling contract. There is a mechanism in place to deconstruct non-serialized types, e.g. `SCResult<T>` becomes `T` and `AsyncCall<Self::BigUint>` becomes `()`. 
- Callbacks and callback proxies needed to be adapted to the new system, but work similar to how they did in the past.
- Contracts can define proxy getter methods using the `#[proxy]` annotation.
- Callbacks can now have names, just like endpoints. This name gets saved in the callback closure in storage, but has no other impact on the contract. The reason I needed it was to help me with defining callback forwarders and avoiding some name collisions there. Callback forwarders are still needed for a little longer, until module callbacks are properly implemented.

### Mandos
- mandos-rs syntax synchronized with mandos-go (`sc:` syntax, new ESDT call value syntax, _no NFTs yet_).

## [elrond-wasm 0.15.1] - 2021-04-30
- Mitigating nested sync calls with Send API `execute_on_dest_context_raw_custom_result_range`

## [elrond-wasm 0.15.0, elrond-codec 0.5.2] - 2021-04-19
- ABI
	- Constructor representation
	- Simplified ABI syntax for tuples and fixed-size arrays
- Final cleanup for the contract APIs: split off blockchain and crypto APIs
- Small fixes in the send API
- `TokenIdentifier` validation
- Minor refactoring in the elrond-codec 

## [elrond-wasm 0.14.2] - 2021-03-29
- Fixed contract call/callback logs in mandos-rs

## [elrond-wasm 0.14.1] - 2021-03-25
- Unified variadic arguments with respective variadic results

## [elrond-wasm 0.14.0, mandos 0.6.0, elrond-codec 0.5.1] - 2021-03-22
- ESDT functionality:
	- ESDT system smart contract proxy, though which it is possible to mint, burn, issue, freeze, pause, etc.
	- Endpoints to handle NFTs. Also added NFT management in the  ESDT system smart contract proxy
	- Get balance, get token data, local mint/burn
- Contract calls:
	- Low-level and high-level support for synchronous calls via `execute_on_dest_context`.
	- Callback bug fix
- Improvements in storage mappers:
	- VecMapper length is now lazy
	- UserMapper more functionality
- Mandos
	- `scQuery` step
	- fixed defaults: unspecified fields now check the default value instead of being ignored
	- check logs
	- `nested:` and `biguint:` syntax
- `elrond-codec-derive` dix - `TopDecodeOrDefault` works with generics
- Upgraded to Rust2021.

## [elrond-wasm 0.13.0] - 2021-03-04
### Main feature
- Events revamped:
	- any event name of any length is accepted. The event name is now expressed as ASCII instead of hex
	- topics can have any length
	- topics and data are serialized using the elrond-codec instead of the old macro-based solution
	- old events are still allowed for now via the `#[legacy_event("0x...")]` syntax; might be removed in the future
### Refactoring 
- Major refactoring of elrond-wasm-derive. This doesn't change much of the functionality, though.
### Minor features
- SingleValueMapper redesigned for easier use. It no longer keeps the storage value cached.

## [elrond-wasm 0.12.0] - 2021-02-25
- Reorganized ESDT and EGLD direct send api.
- New async call syntax
	- redesigned contract proxies
	- contract calls are communicated via objects returned from endpoint methods
	- callbacks now specified programmatically
	- got rid of the `#[callback_arg]` annotation

## [elrond-wasm 0.11.0, elrond-codec 0.5.0, mandos 0.5.0] - 2021-02-05
### Refactor
- Major refactoring of the contract API: split into smaller traits
### Added
- Storage mappers:
	- LinkedListMapper
	- SetMapper
	- MapMapper
- SendApi
	- created SendApi, which groups all functionality related to sending tokens and interactions with other contracts
    - integrated the new TransferESDT hook from Arwen
    - added an unsafe buffer for handling values before transfer
    - mandos-rs fixes
    - contracts now use the new API + more mandos tests
- Call Value API refactor and `#[payable]` updates:
	- Main features:
    	- `#[payable]` annotation more versatile: `#[payable("EGLD")]` `#[payable("TOKEN-ID")]` `#[payable("*")]`
    	- `#[payable]` still accepted but throws a warning, will become unsupported in the future.
    	- `#[payment]` argument attribute now also provides ESDT payment where applicable
    	- a new TokenIdentifier type that encodes the EGLD special token and any ESDT token
    	- a new `#[token_identifier]` argument attribute provides the token id. Similar to `#[payment]` it is a fake argument, not exported.
    	- ABI updated ("payableInTokens" is no longer restricted to "EGLD")
    	- all new features covered by mandos tests
    	- async proxies still only accept `#[payable("EGLD")]`, but that is for future updates
	- Less visible changes:
    	- all call value hooks now grouped in a new CallValueApi
    	- for low-level access, developers now need to write self.call_value().egld_value(), etc.
    	- some optimizations in the handling of call value hooks
	- Refactoring:
    	- parse_attr mod was split into a proper folder with many files, since it had grown too large
    	- an extensive refactoring of elrond-wasm-derive not yet performed, will come soon
### Minor features
- ABI enum discriminants generation
### Fixed
- Crypto API fixes:
	- `keccak256:` prefix also supported in mandos
    - reorganized crypto mandos tests in basic-features
    - mandos-rs was accidentally providing keccak256 instead of sha256


## [elrond-wasm 0.10.5] - 2021-01-27
- Temporary fix: callbacks allow error message argument to be missing

## [elrond-wasm 0.10.4, mandos 0.4.2] - 2021-01-13
- Codec derive with defaults
- Storage mapper infrastructure

## [elrond-wasm 0.10.3] - 2020-12-29
- ABI generation of endpoint output names

## [elrond-wasm 0.10.2, elrond-codec 0.4.2] - 2020-12-28
- Codec type hygiene

## [elrond-wasm 0.10.1, elrond-codec 0.4.1, mandos 0.4.1] - 2020-12-23
- Minor fixes, support for strings

## [elrond-wasm 0.10.0, elrond-codec 0.4.0] - 2020-12-21
- Codec derive
- ABI generation framework
- New example contracts

## [elrond-wasm 0.9.8, elrond-codec 0.3.2, mandos 0.3.1] - 2020-11-23
- SC deploy API

## [elrond-wasm 0.9.7, elrond-codec 0.3.1, mandos 0.3.0] - 2020-11-11
- Monomorphization via codec trait instead of TypeInfo for arguments and storage
- Reorganized all contracts in the `contracts` folder

## [elrond-wasm 0.9.6] - 2020-11-09
- H256 & BoxedBytes fixes

## [elrond-wasm 0.9.5] - 2020-11-09
- H256 is_zero, minor fixes

## [elrond-wasm 0.9.4] - 2020-11-09
- BoxedBytes
	- optimized allocation, used in hooks
	- used for error messages

## [elrond-wasm 0.9.3] - 2020-11-08
- Optimized Address/H256 hooks

## [elrond-wasm 0.9.2] - 2020-11-06
- Allow slices as arguments 
- `storage_is_empty` annotation

## [elrond-wasm 0.9.1] - 2020-11-05
- BigUint serialization bugfix

## [elrond-wasm 0.9.0, elrond-codec 0.3.0, mandos 0.2.0] - 2020-11-04
- Serialization completely refactored to use "fast exit" methods
- Storage/argument/result traits completely redesigned, simplified and optimized
- Completely ditched the approach from elrond-wasm 0.8.0.

## [elrond-wasm 0.8.0, elrond-codec 0.2.0] - 2020-11-02
- Was the first version to split Encode/Decode into TopEncode/NestedEncode/TopDecode/NestedDecode
- Attempted to optimize the serializer to use "fast exit" closures. It worked, but the resulting bytecode size was not satisfactory. Even though it was completely replaced and never got to be used, it historically remains the solution of this release.
- Some of the storage/argument/result trait refactorings, survived.

## [elrond-wasm 0.7.2] - 2020-10-16
- small int EI
- minor refactors, serialization fixes

## [elrond-wasm 0.7.1] - 2020-10-07
- Avoid function selector infinite loop
- Crowdfunding contract initial commit

## [elrond-wasm 0.7.0, mandos 0.1.0] - 2020-10-06
- Code coverage now possible
- Mandos in Rust
- Modules properly integrated in the build process

## [elrond-wasm 0.6.2] - 2020-09-16
- NonZeroUsize iterator and utils

## [elrond-wasm 0.6.1, elrond-codec 0.1.3] - 2020-09-15
- Integrated NonZeroUsize into the framework
- Specialized small int top encoding/decoding
- `only_owner!` macro

## [elrond-wasm 0.6.0, elrond-codec 0.1.2] - 2020-08-25
- Redesigned the entire build process with wasm crates
- Standard modules
- Moved all example contracts from sc-examples-rs to the framework

## [elrond-wasm 0.5.5] - 2020-07-27
- H256 now boxed
- SCResult is_ok, is_err

## [elrond-wasm 0.5.4, elrond-codec 0.1.1] - 2020-07-18
- MultiResultVec - new, from_iter
- EncodeError type

## [elrond-wasm 0.5.3, elrond-codec 0.1.0] - 2020-07-10
- Extracted elrond-codec to separate crate
- Fixed non_snake_case endpoint handling

## [elrond-wasm 0.5.2] - 2020-07-09
- Queue type

## [elrond-wasm 0.5.1] - 2020-07-02
- `#[view]` attribute, same as `#[endpoint]`
- `#[init]` attribute
- `storage get mut` annotation + BorrowedMutStorage
- Encode for references
- Array serialization/deserialization
- Option serialization fix
- Arg name in error message
- Async call arguments based on traits

## [elrond-wasm 0.5.0] - 2020-06-29
- EndpointResult trait, arg serialization trait, arg loader
- Variadic args/results: OptionalArg, OptionalResult, MultiResultX

## [elrond-wasm 0.4.6] - 2020-06-21
- MultiResultVec implementation
- Callback varargs

## [elrond-wasm 0.4.5] - 2020-06-09
- `storage_set` allows slices
- H256 to_vec
- async call and callback argument fixes
- eliminate bloat when no callback
- the new elrond lightweight serializer (would later become elrond-codec)
- imports macro
- OtherContractHandle implementation

## [elrond-wasm 0.4.4] - 2020-05-19
- Serialization fixes for small ints
- `get_cumulated_validator_rewards` hook

## [elrond-wasm 0.4.3] - 2020-05-11
- Allow any (macro-based) serializable argument in async call
- `#[var_args]`
- Call data serialization refactoring

## [elrond-wasm 0.4.2] - 2020-05-07
- Tutorial setup (later abandoned)

## [elrond-wasm 0.4.1] - 2020-05-06
- Direct storage conversion for simple types
- Block info hooks

## [elrond-wasm 0.4.0] - 2020-05-06
- Serde-based serializer (later abandoned)
- Major storage improvements:
	- Generate storage getters & setters
	- Variable length storage keys

## [elrond-wasm 0.3.2] - 2020-04-13
- Fixes in the macro-based argument handling

## [elrond-wasm 0.3.0] - 2020-04-03
- Raw callback support
- `storage_load_len` hook
- Multi args
- Multi args in async calls

## [elrond-wasm 0.2.0] - 2020-03-18
- BigUint trait created, added operators (including bitwise)
- BigUint used for balances

## [elrond-wasm 0.1.1] - 2020-02-27
- Async call contract proxy infrastructure

## [elrond-wasm 0.1.0] - 2020-02-05 
- Initial release of the framework
- Main features at this time:
	- contract main macro
	- handling of arguments and results automagically using macros
	- BigInt generic type, hooked directly to the Arwen big int heap
	- `#[private]` attribute

## [Initial commit] - 2020-01-04
- Early framework moved here from sc-examples
- 4 crates:
	- elrond-wasm
	- elrond-wasm-derive for macros
	- elrond-wasm-node for wasm
	- elrond-wasm-debug for debugging and early tests
