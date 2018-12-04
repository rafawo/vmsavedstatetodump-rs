# vmsavedstatetodumprs
Rust wrapper of VmSavedStateDump

## Overview

This project is a collection of Rust libraries that wrap functionality exposed by [VmSavedStateDumpProvider](https://docs.microsoft.com/en-us/virtualization/api/vm-dump-provider/vm-dump-provider).

VmSavedStateDumpProvider APIs are part of the [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk).

## Requirements

For this wrapper to build properly, the following requirements need to be met by the building machine:

- Windows 10 SDK version **10.0.17763.132**.
- **amd64** architecture.

## Files in the SDK

**_Note: The file paths are based on the default installation path `c:\Program Files (x86)\Windows Kits\10`._**

The relevant Windows 10 SDK files that this project is wrapping are:
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\vmsavedstatedumpdefs.h
- C:\Program Files (x86)\Windows Kits\10\Include\10.0.17763.0\um\vmsavedstatedump.h
- C:\Program Files (x86)\Windows Kits\10\Lib\10.0.17763.0\um\x64\vmsavedstatedumpprovider.lib
- C:\Program Files (x86)\Windows Kits\10\bin\10.0.17763.0\x64\vmsavedstatedumpprovider.dll
