//! Type expression model used for richer dependency analysis.

use serde::{Deserialize, Serialize};

/// Rich representation of a TypeScript type expression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TypeExpr {
    /// A named type reference (e.g., CustomObject)
    Named(String),
    /// Array of another type (e.g., CustomField[])
    Array(Box<TypeExpr>),
    /// Union of multiple types (e.g., A | B)
    Union(Vec<TypeExpr>),
    /// Intersection of multiple types (e.g., A & B)
    Intersection(Vec<TypeExpr>),
    /// Generic type reference (e.g., Foo<Bar, Baz>)
    Generic { base: String, args: Vec<TypeExpr> },
    /// Tuple type (e.g., [A, B])
    Tuple(Vec<TypeExpr>),
    /// Literal value type (e.g., "Foo")
    Literal(String),
    /// Object literal type or inline structure
    Object,
    /// Unknown or unsupported expression
    Unknown,
}

impl TypeExpr {
    pub fn named(name: impl Into<String>) -> Self {
        Self::Named(name.into())
    }
}
