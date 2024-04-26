//! The module/file/archive linker

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

    /// Link EVM modules.
    ///
    /// Is supposed to link both EVM deploy and runtime code, and CREATE dependencies.
    pub fn LLVMLinkMemoryBuffers(
        InMemBufs: *const LLVMMemoryBufferRef,
        NumInBufs: ::libc::c_uint,
        OutMemBuf: *mut LLVMMemoryBufferRef,
        LldArgs: *const *const ::libc::c_char,
        NumLldArgs: ::libc::c_uint,
    ) -> LLVMBool;
}
