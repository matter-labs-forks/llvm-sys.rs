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
    /// Translate textual assembly to object code.
    /// 
    /// The unlinked EraVM bytecode is written to `OutMemBuf`, which must then be
    /// passed to `LLVMLinkEraVM` for linkage.
    pub fn LLVMAssembleEraVM(
        TargetMachine: LLVMTargetMachineRef,
        InMemBuf: LLVMMemoryBufferRef,
        OutMemBuf: *mut LLVMMemoryBufferRef,
    ) -> LLVMBool;

    /// Check if the bytecode fits into the EraVM size limit.
    pub fn LLVMExceedsSizeLimitEraVM(
        InMemBuf: LLVMMemoryBufferRef,
    ) -> LLVMBool;
    
    /// Link EraVM module.
    ///
    /// Removes the ELF wrapper from an EraVM module.
    pub fn LLVMLinkEraVM(
        InMemBuf: LLVMMemoryBufferRef,
        OutMemBuf: *mut LLVMMemoryBufferRef,
    ) -> LLVMBool;
    
    /// Link the source module into the destination module.
    ///
    /// Destroys the source module, returns true on error. Use the diagnostic
    /// handler to get any diagnostic message.
    pub fn LLVMLinkModules2(Dest: LLVMModuleRef, Src: LLVMModuleRef) -> LLVMBool;
}
