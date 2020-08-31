// Copyright (c) 2019 Rafael Alcaraz Mercado. All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.


//! This module provides a Rust equivalent of Windows SDK's `mindumpdef.h` header file.

pub const DMP_PHYSICAL_MEMORY_BLOCK_SIZE_32: usize = 700;
pub const DMP_CONTEXT_RECORD_SIZE_32: usize = 1200;
pub const DMP_RESERVED_0_SIZE_32: usize = 1760;
pub const DMP_RESERVED_2_SIZE_32: usize = 16;
pub const DMP_RESERVED_3_SIZE_32: usize = 56;

pub const DMP_PHYSICAL_MEMORY_BLOCK_SIZE_64: usize = 700;
pub const DMP_CONTEXT_RECORD_SIZE_64: usize = 3000;
pub const DMP_RESERVED_0_SIZE_64: usize = 4008;

pub const DMP_HEADER_COMMENT_SIZE: usize = 128;
#[repr(i8)]
pub enum DumpTypes {
    Invalid = -1,
    Unknown = 0,
    Full = 1,
    Summary = 2,
    Header = 3,
    Triage = 4,
    BitmapFull = 5,
    BitmapKernel = 6,
    Automatic = 7
}

pub const DUMP_SIGNATURE32: &[u8] = b"EGAP";
pub const DUMP_VALID_DUMP32: &[u8] = b"PMUD";

pub const DUMP_SIGNATURE64: &[u8] = b"EGAP";
pub const DUMP_VALID_DUMP64: &[u8] = b"46UD";

pub const DUMP_SUMMARY_SIGNATURE: &[u8] = b"PMDS";
pub const DUMP_SUMMARY_VALID: &[u8] = b"PMUD";

pub const DUMP_FULL_SIGNATURE: &[u8] = b"PMDF";
pub const DUMP_FULL_VALID: &[u8] = b"PMUD";

pub const DUMP_SUMMARY_VALID_KERNEL_VA: u32 = 1;
pub const DUMP_SUMMARY_VALID_CURRENT_USER_VA: u32 = 2;

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PhysicalMemoryRun32 {
    pub base_page: u32,
    pub page_count: u32,
}


#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PhysicalMemoryDescriptor32 {
    pub number_of_runs: u32,
    pub number_of_pages: u32,
    pub run: [PhysicalMemoryRun32; 1],
}


#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PhysicalMemoryRun64 {
    pub base_page: u64,
    pub page_count: u64,
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct PhysicalMemoryDescriptor64 {
    pub number_of_runs: u32,
    pub number_of_pages: u64,
    pub run: [PhysicalMemoryRun64; 1],
}


#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DumpFileAttributes {
    inner: u32,
}

impl DumpFileAttributes {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn attributes(&self) -> u32 {
        self.inner
    }

    pub fn set_attributes(&mut self, value: u32) {
        self.inner = value;
    }

    pub fn hiber_crash(&self) -> bool {
        (self.inner & 0x0000_0001) == 0x0000_0001
    }

