// Copyright (c) 2019 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! This crate provides safe Rust abstractions to
//! [VmSavedStateDumpProvider](https://docs.microsoft.com/en-us/virtualization/api/vm-dump-provider/vm-dump-provider).
//!
//! # Usage
//!
//! To be able to run code that consumes these APIs, vmsavedstatedumpprovider.dll must be discoverable
//! from within the executables/binary runtime.
//!
//! When using these APIs, the main entry point is to create a VmSavedStateDumpProvider
//! by supplying path(s) to bin/vsv - vmrs VM saved state file(s).
//!
//! ```rust,ignore
//! let vmrs_provider = VmSavedStateDumpProvider::load_vmrs("file_path.vmrs");
//! let bin_vsv_provider = VmSavedStateDumpProvider::load_bin_vsv("file_path.bin", "file_path.vsv");
//! ```
//!
//! Once a provider has been instantiated, all of its related APIs can be used in the context
//! of a loaded VM saved state file.
//!
//! The best source of code examples on how to use the APIs are the integration tests,
//! found [here](https://github.com/rafawo/vmsavedstatetodump-rs/blob/master/vmsavedstatedump-rs/tests/integration_test.rs).

pub mod vmsavedstatedump;
pub(crate) mod vmsavedstatedump_bindings;
pub mod vmsavedstatedumpdefs;

#[cfg(feature = "mindumpdef")]
pub mod mindumpdef;

/// Module that declares aliases to windows definitions used by this crate
/// using Rust's naming conventions.
pub(crate) mod windefs {
    pub type DWord = winapi::shared::minwindef::DWORD;
    pub type HResult = winapi::shared::ntdef::HRESULT;
    pub type LPCWStr = winapi::shared::ntdef::LPCWSTR;
    pub type LPWStr = winapi::shared::ntdef::LPWSTR;
    pub type PVoid = winapi::shared::ntdef::PVOID;
    pub type Void = winapi::shared::ntdef::VOID;
}
