//! This module implements safe wrappers of the unsafe API surface to VmSavedStateDump.
//! Defines and provides Rust idiomatic abstractions of the API.

use crate::vmsavedstatedumpdefs::*;
use crate::vmsavedstatedump_raw_bindings::*;
use crate::windefs::*;

use std::ops;
use kernel32::LocalFree;
use widestring::U16CString;
use widestring::WideCString;

/// Common result codes that can be returned by the VmSavedStateDumpProvider API.
#[derive(Debug, PartialEq)]
pub enum ResultCode {
    Success,
    OutOfMemory,
    FileNotFound,
    Fail,
    InvalidArgument,
    Unexpected,
    WindowsHResult(HResult),
}

#[allow(overflowing_literals)]
fn hresult_to_result_code(hresult: &HResult) -> ResultCode {
    match hresult {
        0 => ResultCode::Success,
        0x8007000E => ResultCode::OutOfMemory,
        0x80070002 => ResultCode::FileNotFound,
        0x80004005 => ResultCode::Fail,
        0x80070057 => ResultCode::InvalidArgument,
        0x8000FFFF => ResultCode::Unexpected,
        other => ResultCode::WindowsHResult(other.clone()),
    }
}

/// Enum that represents all possible ways a VM Saved state file can be stored
#[derive(Debug, PartialEq)]
pub enum VmSavedStateFile {
    BinVsv(String, String),
    Vmrs(String),
}

/// Locates the saved state file(s) for a given VM and/or snapshot. This function uses WMI and the V1 or V2
/// virtualization namespace. So this is expected to fail if ran on a machine without Hyper-V installed.
pub fn locate_saved_state_files(vm_name: &str, snapshot_name: &str) -> Result<VmSavedStateFile, ResultCode> {
    let widestr_bin_file_path: WideCString;
    let widestr_vsv_file_path: WideCString;
    let widestr_vmrs_file_path: WideCString;
    let result: HResult;

    unsafe {
        let mut bin_file_path_buffer: LPWStr = std::ptr::null_mut();
        let mut vsv_file_path_buffer: LPWStr = std::ptr::null_mut();
        let mut vmrs_file_path_buffer: LPWStr = std::ptr::null_mut();

        result = LocateSavedStateFiles(
            U16CString::from_str(vm_name).unwrap().as_ptr(),
            U16CString::from_str(snapshot_name).unwrap().as_ptr(),
            &mut bin_file_path_buffer as *mut LPWStr,
            &mut vsv_file_path_buffer as *mut LPWStr,
            &mut vmrs_file_path_buffer as *mut LPWStr,
        );

        widestr_bin_file_path = WideCString::from_ptr_str(bin_file_path_buffer);
        LocalFree(bin_file_path_buffer as PVoid);

        widestr_vsv_file_path = WideCString::from_ptr_str(vsv_file_path_buffer);
        LocalFree(vsv_file_path_buffer as PVoid);

        widestr_vmrs_file_path = WideCString::from_ptr_str(vmrs_file_path_buffer);
        LocalFree(vmrs_file_path_buffer as PVoid);
    }

    let bin_file_path = widestr_bin_file_path.to_string_lossy();
    let vsv_file_path = widestr_vsv_file_path.to_string_lossy();
    let vmrs_file_path = widestr_vmrs_file_path.to_string_lossy();

    match hresult_to_result_code(&result) {
        ResultCode::Success => {
            if vmrs_file_path.is_empty() {
                if bin_file_path.is_empty() || vsv_file_path.is_empty() {
                    Err(ResultCode::FileNotFound)
                } else {
                    Ok(VmSavedStateFile::BinVsv(bin_file_path, vsv_file_path))
                }
            } else {
                Ok(VmSavedStateFile::Vmrs(vmrs_file_path))
            }
        },
        error => Err(error),
    }
}

/// Applies a pending replay log to a VMRS file.
pub fn apply_pending_replay_log(vmrs: &str) -> Result<(), ResultCode> {
    let result: HResult;

    unsafe {
        result = ApplyPendingSavedStateFileReplayLog(U16CString::from_str(vmrs).unwrap().as_ptr())
    }

    match hresult_to_result_code(&result) {
        ResultCode::Success => Ok(()),
        error => Err(error),
    }
}

/// Structure that abstracts access to a loaded VM Saved state file and its dump related APIs.
#[derive(Debug)]
pub struct VmSavedStateDumpProvider {
    handle: VmSavedStateDumpHandle,
}

impl ops::Drop for VmSavedStateDumpProvider {
    fn drop(&mut self) {
        unsafe {
            if hresult_to_result_code(&ReleaseSavedStateFiles(self.handle)) != ResultCode::Success {
                panic!("Failed to release saved state files");
            }
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
            ResultCode::Success => Ok(VmSavedStateDumpProvider {
                handle: dump_handle,
            }),
            error => Err(error),
        }
    }

    /// Loads a VMRS VM Saved state file and returns a VmSavedStateDumpProvider instance
    /// that provides the interface to the dump related APIs.
    pub fn load_vmrs(vmrs: &str) -> Result<VmSavedStateDumpProvider, ResultCode> {
        let mut dump_handle: VmSavedStateDumpHandle = std::ptr::null_mut();
        let result: HResult;

        unsafe {
            result = LoadSavedStateFile(
                U16CString::from_str(vmrs).unwrap().as_ptr(),
                &mut dump_handle,
            );
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(VmSavedStateDumpProvider {
                handle: dump_handle,
            }),
            error => Err(error),
        }
    }

