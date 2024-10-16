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

    /// Links the deploy and runtime ELF object files using the information about
    /// dependencies.
    /// `inBuffers` - array of input memory buffers with following structure:
    ///
    /// `inBuffers[0]` - deploy ELF object code
    /// `inBuffers[1]` - deployed (runtime) ELF object code
    /// --------------------------
    /// `inBuffers[2]` - 1-st sub-contract (final EVM bytecode)
    /// ...
    /// `inBuffers[N]` - N-st sub-contract (final EVM bytecode)
    ///
    /// Sub-contracts are optional. They should have the same ordering as in
    /// the Yul layout.
    ///
    /// `inBuffersIDs` - array of string identifiers of the buffers. IDs correspond
    /// to the object names in the Yul layout.
    /// On success, `outBuffers[0]` will contain the deploy bytecode and `outBuffers[1]`
    /// the runtime bytecode.
    /// In case of an error the function returns `true` and the error message is
    /// passes in `ErrorMessage`. The message should be disposed by
    /// `LLVMDisposeMessage`.
    pub fn LLVMLinkEVM(
        InMemBufs: *const LLVMMemoryBufferRef,
        InMemBufIDs: *const *const ::libc::c_char,
        NumInBufs: u64,
        OutMemBufs: *mut [LLVMMemoryBufferRef; 2],
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;

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

    /// Check whether the memory buffer is a valid ELF binary.
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

    /// Return unresolved symbols from the ELF wrapper.
    pub fn LLVMGetUndefinedLinkerSymbolsEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        LinkerSymbolsSize: *mut u64,
    ) -> *const *const ::libc::c_char;

    /// Link EraVM module.
    ///
    /// Removes the ELF wrapper from an EraVM module if all symbols are resolved.
    pub fn LLVMLinkEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        LinkerSymbols: *const *const ::libc::c_char,
        LinkerSymbolValues: *const ::libc::c_char,
        LinkerSymbolsSize: u64,
        ErrorMessage: *mut *mut ::libc::c_char,
    ) -> LLVMBool;
}
