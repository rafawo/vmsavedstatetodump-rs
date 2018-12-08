//! This file contains the VmSavedState Dump Provider definitions used by the APIs.

use crate::windefs::*;

pub type VmSavedStateDumpHandle = *mut Void;
pub type GuestVirtualAddress = u64;
pub type GuestPhysicalAddress = u64;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PagingMode {
    Invalid = 0,
    NonPaged,
    Bit32,
    Pae,
    Long,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GpaMemoryChunk {
    pub guest_physical_start_page_index: u64,
    pub page_count: u64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum VirtualProcessorArch {
    Unknown = 0,
    X86,
    X64,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RegisterIdx86 {
    //
    // General Purpose Registers
    //
    Eax = 0,
    Ecx,
    Edx,
    Ebx,
    Esp,
    Ebp,
    Esi,
    Edi,
    Eip,
    EFlags,

    //
    // Floating Point Registers
    //
    LowXmm0,
    HighXmm0,
    LowXmm1,
    HighXmm1,
    LowXmm2,
    HighXmm2,
    LowXmm3,
    HighXmm3,
    LowXmm4,
    HighXmm4,
    LowXmm5,
    HighXmm5,
    LowXmm6,
    HighXmm6,
    LowXmm7,
    HighXmm7,
    LowXmm8,
    HighXmm8,
    LowXmm9,
    HighXmm9,
    LowXmm10,
    HighXmm10,
    LowXmm11,
    HighXmm11,
    LowXmm12,
    HighXmm12,
    LowXmm13,
    HighXmm13,
    LowXmm14,
    HighXmm14,
    LowXmm15,
    HighXmm15,
    LowXmmControlStatus,
    HighXmmControlStatus,
    LowFpControlStatus,
    HighFpControlStatus,

    //
    // Control Registers
    //
    Cr0,
    Cr2,
    Cr3,
    Cr4,
    Cr8,
    Efer,

    //
    // Debug Registers
    //
    Dr0,
    Dr1,
    Dr2,
    Dr3,
    Dr6,
    Dr7,

    //
    // Segment Registers
    //
    BaseGs,
    BaseFs,
    SegCs,
    SegDs,
    SegEs,
    SegFs,
    SegGs,
    SegSs,
    Tr,
    Ldtr,

    //
    // Table Registers
    //
    BaseIdtr,
    LimitIdtr,
    BaseGdtr,
    LimitGdtr,

    //
    // Register Count
    //
    Count,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RegisterIdx64 {
    //
    // General Purpose Registers
    //
    Rax = 0,
    Rcx,
    Rdx,
    Rbx,
    Rsp,
    Rbp,
    Rsi,
    Rdi,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    Rip,
    RFlags,

    //
    // Floating Point Registers
    //
    LowXmm0,
    HighXmm0,
    LowXmm1,
    HighXmm1,
    LowXmm2,
    HighXmm2,
    LowXmm3,
    HighXmm3,
    LowXmm4,
    HighXmm4,
    LowXmm5,
    HighXmm5,
    LowXmm6,
    HighXmm6,
    LowXmm7,
    HighXmm7,
    LowXmm8,
    HighXmm8,
    LowXmm9,
    HighXmm9,
    LowXmm10,
    HighXmm10,
    LowXmm11,
    HighXmm11,
    LowXmm12,
    HighXmm12,
    LowXmm13,
    HighXmm13,
    LowXmm14,
    HighXmm14,
    LowXmm15,
    HighXmm15,
    LowXmmControlStatus,
    HighXmmControlStatus,
    LowFpControlStatus,
    HighFpControlStatus,

    //
    // Control Registers
    //
    Cr0,
    Cr2,
    Cr3,
    Cr4,
    Cr8,
    Efer,

    //
    // Debug Registers
    //
    Dr0,
    Dr1,
    Dr2,
    Dr3,
    Dr6,
    Dr7,

    //
    // Segment Registers
    //
    BaseGs,
    BaseFs,
    SegCs,
    SegDs,
    SegEs,
    SegFs,
    SegGs,
    SegSs,
    Tr,
    Ldtr,

    //
    // Table Registers
    //
    BaseIdtr,
    LimitIdtr,
    BaseGdtr,
    LimitGdtr,

    //
    // Register Count
    //
    Count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union RegisterRawId {
    pub register_id: DWord,
    pub register_id_x86: RegisterIdx86,
    pub register_id_x64: RegisterIdx64,
}

impl std::fmt::Debug for RegisterRawId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unsafe { write!(f, "RegisterRawId {{ register_id: {} }}", self.register_id) }
    }
}

/// This struct, when used in a call to GetRegisterValue, its Architecture and RegisterId fields
/// are inputs and the register value is an output.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VirtualProcessorRegister {
    pub architecture: VirtualProcessorArch,
    pub register_value: u64,
    pub raw_id: RegisterRawId,
}
