# vmsavedstatetodump-rs
Rust wrapper of VmSavedStateDump

## Overview

This project provides a Rust abstraction of [VmSavedStateDumpProvider](https://docs.microsoft.com/en-us/virtualization/api/vm-dump-provider/vm-dump-provider) APIs.

VmSavedStateDumpProvider APIs are part of the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk).

## Requirements

For this wrapper to build properly, the following requirements need to be met by the building machine:

- Windows 10 SDK version **10.0.18362.0**.
- **amd64** architecture.

## Wrapped Windows 10 SDK APIs

**_Note: The file paths are based on the default installation path `c:\Program Files (x86)\Windows Kits\10`._**

The relevant Windows 10 SDK files that this project is wrapping are:
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\vmsavedstatedumpdefs.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.18362.0\um\vmsavedstatedump.h
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.18362.0\um\x64\vmsavedstatedumpprovider.lib
- C:\Program Files (x86)\Windows Kits\10\bin\10.0.18362.0\x64\vmsavedstatedumpprovider.dll

## Usage

To be able to run code that consumes these APIs, vmsavedstatedumpprovider.dll must be discoverable
from within the executables/binary runtime.

When using these APIs, the main entry point is to create a VmSavedStateDumpProvider
by supplying path(s) to bin/vsv - vmrs VM saved state file(s).

```
let vmrs_provider = VmSavedStateDumpProvider::load_vmrs("file_path.vmrs");
let bin_vsv_provider = VmSavedStateDumpProvider::load_bin_vsv("file_path.bin", "file_path.vsv");
```

Once a provider has been instantiated, all of its related APIs can be used in the context
of a loaded VM saved state file.

The best source of code examples on how to use the APIs are the integration tests,
found [here](https://github.com/rafawo/vmsavedstatetodump-rs/blob/master/vmsavedstatedump-rs/tests/integration_test.rs).

## How to use locally

Clone the repo to a folder:

```
git clone https://github.com/rafawo/vmsavedstatetodump-rs.git
```

Make sure the machine where you are building has Windows 10 SDK version Windows 10 SDK version **10.0.18362.0** installed. Then run:

```
cd vmsavedstatetodump-rs
cargo build
```

Open documentation by running:
```
cargo doc --open
```

Finally, the build process should have copied `vmsavedstatedumpprovider.dll` to the root directory. Run tests by running:
```
cargo test
```

> **NOTE:There is a bug in vmsavedstatedumpprovider.dll that prevents multiple
saved state files from being loaded in sequence, reusing the same handle.
The fix is available starting at Windows 10 SDK version 10.0.18362.0**

## Crates.io version notes

This section briefly describes notes regarding each published crates.io version of this project.

Ordered from latest to oldest.

- [**0.2.0 Dec 20, 2018**](https://crates.io/crates/vmsavedstatedump_rs/0.2.0)
  - Updated default Windows 10 SDK version to 10.0.18362.0
    - This removes the need of using a private vmsavedstatedumpprovider.dll and the one in the SDK contains the fix to the multiple opened files at the same time on a single loaded DLL module.
- [**0.1.3 Dec 20, 2018**](https://crates.io/crates/vmsavedstatedump_rs/0.1.3)
  - Cleaned up types and error handling.
  - Internal refactoring and added type aliases for windows types.
  - *****NOT A RECOMMENDED VERSION TO USE. IT MIGHT BE YANKED IN THE FUTURE*****
    - This is the last crate version with a hardcoded dependency to Windows 10 SDK 10.0.17763.0, which unfortunately has the broken vmsavedstatedumpprovider.dll version.
- [**0.1.2 Dec 20, 2018**](https://crates.io/crates/vmsavedstatedump_rs/0.1.2)
  - Completed the core implementation of the library
  - Added integration tests.
  - Added note to README regarding the broken version of vmsavedstatedumpprovider.dll, and a link to a private fix.
  - *****NOT A RECOMMENDED VERSION TO USE. IT MIGHT BE YANKED IN THE FUTURE*****
- [**0.1.1 Dec 14, 2018**](https://crates.io/crates/vmsavedstatedump_rs/0.1.1)
  - Minor bug fixes but still missing core functionality.
  - *****NOT A RECOMMENDED VERSION TO USE. IT MIGHT BE YANKED IN THE FUTURE*****
- [**0.1.0 Dec 14, 2018**](https://crates.io/crates/vmsavedstatedump_rs/0.1.0)
  - First version released for this crate.
  - Contains the basic bindings and rust abstractions to the API but still has a lot of bugs.
  - *****NOT A RECOMMENDED VERSION TO USE. IT MIGHT BE YANKED IN THE FUTURE*****
