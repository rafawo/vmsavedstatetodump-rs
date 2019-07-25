// Copyright © rafawo (rafawo1@hotmail.com). All rights reserved.
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// All files in the project carrying such notice may not be copied, modified, or distributed
// except according to those terms.
// THE SOURCE CODE IS AVAILABLE UNDER THE ABOVE CHOSEN LICENSE "AS IS", WITH NO WARRANTIES.

//! Build script that takes care of making sure VmSavedStateDumpProvider.lib is linked
//! to the crate when built.
//!
//! This script relies on the environment variables `WIN10SDK_PATH` and `WIN10SDK_VERSION`.
//! `WIN10SDK_PATH` defaults to `c:\Program Files (x86)\Windows Kits\10` if not set.
//! `WIN10SDK_VERSION` defaults to `10.0.18362.0` if not set.

use std::env::var;
use std::fs;
use std::path::{Path, PathBuf};

pub fn deploy_dll() {
    let root_win10_sdk_path = match var("WIN10SDK_PATH") {
        Ok(path) => path,
        Err(_) => String::from("c:\\Program Files (x86)\\Windows Kits\\10"),
    };

    let win10_sdk_version = match var("WIN10SDK_VERSION") {
        Ok(path) => path,
        Err(_) => String::from("10.0.18362.0"),
    };

    let dll_path = format!(
        "{}\\bin\\{}\\x64\\vmsavedstatedumpprovider.dll",
        root_win10_sdk_path, win10_sdk_version
    );

    println!("cargo:vmsavedstatedump-rs-dll-path={}", &dll_path);
    assert!(Path::new(&dll_path).exists());

    let mut destination = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    destination.push("vmsavedstatedumpprovider.dll");
    let destination = Path::new(&destination);

    if !destination.exists() {
        fs::copy(&dll_path, &destination).unwrap();
        println!(
            "cargo:vmsavedstatedump-rs-dll-copied-to={}",
            destination.to_str().unwrap()
        );
    } else {
        println!(
            "cargo:vmsavedstatedump-rs-dll-already-exists-in-destination={}",
            destination.to_str().unwrap()
        );
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=WIN10SDK_PATH");

    let root_win10_sdk_path = match var("WIN10SDK_PATH") {
        Ok(path) => path,
        Err(_) => String::from("c:\\Program Files (x86)\\Windows Kits\\10"),
    };

    let win10_sdk_version = match var("WIN10SDK_VERSION") {
        Ok(path) => path,
        Err(_) => String::from("10.0.18362.0"),
    };

    let lib_names = vec![String::from("vmsavedstatedumpprovider")];

    let lib_root_path = format!(
        "{}\\Lib\\{}\\um\\x64",
        root_win10_sdk_path, win10_sdk_version
    );

    for lib_name in lib_names {
        println!("cargo:rustc-link-lib=dylib={}", lib_name);
    }

    println!("cargo:rustc-link-search={}", lib_root_path);

    deploy_dll();
}
