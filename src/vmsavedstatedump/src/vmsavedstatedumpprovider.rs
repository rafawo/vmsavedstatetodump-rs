//! This module implements safe wrappers of the unsafe API surface to VmSavedStateDump.
//! Defines and provides Rust idiomatic abstractions of the API.

use crate::vmsavedstatedump::*;
use crate::vmsavedstatedumpdefs::*;
use crate::windefs::*;

use std::ops;
use widestring::U16CString;

/// Common error types that can be returned by the VmSavedStateDumpProvider API.
pub enum Error {
    Success(HResult),
    OutOfMemory(HResult),
    FileNotFound(HResult),
    Fail(HResult),
    Unexpected(HResult),
    WindowsHResult(HResult),
}

fn hresult_to_error_code(hresult: &HResult) -> Error {
    // TODO: fill in the other cases
    match hresult {
        0 => Error::Success(0),
        other => Error::WindowsHResult(other.clone()),
    }
}

/// Applies a pending replay log to a VMRS file.
pub fn apply_pending_replay_log(vmrs: &str) -> HResult {
    unsafe { ApplyPendingSavedStateFileReplayLog(U16CString::from_str(vmrs).unwrap().as_ptr()) }
}

/// Supported VM saved state file formats.
enum VmSavedStateFile {
    BinVsv(String, String),
    Vmrs(String),
}

/// Structure that abstracts access to a loaded VM Saved state file and its dump related APIs.
pub struct VmSavedStateDumpProvider {
    handle: VmSavedStateDumpHandle,
    saved_state: VmSavedStateFile,
}

impl ops::Drop for VmSavedStateDumpProvider {
    fn drop(&mut self) {
        unsafe {
            // We ignore error code on purpose
            ReleaseSavedStateFiles(self.handle.clone());
        }
    }
}

impl VmSavedStateDumpProvider {
    /// Loads a BIN/VSV VM Saved state files and returns a VmSavedStateDumpProvider instance
    /// that provides the interface to the dump related APIs.
    pub fn load_bin_vsv(bin: &str, vsv: &str) -> Result<VmSavedStateDumpProvider, Error> {
        let mut dump_handle: VmSavedStateDumpHandle = std::ptr::null_mut();
        let result: HResult;

        unsafe {
            result = LoadSavedStateFiles(
                U16CString::from_str(bin).unwrap().as_ptr(),
                U16CString::from_str(vsv).unwrap().as_ptr(),
                &mut dump_handle,
            );
        }

        match hresult_to_error_code(&result) {
            Error::Success(_) => Ok(VmSavedStateDumpProvider {
                handle: dump_handle,
                saved_state: VmSavedStateFile::BinVsv(String::from(bin), String::from(vsv)),
            }),
            error => Err(error),
        }
    }

    /// Loads a VMRS VM Saved state file and returns a VmSavedStateDumpProvider instance
    /// that provides the interface to the dump related APIs.
    pub fn load_saved_state_file(vmrs: &str) -> Result<VmSavedStateDumpProvider, Error> {
        let mut dump_handle: VmSavedStateDumpHandle = std::ptr::null_mut();
        let result: HResult;

        unsafe {
            result = LoadSavedStateFile(
                U16CString::from_str(vmrs).unwrap().as_ptr(),
                &mut dump_handle,
            );
        }

        match hresult_to_error_code(&result) {
            Error::Success(_) => Ok(VmSavedStateDumpProvider {
                handle: dump_handle,
                saved_state: VmSavedStateFile::Vmrs(String::from(vmrs)),
            }),
            error => Err(error),
        }
    }

    /// Returns the virtual processor count.
    // TODO:Implement a VirtualProcessor struct abstraction
    // that provides an iterator and all the functions that operate
    // over the VP, so that the rest of the functions don't have to
    // manually specify each VP ID and it's safer to work on them.
    pub fn vp_count(&self) -> Result<u32, Error> {
        let mut vp_count = 0;
        let result: HResult;

        unsafe {
            result = GetVpCount(self.handle.clone(), &mut vp_count);
        }

        match hresult_to_error_code(&result) {
            Error::Success(_) => Ok(vp_count),
            error => Err(error),
        }
    }

    /// Returns the virtual processor architecture.
    pub fn get_vp_architecture(&self, vp_id: u32) -> Result<VirtualProcessorArch, Error> {
        let mut vp_arch = VirtualProcessorArch::Unknown;
        let result: HResult;

        unsafe {
            result = GetArchitecture(self.handle.clone(), vp_id, &mut vp_arch);
        }

        match hresult_to_error_code(&result) {
            Error::Success(_) => Ok(vp_arch),
            error => Err(error),
        }
    }

    /// Returns a virtual processor register value.
    pub fn get_vp_register_value(&self, vp_id: u32) -> Result<VirtualProcessorRegister, Error> {
        let mut vp_register_value: VirtualProcessorRegister = VirtualProcessorRegister {
            architecture: VirtualProcessorArch::Unknown,
            register_value: 0,
            raw_id: RegisterRawId { register_id: 0 },
        };
        let result: HResult;

        unsafe {
            result = GetRegisterValue(self.handle.clone(), vp_id, &mut vp_register_value);
        }

        match hresult_to_error_code(&result) {
            Error::Success(_) => Ok(vp_register_value),
            error => Err(error),
        }
    }

    /// Returns a virtual processor paging mode.
    pub fn get_vp_paging_mode(&self, vp_id: u32) -> Result<PagingMode, Error> {
        let mut vp_paging_mode = PagingMode::Invalid;
        let result: HResult;

        unsafe {
            result = GetPagingMode(self.handle.clone(), vp_id, &mut vp_paging_mode);
        }

        match hresult_to_error_code(&result) {
            Error::Success(_) => Ok(vp_paging_mode),
            error => Err(error),
        }
    }

    /// Reads a sized guest physical address into the supplied buffer.
    pub fn read_guest_physical_address(
        &self,
        physical_address: GuestPhysicalAddress,
        buffer: &mut [u8],
    ) -> Result<u32, Error> {
        let buffer_size = buffer.len() as u32;
        let buffer_ptr = buffer.as_mut_ptr();
        let mut bytes_read: u32 = 0;
        let result: HResult;

        unsafe {
            result = ReadGuestPhysicalAddress(
                self.handle.clone(),
                physical_address,
                buffer_ptr as PVoid,
                buffer_size,
                &mut bytes_read,
            );
        }

        match hresult_to_error_code(&result) {
            Error::Success(_) => Ok(bytes_read),
            error => Err(error),
        }
    }

    /// Translates a virtual address to a physical address using information found in the
    /// guest's memory and processor's state.
    pub fn guest_virtual_to_physical_address(
        &self,
        vp_id: u32,
        virtual_address: GuestVirtualAddress,
    ) -> Result<GuestPhysicalAddress, Error> {
        let mut physical_address: GuestPhysicalAddress = 0;
        let result: HResult;

        unsafe {
            result = GuestVirtualAddressToPhysicalAddress(
                self.handle.clone(),
                vp_id,
                virtual_address,
                &mut physical_address,
            );
        }

        match hresult_to_error_code(&result) {
            Error::Success(_) => Ok(physical_address),
            error => Err(error),
        }
    }
}
