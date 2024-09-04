#![feature(try_trait_v2)]
#![feature(type_changing_struct_update)]
#![feature(macro_metavar_expr)]
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
mod enum_variants_table;

mod transition_result;
pub use transition_result::{Transition, Transition::Unchanged, Transition::ChangedTo};

pub trait FromEnum<Enum> {
	fn from_enum(t: Enum) -> Option<Self> where Self: Sized;
}

pub trait ToVariant<Variant> {
	fn to_variant(self) -> Option<Variant>;
}

impl<Enum, Variant> ToVariant<Variant> for Enum where Variant: FromEnum<Enum> {
	fn to_variant(self) -> Option<Variant> { Variant::from_enum(self) }
}

pub trait Is {
	fn is<Variant>(&self) -> bool where for<'a> &'a Self: ToVariant<&'a Variant> {
		self.to_variant().is_some()
	}
}

impl<T> Is for T {}
