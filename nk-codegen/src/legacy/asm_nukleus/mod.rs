#![allow(
    clippy::module_inception,
    clippy::new_without_default,
    clippy::manual_is_multiple_of,
    clippy::unwrap_used,
    unused_imports,
    unused_variables,
    dead_code,
)]

pub mod asm_nukleus;
pub mod types;
pub mod x86;
pub mod arm;

pub use asm_nukleus::{Architecture, AsmBuilder, CompiledCode, Instruction, Register};
pub use types::{CodegenError, RegisterSize, VarType};
