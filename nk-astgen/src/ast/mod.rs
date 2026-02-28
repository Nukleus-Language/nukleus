//! Re-export AST types from nk-ast for API compatibility.
//!
//! New code should use `nk_ast::*` directly. This module preserves
//! `astgen::ast::*` and `astgen::AST` for existing callers.

pub use nk_ast::*;
