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
        PC: ::libc::c_uint,
        Options: ::libc::c_uint,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Check whether the memory buffer is a valid ELF binary.
    pub fn LLVMIsELFEraVM(InMemBuf: LLVMMemoryBufferRef) -> LLVMBool;

    /// Add metadata to the ELF wrapper.
    pub fn LLVMAddMetadataEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        MetadataPtr: *const ::libc::c_char,
        MetadataSize: ::libc::c_uint,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

    /// Check if the bytecode fits into the EraVM size limit.
    pub fn LLVMExceedsSizeLimitEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        MetadataSize: ::libc::c_uint,
    ) -> LLVMBool;

    /// Return unresolved symbols from the ELF wrapper.
    pub fn LLVMGetUndefinedLinkerSymbolsEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        LinkerSymbolsSize: *mut ::libc::c_uint,
    ) -> *const *const ::libc::c_char;

    /// Link EraVM module.
    ///
    /// Removes the ELF wrapper from an EraVM module if all symbols are resolved.
    pub fn LLVMLinkEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        LinkerSymbols: *const *const ::libc::c_char,
        LinkerSymbolValues: *const ::libc::c_char,
        LinkerSymbolsSize: ::libc::c_uint,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
}
