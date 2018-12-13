//! There is a bug in vmsavedstatedumpprovider.dll that prevents multiple
//! saved state files from being loaded in sequence, reusing the same handle.
//!
//! A fix has been made, but won't be in until the next official release of the windows 10 SDK.
//! As a workaround, a private fix can be found in https://1drv.ms/u/s!ArJxuNplQduwr8V4AuAELz2KE6SLvQ
//!

use std::path::{Path, PathBuf};
use vmsavedstatedump_rs::vmsavedstatedumpdefs::*;
use vmsavedstatedump_rs::vmsavedstatedumpprovider::*;

fn get_test_bin_vsv_file_paths() -> (String, String) {
    let mut bin_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bin_file_path.push("tests\\test_file.bin");
    assert!(Path::new(&bin_file_path).exists());
    println!("Test file path: {}", bin_file_path.to_str().unwrap());

    let mut vsv_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vsv_file_path.push("tests\\test_file.vsv");
    assert!(Path::new(&vsv_file_path).exists());
    println!("Test file path: {}", vsv_file_path.to_str().unwrap());

    (
        String::from(bin_file_path.to_str().unwrap()),
        String::from(vsv_file_path.to_str().unwrap()),
    )
}

fn get_test_vmrs_file_path() -> String {
    let mut vmrs_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    vmrs_file_path.push("tests\\test_file.vmrs");
    assert!(Path::new(&vmrs_file_path).exists());
    println!("Test file path: {}", vmrs_file_path.to_str().unwrap());
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
        register_id_x64: RegisterIdx64::Cr3,
    };

    let register = provider.get_vp_register_value(0, VirtualProcessorArch::X86, register_id);
    assert_eq!(4294905840, register.unwrap().value);
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
