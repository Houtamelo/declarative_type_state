#![feature(try_trait_v2)]
#![feature(type_changing_struct_update)]
#![allow(clippy::tabs_in_doc_comments)]

mod extract_variants_into_enum;
mod type_table;
mod type_value_table;
mod enum_variants_convert;
mod type_state_enum;
mod enum_delegate_impls;
mod delegated_enum;
mod extract_single_variant;
mod extract_variants;

mod transition_result;
pub use transition_result::{Transition, Transition::Unchanged, Transition::ChangedTo};

