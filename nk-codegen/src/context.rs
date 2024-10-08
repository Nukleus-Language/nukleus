use std::borrow::Cow;

use cranelift::prelude::types;
use cranelift_codegen::{ir::Signature, isa::CallConv};
use cranelift_module::{DataDescription, FuncId, Linkage, Module};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Cow<'static, str>,
    pub type_name: Cow<'static, str>,
    pub ir_type: types::Type,
    pub is_mutable: bool,
    pub address: i32,
}
