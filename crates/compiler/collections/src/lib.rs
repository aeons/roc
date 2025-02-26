#![warn(clippy::dbg_macro)]
// See github.com/roc-lang/roc/issues/800 for discussion of the large_enum_variant check.
#![allow(clippy::large_enum_variant)]

pub mod all;
mod reference_matrix;
mod small_string_interner;
pub mod soa;
mod vec_map;
mod vec_set;

pub use all::{default_hasher, BumpMap, ImEntry, ImMap, ImSet, MutMap, MutSet, SendMap};
pub use reference_matrix::{ReferenceMatrix, Sccs, TopologicalSort};
pub use small_string_interner::SmallStringInterner;
pub use vec_map::VecMap;
pub use vec_set::VecSet;
