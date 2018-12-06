#[allow(dead_code)]
pub mod vmsavedstatedumpdefs;

#[allow(dead_code)]
pub mod vmsavedstatedump;

mod windefs {
    use libc;

    pub type DWord = libc::c_ulong;
    pub type Void = libc::c_void;
    pub type HResult = libc::c_long;
    pub type LPCWStr = *const libc::wchar_t;
    pub type LPWStr = *mut libc::wchar_t;
    pub type PVoid = *mut Void;
}
