//! The module/file/archive linker

use crate::target_machine::LLVMTargetMachineRef;

use super::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub enum LLVMLinkerMode {
    LLVMLinkerDestroySource = 0,
    #[deprecated(since = "37.0.0", note = "LLVMLinkerPreserveSource has no effect")]
    LLVMLinkerPreserveSource_Removed = 1,
}

extern "C" {
    /// Link the source module into the destination module.
    ///
    /// Destroys the source module, returns true on error. Use the diagnostic
    /// handler to get any diagnostic message.
    pub fn LLVMLinkModules2(Dest: LLVMModuleRef, Src: LLVMModuleRef) -> LLVMBool;

    /// Returns EVM immutables and their offsets of the ELF object file passed in `InMemBuf`.
    pub fn LLVMGetImmutablesEVM(
        InMemBuf: LLVMMemoryBufferRef,
        immutableIDs: *mut *mut *mut ::libc::c_char,
        immutableOffsets: *mut *mut u64,
    ) -> u64;

    /// Links all EVM dependencies with the main module.
    /// All input buffers must be valid ELF object files.
    pub fn LLVMAssembleEVM(
        CodeSegment: u64,
        InMemBufs: *const LLVMMemoryBufferRef,
        InMemBufIDs: *const *const ::libc::c_char,
        NumInBufs: u64,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Resolves undefined linker symbols in the ELF object file `InMemBuf`.
    /// Returns ELF object file if there remain unresolved linker symbols. Otherwise returns the bytecode.
    pub fn LLVMLinkEVM(
        InMemBuf: LLVMMemoryBufferRef,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        LinkerSymbolKeys: *const *const ::libc::c_char,
        LinkerSymbolValues: *const ::libc::c_char,
        LinkerSymbolsSize: u64,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Check whether the EVM memory buffer is a valid ELF binary.
    pub fn LLVMIsELFEVM(InMemBuf: LLVMMemoryBufferRef) -> LLVMBool;

    /// Dispose immutable names and their offsets returned by LLVMGetImmutablesEVM.
    pub fn LLVMDisposeImmutablesEVM(
        immutableIDs: *const *const ::libc::c_char,
        immutableOffsets: *const u64,
        numOfImmutables: u64,
    );

    /// Translate textual assembly to object code.
    ///
    /// The unlinked EraVM bytecode is written to `OutMemBuf`, which must then be
    /// passed to `LLVMLinkEraVM` for linkage.
    pub fn LLVMAssembleEraVM(
        TargetMachine: LLVMTargetMachineRef,
        InMemBuf: LLVMMemoryBufferRef,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Disassemble the EraVM bytecode passed in `InBuffer`` starting at the offset `PC`.
    ///
    /// The result is returned via `OutBuffer``.
    /// In case of an error the function returns 'true' and an error message is passed
    /// via `ErrorMessage``. The message should be disposed with `LLVMDisposeMessage`.
    pub fn LLVMDisassembleEraVM(
        TargetMachine: LLVMTargetMachineRef,
        InMemBuf: LLVMMemoryBufferRef,
        PC: u64,
        Options: u64,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Check whether the EraVM memory buffer is a valid ELF binary.
    pub fn LLVMIsELFEraVM(InMemBuf: LLVMMemoryBufferRef) -> LLVMBool;

    /// Check if the bytecode fits into the EraVM size limit.
    pub fn LLVMExceedsSizeLimitEraVM(InMemBuf: LLVMMemoryBufferRef, MetadataSize: u64) -> LLVMBool;

    /// Return undefined references of the ELF object.
    pub fn LLVMGetUndefinedReferencesEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        LinkerSymbols: *mut *mut *mut ::libc::c_char,
        LinkerSymbolsSize: *mut u64,
        FactoryDependencies: *mut *mut *mut ::libc::c_char,
        FactoryDependenciesSize: *mut u64,
    );

    /// Link EraVM module.
    ///
    /// Removes the ELF wrapper from an EraVM module if all symbols are resolved.
    pub fn LLVMLinkEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        LinkerSymbolKeys: *const *const ::libc::c_char,
        LinkerSymbolValues: *const ::libc::c_char,
        LinkerSymbolsSize: u64,
        FactoryDependencyKeys: *const *const ::libc::c_char,
        FactoryDependencyValues: *const ::libc::c_char,
        FactoryDependenciesSize: u64,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Add metadata to the ELF-wrapped module.
    pub fn LLVMAddMetadata(
        InMemBuf: LLVMMemoryBufferRef,
        MetadataPtr: *const ::libc::c_char,
        MetadataSize: u64,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Dispose the undefined references.
    pub fn LLVMDisposeUndefinedReferences(
        References: *const *const ::libc::c_char,
        ReferencesSize: u64,
    );
}
