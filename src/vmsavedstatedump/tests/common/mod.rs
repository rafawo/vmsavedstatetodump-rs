use std::env::var;
use std::path::{ Path, PathBuf };
use std::fs;

pub fn deploy_dll() {
    let root_win10_sdk_path = match var("WIN10SDK_PATH") {
        Ok(path) => path,
        Err(_) => String::from("c:\\Program Files (x86)\\Windows Kits\\10"),
    };

    let win10_sdk_version = match var("WIN10SDK_VERSION") {
        Ok(path) => path,
        Err(_) => String::from("10.0.17763.0"),
    };

    let dll_path = format!(
        "{}\\bin\\{}\\x64\\vmsavedstatedumpprovider.dll",
        root_win10_sdk_path, win10_sdk_version
    );

    println!("DLL path: {}", &dll_path);
    assert!(Path::new(&dll_path).exists());

    let mut destination = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    destination.push("vmsavedstatedumpprovider.dll");
    let destination = Path::new(&destination);

    if !destination.exists() {
        fs::copy(&dll_path, &destination).unwrap();
        println!("Copied to: {}", destination.to_str().unwrap());
    } else {
        println!("DLL already exists in destination: {}", destination.to_str().unwrap());
    }
}

pub fn get_test_file_path() -> String {
    let mut cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    cargo_manifest_dir.push("tests\\test_file.vmrs");

    let test_file_path = Path::new(&cargo_manifest_dir);
    assert!(test_file_path.exists());
    println!("Test file path: {}", test_file_path.to_str().unwrap());
    String::from(test_file_path.to_str().unwrap())
}