//! Built-in function registry for Lamina backend.
//!
//! This module defines how nukleus built-ins map to Lamina IR. Adding a new
//! function requires: (1) register here, (2) implement lowering in emitter.
//!
//! Lamina provides: `print value` (built-in), `call @name(args)` (external).

use astgen::ast::ASTtypename;

/// Built-in names known to the Lamina backend.
pub const PRINT: &str = "print";
pub const PRINTLN: &str = "println";

/// Returns true if `name` is a built-in that uses lamina's native `print` instruction.
pub fn is_native_print(name: &str) -> bool {
    name == PRINT || name == PRINTLN
}

/// Parameter and return types for a built-in (for future use).
#[derive(Debug, Clone)]
pub struct BuiltinSignature {
    pub params: Vec<ASTtypename>,
    pub return_type: ASTtypename,
}

/// Registry of built-in functions. Extend this to add new built-ins.
pub fn builtin_signature(name: &str) -> Option<BuiltinSignature> {
    match name {
        PRINT => Some(BuiltinSignature {
            params: vec![ASTtypename::QuotedString],
            return_type: ASTtypename::TypeVoid,
        }),
        PRINTLN => Some(BuiltinSignature {
            params: vec![ASTtypename::QuotedString],
            return_type: ASTtypename::TypeVoid,
        }),
        _ => None,
    }
}
