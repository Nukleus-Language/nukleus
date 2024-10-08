mod context;
pub mod cranelift_JIT;

use astgen::ast::*;
use astgen::AST;

use cranelift::prelude::*;
use cranelift_codegen::ir::entities::FuncRef;
use cranelift_codegen::ir::Signature;
use cranelift_codegen::isa::CallConv;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{DataDescription, FuncId, Linkage, Module};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct FuncInfo {
    pub id: FuncId,
    pub fnref: FuncRef,
    pub signature: Signature,
}
