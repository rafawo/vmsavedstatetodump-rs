//! There is a bug in vmsavedstatedumpprovider.dll that prevents multiple
//! saved state files from being loaded in sequence, reusing the same handle.
//!
//! A fix has been made, but won't be in until the next official release of the windows 10 SDK.
//! As a workaround, a private fix can be found in https://1drv.ms/u/s!ArJxuNplQduwr8V4AuAELz2KE6SLvQ
//!

use std::path::{Path, PathBuf};
use vmsavedstatedump_rs::vmsavedstatedump::*;
use vmsavedstatedump_rs::vmsavedstatedumpdefs::*;

fn get_test_bin_vsv_file_paths() -> (String, String) {
    let mut bin_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bin_file_path.push("tests\\test_file.bin");
    assert!(Path::new(&bin_file_path).exists());

    let mut vsv_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vsv_file_path.push("tests\\test_file.vsv");
    assert!(Path::new(&vsv_file_path).exists());

    (
        String::from(bin_file_path.to_str().unwrap()),
        String::from(vsv_file_path.to_str().unwrap()),
    )
}

fn get_test_vmrs_file_path() -> String {
    let mut vmrs_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vmrs_file_path.push("tests\\test_file.vmrs");
    assert!(Path::new(&vmrs_file_path).exists());
    String::from(vmrs_file_path.to_str().unwrap())
}

fn try_get_bin_vsv_test_provider() -> Result<VmSavedStateDumpProvider, ResultCode> {
    let file_paths = get_test_bin_vsv_file_paths();
    VmSavedStateDumpProvider::load_bin_vsv(&file_paths.0, &file_paths.1)
}

fn get_bin_vsv_test_provider() -> VmSavedStateDumpProvider {
    try_get_bin_vsv_test_provider().unwrap()
}

fn try_get_vmrs_test_provider() -> Result<VmSavedStateDumpProvider, ResultCode> {
    let file_path = get_test_vmrs_file_path();
    VmSavedStateDumpProvider::load_vmrs(&file_path)
}

fn get_vmrs_test_provider() -> VmSavedStateDumpProvider {
    try_get_vmrs_test_provider().unwrap()
}

#[test]
fn bin_vsv_can_be_loaded() {
    let provider = try_get_bin_vsv_test_provider();
    assert!(provider.is_ok());
}

#[test]
fn wrong_path_bin_vsv_cant_be_loaded() {
    let provider =
        VmSavedStateDumpProvider::load_bin_vsv("some_wrong_path.bin", "some_wrong_path.vsv");
    assert!(provider.is_err());
    assert_eq!(ResultCode::FileNotFound, provider.unwrap_err());
}

#[test]
fn vmrs_file_can_be_loaded() {
    let provider = try_get_vmrs_test_provider();
    assert!(provider.is_ok());
}

#[test]
fn wrong_path_vmrs_file_cant_be_loaded() {
    let provider = VmSavedStateDumpProvider::load_vmrs("some_wrong_path.vmrs");
    assert!(provider.is_err());
    assert_eq!(ResultCode::FileNotFound, provider.unwrap_err());
}

fn validate_vp_count(provider: &VmSavedStateDumpProvider) {
    let vp_count = provider.vp_count();
    assert_eq!(4, vp_count.unwrap());
}

#[test]
fn bin_vsv_get_vp_count() {
    let provider = get_bin_vsv_test_provider();
    validate_vp_count(&provider);
}

#[test]
fn vmrs_get_vp_count() {
    let provider = get_vmrs_test_provider();
    validate_vp_count(&provider);
}

fn validate_get_architecture(provider: &VmSavedStateDumpProvider) {
    let architecture = provider.get_vp_architecture(0);
    assert_eq!(VirtualProcessorArch::X86, architecture.unwrap());
}

#[test]
fn bin_vsv_get_architecture() {
    let provider = get_bin_vsv_test_provider();
    validate_get_architecture(&provider);
}

#[test]
fn vmrs_get_architecture() {
    let provider = get_vmrs_test_provider();
    validate_get_architecture(&provider);
}

fn validate_get_register_value(provider: &VmSavedStateDumpProvider) {
    let register_id = RegisterRawId {
        register_id_x86: RegisterIdx86::Ecx,
    };

    let register = provider.get_vp_register_value(0, VirtualProcessorArch::X86, register_id);
    assert_eq!(4, register.unwrap().value);
}

#[test]
fn bin_vsv_get_register_value() {
    let provider = get_bin_vsv_test_provider();
    validate_get_register_value(&provider);
}

#[test]
fn vmrs_get_register_value() {
    let provider = get_vmrs_test_provider();
    validate_get_register_value(&provider);
}

fn validate_get_paging_mode(provider: &VmSavedStateDumpProvider) {
    let paging_mode = provider.get_vp_paging_mode(0);
    assert_eq!(PagingMode::Bit32, paging_mode.unwrap());
}

