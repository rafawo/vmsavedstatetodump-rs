pub(crate) mod vmsavedstatedump_raw_bindings;
pub mod vmsavedstatedumpdefs;
pub mod vmsavedstatedump;

pub(crate) mod windefs {
    use libc;

    pub type DWord = libc::c_ulong;
    pub type HResult = libc::c_long;
    pub type LPCWStr = *const libc::wchar_t;
    pub type LPWStr = *mut libc::wchar_t;
    pub type PVoid = *mut Void;
    pub type Void = libc::c_void;
}
