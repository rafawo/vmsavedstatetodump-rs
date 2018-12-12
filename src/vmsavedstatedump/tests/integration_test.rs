use std::path::{Path, PathBuf};
use vmsavedstatedump_rs::vmsavedstatedumpprovider::*;

fn get_test_bin_vsv_file_paths() -> (String, String) {
    let mut bin_file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bin_file_path.push("tests\\test_file.vsv");
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

#[test]
fn bin_vsv_can_be_loaded() {
    let file_paths = get_test_bin_vsv_file_paths();
    let provider = VmSavedStateDumpProvider::load_bin_vsv(&file_paths.0, &file_paths.1);
    assert!(provider.is_ok());
}

#[test]
fn vmrs_file_can_be_loaded() {
    let file_path = get_test_vmrs_file_path();
    let provider = VmSavedStateDumpProvider::load_vmrs(&file_path);
    assert!(provider.is_ok());
}
