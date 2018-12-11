//! This module implements safe wrappers of the unsafe API surface to VmSavedStateDump.
//! Defines and provides Rust idiomatic abstractions of the API.

use crate::vmsavedstatedump::*;
use crate::vmsavedstatedumpdefs::*;
use crate::windefs::*;

use std::ops;
use widestring::U16CString;

/// Common error types that can be returned by the VmSavedStateDumpProvider API.
pub enum ResultCode {
    Success(HResult),
    OutOfMemory(HResult),
    FileNotFound(HResult),
    Fail(HResult),
    InvalidArgument(HResult),
    Unexpected(HResult),
    WindowsHResult(HResult),
}

#[allow(overflowing_literals)]
fn hresult_to_result_code(hresult: &HResult) -> ResultCode {
    let out_of_memory: HResult = 0x8007000E;
    let file_not_found: HResult = 0x80030002;
    let fail: HResult = 0x80004005;
    let invalid_argument: HResult = 0x80070057;
    let unexpected: HResult = 0x8000FFFF;

    match hresult {
        0 => ResultCode::Success(0),
        0x8007000E => ResultCode::OutOfMemory(out_of_memory),
        0x80030002 => ResultCode::FileNotFound(file_not_found),
        0x80004005 => ResultCode::Fail(fail),
        0x80070057 => ResultCode::Fail(invalid_argument),
        0x8000FFFF => ResultCode::Fail(unexpected),
        other => ResultCode::WindowsHResult(other.clone()),
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

    #[allow(dead_code)]
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
    pub fn load_bin_vsv(bin: &str, vsv: &str) -> Result<VmSavedStateDumpProvider, ResultCode> {
        let mut dump_handle: VmSavedStateDumpHandle = std::ptr::null_mut();
        let result: HResult;

        unsafe {
            result = LoadSavedStateFiles(
                U16CString::from_str(bin).unwrap().as_ptr(),
                U16CString::from_str(vsv).unwrap().as_ptr(),
                &mut dump_handle,
            );
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(VmSavedStateDumpProvider {
                handle: dump_handle,
                saved_state: VmSavedStateFile::BinVsv(String::from(bin), String::from(vsv)),
            }),
            error => Err(error),
        }
    }

    /// Loads a VMRS VM Saved state file and returns a VmSavedStateDumpProvider instance
    /// that provides the interface to the dump related APIs.
    pub fn load_saved_state_file(vmrs: &str) -> Result<VmSavedStateDumpProvider, ResultCode> {
        let mut dump_handle: VmSavedStateDumpHandle = std::ptr::null_mut();
        let result: HResult;

        unsafe {
            result = LoadSavedStateFile(
                U16CString::from_str(vmrs).unwrap().as_ptr(),
                &mut dump_handle,
            );
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(VmSavedStateDumpProvider {
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
    pub fn vp_count(&self) -> Result<u32, ResultCode> {
        let mut vp_count = 0;
        let result: HResult;

        unsafe {
            result = GetVpCount(self.handle.clone(), &mut vp_count);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(vp_count),
            error => Err(error),
        }
    }

    /// Returns the virtual processor architecture.
    pub fn get_vp_architecture(&self, vp_id: u32) -> Result<VirtualProcessorArch, ResultCode> {
        let mut vp_arch = VirtualProcessorArch::Unknown;
        let result: HResult;

        unsafe {
            result = GetArchitecture(self.handle.clone(), vp_id, &mut vp_arch);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(vp_arch),
            error => Err(error),
        }
    }

    /// Returns a virtual processor register value.
    pub fn get_vp_register_value(
        &self,
        vp_id: u32,
    ) -> Result<VirtualProcessorRegister, ResultCode> {
        let mut vp_register_value: VirtualProcessorRegister = VirtualProcessorRegister {
            architecture: VirtualProcessorArch::Unknown,
            register_value: 0,
            raw_id: RegisterRawId { register_id: 0 },
        };
        let result: HResult;

        unsafe {
            result = GetRegisterValue(self.handle.clone(), vp_id, &mut vp_register_value);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(vp_register_value),
            error => Err(error),
        }
    }

    /// Returns a virtual processor paging mode.
    pub fn get_vp_paging_mode(&self, vp_id: u32) -> Result<PagingMode, ResultCode> {
        let mut vp_paging_mode = PagingMode::Invalid;
        let result: HResult;

        unsafe {
            result = GetPagingMode(self.handle.clone(), vp_id, &mut vp_paging_mode);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(vp_paging_mode),
            error => Err(error),
        }
    }

    /// Reads a sized guest physical address into the supplied buffer.
    pub fn read_guest_physical_address(
        &self,
        physical_address: GuestPhysicalAddress,
        buffer: &mut [u8],
    ) -> Result<u32, ResultCode> {
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

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(bytes_read),
            error => Err(error),
        }
    }

    /// Translates a virtual address to a physical address using information found in the
    /// guest's memory and processor's state.
    pub fn guest_virtual_to_physical_address(
        &self,
        vp_id: u32,
        virtual_address: GuestVirtualAddress,
    ) -> Result<GuestPhysicalAddress, ResultCode> {
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

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(physical_address),
            error => Err(error),
        }
    }

    /// Returns a tuple with the page size and the layout of the physical memory of the guest.
    pub fn guest_physical_memory_chunks(&self) -> Result<(u64, Vec<GpaMemoryChunk>), ResultCode> {
        let mut memory_chunks: Vec<GpaMemoryChunk> = vec![];
        let mut page_size: u64 = 0;
        let mut chunk_count: u64 = 0;
        let mut result: HResult;

        // First figure out memory chunks vector size
        unsafe {
            result = GetGuestPhysicalMemoryChunks(
                self.handle.clone(),
                &mut page_size,
                std::ptr::null_mut(),
                &mut chunk_count,
            );

            result = match hresult_to_result_code(&result) {
                ResultCode::OutOfMemory(_) => {
                    // Allocate enough memory in the vector to fit the memory chunks
                    for _ in 0..chunk_count {
                        memory_chunks.push(GpaMemoryChunk {
                            guest_physical_start_page_index: 0,
                            page_count: 0,
                        })
                    }

                    // Actually get the chunks
                    GetGuestPhysicalMemoryChunks(
                        self.handle.clone(),
                        &mut page_size,
                        memory_chunks.as_mut_ptr(),
                        &mut chunk_count,
                    )
                }
                error => return Err(error), // Any other result here is unexpected
            }
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok((page_size, memory_chunks)),
            error => Err(error),
        }
    }

    /// Translates the given guest physical address to a raw saved memory offset.
    pub fn guest_physical_address_to_raw_saved_memory_offset(
        &self,
        physical_address: GuestPhysicalAddress,
    ) -> Result<u64, ResultCode> {
        let mut raw_saved_memory_offset: u64 = 0;
        let result: HResult;

        unsafe {
            result = GuestPhysicalAddressToRawSavedMemoryOffset(
                self.handle.clone(),
                physical_address,
                &mut raw_saved_memory_offset,
            );
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(raw_saved_memory_offset),
            error => Err(error),
        }
    }

    /// Reads raw memory from the saved state file. This function reads raw memory from the saved state file
    /// as if it were a flat memory layout, regardless of the guest memory layout.
    pub fn read_guest_raw_saved_memory(
        &self,
        offset: u64,
        buffer: &mut [u8],
    ) -> Result<u32, ResultCode> {
        let buffer_size = buffer.len() as u32;
        let buffer_ptr = buffer.as_mut_ptr();
        let mut bytes_read: u32 = 0;
        let result: HResult;

        unsafe {
            result = ReadGuestRawSavedMemory(
                self.handle.clone(),
                offset,
                buffer_ptr as PVoid,
                buffer_size,
                &mut bytes_read,
            );
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(bytes_read),
            error => Err(error),
        }
    }

    /// Returns the size in bytes of the saved memory for a given VM saved state file.
    pub fn guest_raw_saved_memory_size(&self) -> Result<u64, ResultCode> {
        let mut raw_memory_size: u64 = 0;
        let result: HResult;

        unsafe {
            result = GetGuestRawSavedMemorySize(self.handle.clone(), &mut raw_memory_size);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success(_) => Ok(raw_memory_size),
            error => Err(error),
        }
    }
}
