//! This file contains the interface definitions for the VmSavedState Dump Provider APIs.

use crate::vmsavedstatedumpdefs::*;
use crate::windefs::*;

#[link(name = "vmsavedstatedumpprovider")]
extern "C" {

    /// Locates the saved state file(s) for a given VM and/or snapshot. This function uses WMI and the V1 or V2
    /// virtualization namespace. So this is expected to fail if ran on a machine without Hyper-V installed.
    /// * If the given VM has a VMRS file, parameters BinPath and VsvPath will be a single null terminator character.
    /// * If the given VM has BIN and VSV files, parameter VmrsPath will be a single null terminator character.
    /// * If no saved state files are found, all three returned string parameters will be single null terminator characters.
    ///
    /// # Arguments
    ///
    /// * `VmName` - Supplies the VM name for which the saved state file will be located.
    /// * `SnapshotName` - Supplies an optional snapshot name to locate its saved state file
    ///                    on relation to the given VM name.
    /// * `BinPath` - Returns a pointer to a NULL-terminated string containing the full path name to the BIN file.
    ///               The caller must call LocalFree on the returned pointer in order to release the memory occupied by the string.
    /// * `VsvPath` - Returns a pointer to a NULL-terminated string containing the full path name to the VSV file.
    ///               The caller must call LocalFree on the returned pointer in order to release the memory occupied by the string.
    /// * `VmrsPath` - Returns a pointer to a NULL-terminated string containing the full path name to the VMRS file.
    ///                The caller must call LocalFree on the returned pointer in order to release the memory occupied by the string.
    ///
    /// # Returns
    ///
    /// * `S_OK` - The full path(s) to the saved state file were returned successfully.
    /// * `E_OUTOFMEMORY` - There was insufficient memory to return the full path(s).
    /// * `HResult` - Other HResult failure codes might be returned.
    ///
    pub fn LocateSavedStateFiles(
        VmName: LPCWStr,
        SnapshotName: LPCWStr,
        BinPath: *mut LPWStr,
        VsvPath: *mut LPWStr,
        VmrsPath: *mut LPWStr,
    ) -> HResult;

    /// Loads the given saved state file and creates an instance of VmSavedStateDump.
    /// This instance can be referenced on the other methods with the returned UINT64 Id.
    ///
    /// # Arguments
    ///
    /// * `VmrsFile` - Supplies the path to the VMRS file to load.
    /// * `VmSavedStateDumpHandle` - Returns a Handle to the dump provider instance created.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn LoadSavedStateFile(
        VmrsFile: LPCWStr,
        VmSavedStateDumpHandle: *mut VmSavedStateDumpHandle,
    ) -> HResult;

    /// Opens the given saved state file in read-write exclusive mode so that it applies any pending
    /// replay logs to the contents. This method doesn't loads the saved state file into the library
    /// and can't be used to get content data; function LoadSavedStateFile must be used instead.
    ///
    /// # Arguments
    ///
    /// * `mrsFile` - Supplies the path to the VMRS file whose any pending replay log will be applied.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn ApplyPendingSavedStateFileReplayLog(VmrsFile: LPCWStr) -> HResult;

    /// Loads the given saved state files and creates an instance of VmSavedStateDump.
    /// This instance can be referenced on the other methods with the returned UINT64 Id.
    ///
    /// # Arguments
    ///
    /// * `BinFile` - Supplies the path to the BIN file to load.
    /// * `VsvFile` - Supplies the path to the VSV file to load.
    /// * `VmSavedStateDumpHandle` - Returns the ID for the dump provider instance created.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn LoadSavedStateFiles(
        BinFile: LPCWStr,
        VsvFile: LPCWStr,
        VmSavedStateDumpHandle: *mut VmSavedStateDumpHandle,
    ) -> HResult;

    /// Releases the given VmSavedStateDump provider that matches the supplied ID.
    /// Releasing the provider releases the locks to the saved state files.
    /// This means that it won't be available for use on other methods.
    ///
    /// # Arguments
    ///
    /// * `mSavedStateDumpHandle` - Supplies the ID of the dump provider instance to release.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn ReleaseSavedStateFiles(VmSavedStateDumpHandle: VmSavedStateDumpHandle) -> HResult;

    /// Queries for the Virtual Processor count for a given VmSavedStateDump.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `VpCount` - Returns the Virtual Processor count.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn GetVpCount(VmSavedStateDumpHandle: VmSavedStateDumpHandle, VpCount: *mut u32)
        -> HResult;

    /// Queries for the current Architecture/ISA the virtual processor was running at the time the
    /// saved state file was generated.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `VpId` - Supplies the VP to query.
    /// * `Architecture` - Returns the architecture of the supplied vp.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn GetArchitecture(
        VmSavedStateDumpHandle: VmSavedStateDumpHandle,
        VpId: u32,
        Architecture: *mut VirtualProcessorArch,
    ) -> HResult;

    /// Queries for a specific register value for a given VP in a VmSavedStateDump.
    /// Callers must specify architecture and register ID in parameter Register, and this function
    /// returns the register value through it.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `VpId` - Supplies the Virtual Processor Id.
    /// * `Register` - Supplies the register architecture and ID, and returns the value.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn GetRegisterValue(
        VmSavedStateDumpHandle: VmSavedStateDumpHandle,
        VpId: u32,
        Register: *mut VirtualProcessorRegister,
    ) -> HResult;

    /// Queries for the current Paging Mode in use by the virtual processor at the time the
    /// saved state file was generated.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `VpId` - Supplies the Virtual Processor Id.
    /// * `PagingMode` - Returns the paging mode.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn GetPagingMode(
        VmSavedStateDumpHandle: VmSavedStateDumpHandle,
        VpId: u32,
        PagingMode: *mut PagingMode,
    ) -> HResult;

    /// Reads from the saved state file the given guest physical address range and then
    /// it is written into the supplied buffer.
    /// If BytesRead returns something lower than BufferSize, then the end of memory has been reached.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `PhysicalAddress` - Supplies the physical address to read.
    /// * `Buffer` - Returns the read memory range on the given address.
    /// * `BufferSize` - Supplies the requested byte count to read.
    /// * `BytesRead` - Optionally returns the bytes actually read.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn ReadGuestPhysicalAddress(
        VmSavedStateDumpHandle: VmSavedStateDumpHandle,
        PhysicalAddress: GuestPhysicalAddress,
        Buffer: PVoid,
        BufferSize: u32,
        BytesRead: *mut u32,
    ) -> HResult;

    /// Translates a virtual address to a physical address using information found in the
    /// guest's memory and processor's state.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `VpId` - Supplies the VP from where the virtual address is read.
    /// * `VirtualAddress` - Supplies the virtual address to translate.
    /// * `PhysicalAddress` - Returns the physical address assigned to the supplied virtual address.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn GuestVirtualAddressToPhysicalAddress(
        VmSavedStateDumpHandle: VmSavedStateDumpHandle,
        VpId: u32,
        VirtualAddress: GuestVirtualAddress,
        PhysicalAddress: *mut GuestPhysicalAddress,
    ) -> HResult;

    /// Returns the layout of the physical memory of the guest. This information contains the chunks of memory
    /// with consecutive pages and from where each one starts. If the supplied count is less than the amount
    /// of chunks for this guest, then this function returns the expected chunk count.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `MemoryChunkPageSize` - Returns the size of a page in the memory chunk layout.
    /// * `MemoryChunks` - Supplies a buffer of memory chunk structures that are filled up with the
    ///                    requested information if the buffer size is the same or bigger than the
    ///                    memory chunks count for this guest.
    /// * `MemoryChunkCount` - Supplies the size of the MemoryChunks buffer. If this count is lower than
    ///                        what the guest really has, then it returns the expected count. If it was
    ///                        higher than what the guest has, then it returns the exact count.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn GetGuestPhysicalMemoryChunks(
        VmSavedStateDumpHandle: VmSavedStateDumpHandle,
        MemoryChunkPageSize: *mut u64,
        MemoryChunks: *mut GpaMemoryChunk,
        MemoryChunkCount: *mut u64,
    ) -> HResult;

    /// Translates the given guest physical address to a raw saved memory offset. This is specially useful
    /// if callers need to read a memory range directly from all of the guest's saved memory starting
    /// in the saved memory address equivalent to the supplied guest physical address.
    /// Translation from raw saved memory offset to physical address is not supported.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `PhysicalAddress` - Supplies the guest physical address to translate.
    /// * `RawSavedMemoryOffset` - Returns the raw saved memory offset for a given physical address.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn GuestPhysicalAddressToRawSavedMemoryOffset(
        VmSavedStateDumpHandle: VmSavedStateDumpHandle,
        PhysicalAddress: GuestPhysicalAddress,
        RawSavedMemoryOffset: *mut u64,
    ) -> HResult;

    /// Reads raw memory from the saved state file. This function reads raw memory from the saved state file
    /// as if it were a flat memory layout, regardless of the guest memory layout.
    /// If BytesRead returns something lower than BufferSize, then the end of memory has been reached.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `RawSavedMemoryOffset` - Byte offset on the raw saved memory from where to start reading.
    /// * `Buffer` - Returns the raw memory read on the current raw memory offset.
    /// * `BufferSize` - Supplies the requested byte count to read.
    /// * `BytesRead` - Optionally returns the bytes actually read.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn ReadGuestRawSavedMemory(
        VmSavedStateDumpHandle: VmSavedStateDumpHandle,
        RawSavedMemoryOffset: u64,
        Buffer: *mut PVoid,
        BufferSize: u32,
        BytesRead: *mut u32,
    ) -> HResult;

    /// Returns the size in bytes of the saved memory for a given VM saved state file.
    ///
    /// # Arguments
    ///
    /// * `VmSavedStateDumpHandle` - Supplies a handle to a dump provider instance.
    /// * `GuestRawSavedMemorySize` - Returns the size of the saved memory of a given guest in bytes.
    ///
    /// # Returns
    ///
    /// * `HResult`.
    ///
    pub fn GetGuestRawSavedMemorySize(
        VmSavedStateDumpHandle: VmSavedStateDumpHandle,
        GuestRawSavedMemorySize: *mut u64,
    ) -> HResult;

}
