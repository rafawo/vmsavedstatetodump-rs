use std::path::{ Path, PathBuf };
use vmsavedstatedump_rs::vmsavedstatedumpprovider::*;

fn get_test_file_path() -> String {
    let mut cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cargo_manifest_dir.push("tests\\test_file.vmrs");

    let test_file_path = Path::new(&cargo_manifest_dir);
    assert!(test_file_path.exists());
    println!("Test file path: {}", test_file_path.to_str().unwrap());
    String::from(test_file_path.to_str().unwrap())
}

#[test]
fn vmrs_file_can_be_opened() {
    let file_path = get_test_file_path();
    let _provider = VmSavedStateDumpProvider::load_vmrs(&file_path);
}