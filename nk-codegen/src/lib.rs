mod context;
pub mod error;
pub mod cranelift_jit;


use cranelift_codegen::ir::entities::FuncRef;
use cranelift_codegen::ir::Signature;
use cranelift_module::FuncId;

#[derive(Debug, Clone)]
struct FuncInfo {
    pub id: FuncId,
    pub fnref: FuncRef,
    pub signature: Signature,
}
