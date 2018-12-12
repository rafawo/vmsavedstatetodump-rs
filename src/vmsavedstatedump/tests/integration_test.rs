//use vmsavedstatedump_rs::vmsavedstatedumpprovider::*;

mod common;

#[test]
fn vmrs_file_can_be_opened() {
    common::deploy_dll();
    let _file_path = common::get_test_file_path();
    //let _provider = VmSavedStateDumpProvider::load_vmrs(&file_path);
}