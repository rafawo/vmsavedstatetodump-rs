# vmsavedstatetodumprs
Rust wrapper of VmSavedStateDump

## Overview

This project provides a Rust abstraction of [VmSavedStateDumpProvider](https://docs.microsoft.com/en-us/virtualization/api/vm-dump-provider/vm-dump-provider) APIs.

VmSavedStateDumpProvider APIs are part of the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk).

## Requirements

For this wrapper to build properly, the following requirements need to be met by the building machine:

- Windows 10 SDK version **10.0.17763.132**.
- **amd64** architecture.

## Wrapped Windows 10 SDK APIs

**_Note: The file paths are based on the default installation path `c:\Program Files (x86)\Windows Kits\10`._**

The relevant Windows 10 SDK files that this project is wrapping are:
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\vmsavedstatedumpdefs.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\vmsavedstatedump.h
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\vmsavedstatedumpprovider.lib
- C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64\vmsavedstatedumpprovider.dll

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
found [here](https://github.com/rafawo/vmsavedstatetodump-rs/blob/master/src/vmsavedstatedump-rs/tests/integration_test.rs).
