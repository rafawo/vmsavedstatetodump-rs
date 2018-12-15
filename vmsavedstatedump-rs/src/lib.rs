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

pub(crate) mod windefs {
    use libc;

    pub type DWord = libc::c_ulong;
    pub type HResult = libc::c_long;
    pub type LPCWStr = *const libc::wchar_t;
    pub type LPWStr = *mut libc::wchar_t;
    pub type PVoid = *mut Void;
    pub type Void = libc::c_void;
}