    /// Returns the virtual processor count.
    pub fn vp_count(&self) -> Result<u32, ResultCode> {
        let mut vp_count = 0;
        let result: HResult;

        unsafe {
            result = GetVpCount(self.handle, &mut vp_count);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(vp_count),
            error => Err(error),
        }
    }

    /// Returns an iterator to virtual processors associated to this saved state file.
    pub fn vp_iter(&self) -> VirtualProcessorIter {
        VirtualProcessorIter {
            provider: &self,
            current_id: 0,
            count: self.vp_count().unwrap(),
        }
    }

    /// Returns the virtual processor architecture.
    pub fn get_vp_architecture(&self, vp_id: u32) -> Result<VirtualProcessorArch, ResultCode> {
        let mut vp_arch = VirtualProcessorArch::Unknown;
        let result: HResult;

        unsafe {
            result = GetArchitecture(self.handle, vp_id, &mut vp_arch);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(vp_arch),
            error => Err(error),
        }
    }

    /// Returns a virtual processor register value.
    pub fn get_vp_register_value(
        &self,
        vp_id: u32,
        arch: VirtualProcessorArch,
        register_id: RegisterRawId,
    ) -> Result<VirtualProcessorRegister, ResultCode> {
        let mut vp_register_value: VirtualProcessorRegister = VirtualProcessorRegister {
            architecture: arch,
            value: 0,
            raw_id: register_id,
        };
        let result: HResult;

        unsafe {
            result = GetRegisterValue(self.handle, vp_id, &mut vp_register_value);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(vp_register_value),
            error => Err(error),
        }
    }

    /// Returns a virtual processor paging mode.
    pub fn get_vp_paging_mode(&self, vp_id: u32) -> Result<PagingMode, ResultCode> {
        let mut vp_paging_mode = PagingMode::Invalid;
        let result: HResult;

        unsafe {
            result = GetPagingMode(self.handle, vp_id, &mut vp_paging_mode);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(vp_paging_mode),
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
                self.handle,
                physical_address,
                buffer_ptr as PVoid,
                buffer_size,
                &mut bytes_read,
            );
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(bytes_read),
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
                self.handle,
                vp_id,
                virtual_address,
                &mut physical_address,
            );
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(physical_address),
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
                self.handle,
                &mut page_size,
                std::ptr::null_mut(),
                &mut chunk_count,
            );

            result = match hresult_to_result_code(&result) {
                ResultCode::OutOfMemory => {
                    // Allocate enough memory in the vector to fit the memory chunks
                    memory_chunks.resize(
                        chunk_count as usize,
                        GpaMemoryChunk {
                            guest_physical_start_page_index: 0,
                            page_count: 0,
                        },
                    );

                    // Actually get the chunks
                    GetGuestPhysicalMemoryChunks(
                        self.handle,
                        &mut page_size,
                        memory_chunks.as_mut_ptr(),
                        &mut chunk_count,
                    )
                }
                error => return Err(error), // Any other result here is unexpected
            }
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok((page_size, memory_chunks)),
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
                self.handle,
                physical_address,
                &mut raw_saved_memory_offset,
            );
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(raw_saved_memory_offset),
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
                self.handle,
                offset,
                buffer_ptr as PVoid,
                buffer_size,
                &mut bytes_read,
            );
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(bytes_read),
            error => Err(error),
        }
    }

    /// Returns the size in bytes of the saved memory for a given VM saved state file.
    pub fn guest_raw_saved_memory_size(&self) -> Result<u64, ResultCode> {
        let mut raw_memory_size: u64 = 0;
        let result: HResult;

        unsafe {
            result = GetGuestRawSavedMemorySize(self.handle, &mut raw_memory_size);
        }

        match hresult_to_result_code(&result) {
            ResultCode::Success => Ok(raw_memory_size),
            error => Err(error),
        }
    }
}

/// Represents a virtual processor of a VmSavedStateDumpProvider
/// and exposes simpler APIs that work with the VP it represents.
#[derive(Debug)]
pub struct VirtualProcessor<'a> {
    provider: &'a VmSavedStateDumpProvider,
    id: u32,
}

/// Virtual processor iterator that enumerates all valid virtual processors
/// for a given VmSavedStateDumpProvider.
#[derive(Debug)]
pub struct VirtualProcessorIter<'a> {
    provider: &'a VmSavedStateDumpProvider,
    current_id: u32,
    count: u32,
}

impl<'a> Iterator for VirtualProcessorIter<'a> {
    type Item = VirtualProcessor<'a>;

    fn next(&mut self) -> Option<VirtualProcessor<'a>> {
        let vp_id = self.current_id;
        self.current_id += 1;

        if vp_id < self.count {
            Some(VirtualProcessor {
                provider: &self.provider,
                id: vp_id,
            })
        } else {
            None
        }
    }
}

impl<'a> VirtualProcessor<'a> {
    /// Returns the id of a virtual processor.
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Returns the architecture of a given virtual processor.
    pub fn architecture(&self) -> Result<VirtualProcessorArch, ResultCode> {
        self.provider.get_vp_architecture(self.id)
    }

    /// Returns the register value of a given virtual processor.
    pub fn register_value(
        &self,
        arch: VirtualProcessorArch,
        register_id: RegisterRawId,
    ) -> Result<VirtualProcessorRegister, ResultCode> {
        self.provider
            .get_vp_register_value(self.id, arch, register_id)
    }

    /// Returns the paging mode of a given virtual processor.
    pub fn paging_mode(&self) -> Result<PagingMode, ResultCode> {
        self.provider.get_vp_paging_mode(self.id)
    }
}
