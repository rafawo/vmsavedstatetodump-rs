use crate::vmsavedstatedumpdefs::*;
use crate::windefs::*;

use std::ops;

pub enum ErrorCode {
    Success(HResult),
    OutOfMemory(HResult),
    FileNotFound(HResult),
    Fail(HResult),
    Unexpected(HResult),
    WindowsHResult(HResult),
}

fn hresult_to_error_code(hresult: &HResult) -> ErrorCode {
    // TODO: fill in the other cases
    match hresult {
        0 => ErrorCode::Success(0),
        other => ErrorCode::WindowsHResult(other.clone()),
    }
}

enum VmSavedStateFile {
    BinVsv(String, String),
    Vmrs(String),
}

pub struct VmSavedStateDumpProvider {
    handle: VmSavedStateDumpHandle,
    saved_state: VmSavedStateFile,
    virtual_processor_count: u32,
}

impl ops::Drop for VmSavedStateDumpProvider {
    fn drop(&mut self) {

    }
}
