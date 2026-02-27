#[cfg(feature = "jit")]
mod context;
#[cfg(feature = "jit")]
pub mod cranelift_jit;
pub mod error;
pub mod lamina;

#[cfg(feature = "jit")]
use cranelift_codegen::ir::Signature;
#[cfg(feature = "jit")]
use cranelift_codegen::ir::entities::FuncRef;
#[cfg(feature = "jit")]
use cranelift_module::FuncId;

#[cfg(feature = "jit")]
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct FuncInfo {
    pub id: FuncId,
    pub fnref: FuncRef,
    pub signature: Signature,
}