#[test]
fn bin_vsv_get_paging_mode() {
    let provider = get_bin_vsv_test_provider();
    validate_get_paging_mode(&provider);
}

#[test]
fn vmrs_get_paging_mode() {
    let provider = get_vmrs_test_provider();
    validate_get_paging_mode(&provider);
}

#[test]
fn vp_iterator() {
    let provider = get_vmrs_test_provider();
    let vp_iter = provider.vp_iter();
    let mut vp_id = 0;
    let register_id = RegisterRawId {
        register_id_x86: RegisterIdx86::Ecx,
    };

    assert_eq!(4, provider.vp_count().unwrap());

    for vp in vp_iter {
        assert_eq!(vp_id, vp.id());
        println!("Iterating in vp {}", vp_id);
        assert_eq!(VirtualProcessorArch::X86, vp.architecture().unwrap());
        assert_eq!(PagingMode::Bit32, vp.paging_mode().unwrap());

        // Each vp has its own register value
        let register_value = match vp_id {
            0 => 4,
            _ => 0,
        };

        assert_eq!(
            register_value,
            vp.register_value(VirtualProcessorArch::X86, register_id)
                .unwrap()
                .value
        );
        vp_id += 1;
    }

    assert_eq!(4, vp_id);
}

#[test]
fn guest_physical_memory_chunks() {
    let provider = get_vmrs_test_provider();
    let (page_size, memory_chunks) = provider.guest_physical_memory_chunks().unwrap();

    assert_eq!(4096, page_size);
    assert_eq!(1, memory_chunks.len());

    assert_eq!(
        GpaMemoryChunk {
            guest_physical_start_page_index: 0,
            page_count: 64512,
        },
        memory_chunks[0]
    );
}

#[test]
fn guest_raw_saved_memory() {
    let provider = get_vmrs_test_provider();
    let raw_memory_size = provider.guest_raw_saved_memory_size().unwrap();
    assert_eq!(264241152, raw_memory_size);

    let mut buffer: Vec<u8> = Vec::new();
    buffer.resize(1024 * 1024, 0);

    let mut offset: u64 = 0;
    let mut bytes_read = provider
        .read_guest_raw_saved_memory(offset, buffer.as_mut_slice())
        .unwrap() as u64;
    offset += bytes_read;

    while bytes_read == 1024 * 1024 {
        bytes_read = provider
            .read_guest_raw_saved_memory(offset, buffer.as_mut_slice())
            .unwrap() as u64;
        offset += bytes_read;
    }

    assert_eq!(raw_memory_size, offset);

    // Because the test file has only one memory chunk sized, the translation
    // from physical address to raw saved memory offset corresponds to the same value
    assert_eq!(
        0xC0FFE,
        provider
            .guest_physical_address_to_raw_saved_memory_offset(0xC0FFE)
            .unwrap()
    );
}

#[test]
fn read_physical_address() {
    let provider = get_vmrs_test_provider();

    // Because the test file comes from a VM without a guest,
    // virtual to physical address translation can't really happen
    // because the physical memory is missing basic page tables.
    // So we at least verify a translation fails.
    assert!(provider
        .guest_virtual_to_physical_address(0, 0xC0FFE)
        .is_err());

    #[repr(C)]
    #[derive(Debug, PartialEq)]
    struct FakeStruct {
        x: u32,
        y: u32,
        z: u32,
    }

    let mut fake_struct: FakeStruct = FakeStruct { x: 0, y: 0, z: 0 };
    const FAKE_STRUCT_SIZE: usize = std::mem::size_of::<FakeStruct>();
    let slice = unsafe {
        std::mem::transmute::<&mut FakeStruct, &mut [u8; FAKE_STRUCT_SIZE]>(&mut fake_struct)
    };

    assert_eq!(slice.len(), FAKE_STRUCT_SIZE);
    let bytes_read = provider
        .read_guest_physical_address(0xC0FFE, slice)
        .unwrap();

    assert_eq!(FAKE_STRUCT_SIZE as u32, bytes_read);
    assert_eq!(
        FakeStruct {
            x: 75826887,
            y: 1235222542,
            z: 3439375364
        },
        fake_struct
    );
}

#[test]
fn apply_replay_log() {
    // This test case needs to run with its own test file
    // because the apply_pending_replay_log API opens the file exclusively,
    // and test_file.vmrs is shared amongst all other test cases running in parallel
    let mut vmrs_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vmrs_file_path.push("tests\\test_file.apply_pending_replay_log.vmrs");
    assert!(Path::new(&vmrs_file_path).exists());
    assert_eq!(
        Ok(()),
        apply_pending_replay_log(vmrs_file_path.to_str().unwrap())
    );
}
