use crate::vmsavedstatedumpdefs::*;
use crate::vmsavedstatedump;
use crate::windefs::*;

use std::ops;
use widestring::U16CString;

pub enum ErrorType {
    Success(HResult),
    OutOfMemory(HResult),
    FileNotFound(HResult),
    Fail(HResult),
    Unexpected(HResult),
    WindowsHResult(HResult),
}

fn hresult_to_error_code(hresult: &HResult) -> ErrorType {
    // TODO: fill in the other cases
    match hresult {
        0 => ErrorType::Success(0),
        other => ErrorType::WindowsHResult(other.clone()),
    }
}

pub fn apply_pending_replay_log(vmrs: &str) -> HResult {
    unsafe {
        vmsavedstatedump::ApplyPendingSavedStateFileReplayLog(U16CString::from_str(vmrs).unwrap().as_ptr())
    }
}

enum VmSavedStateFile {
    BinVsv(String, String),
    Vmrs(String),
}

pub struct VmSavedStateDumpProvider {
    handle: VmSavedStateDumpHandle,
    saved_state: VmSavedStateFile,
}

impl ops::Drop for VmSavedStateDumpProvider {
    fn drop(&mut self) {
        unsafe {
            // We ignore error code on purpose
            vmsavedstatedump::ReleaseSavedStateFiles(self.handle.clone());
        }
    }
}

impl VmSavedStateDumpProvider {
    pub fn load_saved_state_files(bin: &str, vsv: &str) -> Result<VmSavedStateDumpProvider, ErrorType> {
        let mut dump_handle: VmSavedStateDumpHandle = std::ptr::null_mut();
        let result: HResult;

        unsafe {
            result = vmsavedstatedump::LoadSavedStateFiles(
                U16CString::from_str(bin).unwrap().as_ptr(),
                U16CString::from_str(vsv).unwrap().as_ptr(),
                &mut dump_handle);
        }

        match hresult_to_error_code(&result) {
            ErrorType::Success(_) => Ok(VmSavedStateDumpProvider {
                handle: dump_handle,
                saved_state: VmSavedStateFile::BinVsv(String::from(bin), String::from(vsv)),
            }),
            error => Err(error),
        }
    }

    pub fn load_saved_state_file(vmrs: &str) -> Result<VmSavedStateDumpProvider, ErrorType> {
        let mut dump_handle: VmSavedStateDumpHandle = std::ptr::null_mut();
        let result: HResult;

        unsafe {
            result = vmsavedstatedump::LoadSavedStateFile(
                U16CString::from_str(vmrs).unwrap().as_ptr(),
                &mut dump_handle);
        }

        match hresult_to_error_code(&result) {
            ErrorType::Success(_) => Ok(VmSavedStateDumpProvider {
                handle: dump_handle,
                saved_state: VmSavedStateFile::Vmrs(String::from(vmrs)),
            }),
            error => Err(error),
        }
    }

    pub fn get_vp_count(&self) -> Result<u32, ErrorType> {
        let mut vp_count: u32 = 0;
        let result: HResult;

        unsafe {
            result = vmsavedstatedump::GetVpCount(self.handle.clone(), &mut vp_count);
        }

        match hresult_to_error_code(&result) {
            ErrorType::Success(_) => Ok(vp_count),
            error => Err(error),
        }
    }

    pub fn try_get_vp_count(&self) -> u32 {
        match self.get_vp_count() {
            Ok(vp_count) => vp_count,
            Err(_) => 0,
        }
    }
}