    pub fn set_hiber_crash(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0001;
        } else {
            self.inner = self.inner & !0x0000_0001;
        }
    }

    pub fn dump_device_power_off(&self) -> bool {
        (self.inner & 0x0000_0002) == 0x0000_0002
    }

    pub fn set_dump_device_power_off(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0002;
        } else {
            self.inner = self.inner & !0x0000_0002;
        }
    }

    pub fn insufficient_dumpfile_size(&self) -> bool {
        (self.inner & 0x0000_0004) == 0x0000_0004
    }

    pub fn set_insufficient_dumpfile_size(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0004;
        } else {
            self.inner = self.inner & !0x0000_0004;
        }
    }

    pub fn kernel_generated_triage_dump(&self) -> bool {
        (self.inner & 0x0000_0008) == 0x0000_0008
    }

    pub fn set_kernel_generated_triage_dump(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0008;
        } else {
            self.inner = self.inner & !0x0000_0008;
        }
    }

    pub fn live_dump_generated_dump(&self) -> bool {
        (self.inner & 0x0000_0010) == 0x0000_0010
    }

    pub fn set_live_dump_generated_dump(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0010;
        } else {
            self.inner = self.inner & !0x0000_0010;
        }
    }

    pub fn dump_is_generated_offline(&self) -> bool {
        (self.inner & 0x0000_0020) == 0x0000_0020
    }

    pub fn set_dump_is_generated_offline(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0020;
        } else {
            self.inner = self.inner & !0x0000_0020;
        }
    }

    pub fn filter_dump_file(&self) -> bool {
        (self.inner & 0x0000_0040) == 0x0000_0040
    }

    pub fn set_filter_dump_file(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0040;
        } else {
            self.inner = self.inner & !0x0000_0040;
        }
    }

    pub fn early_boot_crash(&self) -> bool {
        (self.inner & 0x0000_0080) == 0x0000_0080
    }

    pub fn set_early_boot_crash(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0080;
        } else {
            self.inner = self.inner & !0x0000_0080;
        }
    }

    /// If `encrypted_dump_data` flag is set, it means Dump data (i.e. non-secureheader data) is encrypted, and Secure header is in use.
    pub fn encrypted_dump_data(&self) -> bool {
        (self.inner & 0x0000_0100) == 0x0000_0100
    }

    pub fn set_encrypted_dump_data(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0100;
        } else {
            self.inner = self.inner & !0x0000_0100;
        }
    }

    /// `decrypted_dump` flag would be set by dump decryption software to indicate that the dump was originally encrypted and
    /// current dump is obtained after decryption of the original dump data.
    pub fn decrypted_dump(&self) -> bool {
        (self.inner & 0x0000_0200) == 0x0000_0200
    }

    pub fn set_decrypted_dump(&mut self, value: bool) {
        if value {
            self.inner |= 0x0000_0200;
        } else {
            self.inner = self.inner & !0x0000_0200;
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DumpHeader32 {
    pub signature: u32,
    pub valid_dump: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub directory_table_base: u32,
    pub pfn_data_base: u32,
    pub ps_loaded_module_list: u32,
    pub ps_active_process_head: u32,
    pub machine_image_type: u32,
    pub number_processors: u32,
    pub bug_check_code: u32,
    pub bug_check_parameter1: u32,
    pub bug_check_parameter2: u32,
    pub bug_check_parameter3: u32,
    pub bug_check_parameter4: u32,
    pub version_user: [u8; 32],

    /// Present only for Win2k and better
    pub pae_enabled: u8,

    /// Present only for W2K3 SP1 and better
    pub kd_secondary_version: u8,

    pub spare2: [u8; 2],

    /// Present only for Win2k SP1 and better.
    pub kd_debugger_data_block: u32,

    pub physical_memory_block_buffer: [u8; DMP_PHYSICAL_MEMORY_BLOCK_SIZE_32],

    pub context_record: [u8; DMP_CONTEXT_RECORD_SIZE_32],

    pub exception: winapi::um::winnt::EXCEPTION_RECORD32,

    /// May not be present.
    pub comment: [u8; DMP_HEADER_COMMENT_SIZE],

    pub attributes: DumpFileAttributes,
    pub boot_id: u32,
    reserved0: [u8; DMP_RESERVED_0_SIZE_32],

    /// Present for Win2k and better.
    pub dump_type: u32,

    pub mini_dump_fields: u32,
    pub secondary_data_state: u32,
    pub product_type: u32,
    pub suite_mask: u32,
    pub writer_status: u32,


    /// Present for Win2k and better.
    pub required_dump_space: winapi::shared::ntdef::LARGE_INTEGER,

    reserved2: [u8; DMP_RESERVED_2_SIZE_32],

    /// Present only for Whistler and better.
    pub system_up_time: winapi::shared::ntdef::LARGE_INTEGER,

    /// Present only for Win2k and better.
    pub system_time: winapi::shared::ntdef::LARGE_INTEGER,

    reserved3: [u8; DMP_RESERVED_3_SIZE_32],
}

impl Default for DumpHeader32 {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<Self>() }
    }
}

impl DumpHeader32 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn physical_memory_block(&self) -> &PhysicalMemoryDescriptor32 {
        unsafe { &(*((&self.physical_memory_block_buffer as *const _) as *const PhysicalMemoryDescriptor32)) }
    }

    pub fn physical_memory_block_mut(&mut self) -> &mut PhysicalMemoryDescriptor32 {
        unsafe { &mut (*((&mut self.physical_memory_block_buffer as *mut _) as *mut PhysicalMemoryDescriptor32)) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DumpHeader64 {
    pub signature: u32,
    pub valid_dump: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub directory_table_base: u64,
    pub pfn_data_base: u64,
    pub ps_loaded_module_list: u64,
    pub ps_active_process_head: u64,
    pub machine_image_type: u32,
    pub number_processors: u32,
    pub bug_check_code: u32,
    pub bug_check_parameter1: u64,
    pub bug_check_parameter2: u64,
    pub bug_check_parameter3: u64,
    pub bug_check_parameter4: u64,
    pub version_user: [u8; 32],
    pub kd_debugger_data_block: u64,
    pub physical_memory_block_buffer: [u8; DMP_PHYSICAL_MEMORY_BLOCK_SIZE_64],
    pub context_record: [u8; DMP_CONTEXT_RECORD_SIZE_64],
    pub exception: winapi::um::winnt::EXCEPTION_RECORD64,
    pub dump_type: u32,
    pub required_dump_space: winapi::shared::ntdef::LARGE_INTEGER,
    pub system_time: winapi::shared::ntdef::LARGE_INTEGER,

    /// May not be present.
    pub comment: [u8; DMP_HEADER_COMMENT_SIZE],

    pub system_up_time: winapi::shared::ntdef::LARGE_INTEGER,
    pub mini_dump_fields: u32,
    pub secondary_data_state: u32,
    pub product_type: u32,
    pub suite_mask: u32,
    pub writer_status: u32,
    unused1: u8,

    /// Present only for W2K3 SP1 and better
    pub kd_secondary_version: u8,

    unused2: [u8; 2],
    pub attributes: DumpFileAttributes,
    pub boot_id: u32,
    reserved0: [u8; DMP_RESERVED_0_SIZE_64],
}

impl Default for DumpHeader64 {
    fn default() -> Self {
        unsafe { std::mem::zeroed::<Self>() }
    }
}

impl DumpHeader64 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn physical_memory_block(&self) -> &PhysicalMemoryDescriptor64 {
        unsafe { &(*((&self.physical_memory_block_buffer as *const _) as *const PhysicalMemoryDescriptor64)) }
    }

    pub fn physical_memory_block_mut(&mut self) -> &mut PhysicalMemoryDescriptor64 {
        unsafe { &mut (*((&mut self.physical_memory_block_buffer as *mut _) as *mut PhysicalMemoryDescriptor64)) }
    }
}
