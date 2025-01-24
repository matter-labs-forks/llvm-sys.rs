//! The module/file/archive linker

use crate::target_machine::LLVMTargetMachineRef;

use super::prelude::*;

#[repr(C)]
#[derive(Debug)]
pub enum LLVMLinkerMode {
    LLVMLinkerDestroySource = 0,
    #[deprecated(since = "3.7.0", note = "LLVMLinkerPreserveSource has no effect")]
    LLVMLinkerPreserveSource_Removed = 1,
}

extern "C" {
    /// Link the source module into the destination module.
    ///
    /// Destroys the source module, returns true on error. Use the diagnostic
    /// handler to get any diagnostic message.
    pub fn LLVMLinkModules2(Dest: LLVMModuleRef, Src: LLVMModuleRef) -> LLVMBool;

    /// Links the deploy and runtime ELF object files using the information about dependencies.
    pub fn LLVMLinkEVM(
        InMemBufs: *const LLVMMemoryBufferRef,
        InMemBufIDs: *const *const ::libc::c_char,
        NumInBufs: u64,
        OutMemBufs: *mut [LLVMMemoryBufferRef; 2],
        LinkerSymbolKeys: *const *const ::libc::c_char,
        LinkerSymbolValues: *const ::libc::c_char,
        LinkerSymbolsSize: u64,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Check whether the EVM memory buffer is a valid ELF binary.
    pub fn LLVMIsELFEVM(InMemBuf: LLVMMemoryBufferRef) -> LLVMBool;

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

    /// Disassemble the bytecode passed in `InBuffer`` starting at the offset `PC`.
    ///
    /// The result is returned via `OutBuffer``.
    /// In case of an error the function returns 'true' and an error message is passed
    /// via `ErrorMessage``. The message should be disposed with `LLVMDisposeMessage`.
    ///
    /// Added in LLVM patch: https://github.com/matter-labs/era-compiler-llvm/pull/692
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

    /// Add metadata to the ELF wrapper.
    pub fn LLVMAddMetadataEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        MetadataPtr: *const ::libc::c_char,
        MetadataSize: u64,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

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

    /// Dispose the undefined references.
    pub fn LLVMDisposeUndefinedReferences(
        References: *const *const ::libc::c_char,
        ReferencesSize: u64,
    );
}
