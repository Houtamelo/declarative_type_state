#![feature(try_trait_v2)]
#![feature(type_changing_struct_update)]
#![feature(macro_metavar_expr)]
#![allow(clippy::tabs_in_doc_comments)]

pub use transition_result::{
	Transition,
	Transition::{ChangedTo, Unchanged},
};

mod extract_variants_into_enum;
mod newtype_table;
mod type_state_enum;
mod type_table;
mod type_value_table;

mod delegated_enum;
mod extract_single_variant;
mod extract_variants;

mod enum_delegate_impls;
mod enum_variants_convert;
mod enum_variants_table;

mod unit_enum_delegate_impls;
mod unit_enum_delegated;
mod unit_enum_variants_convert;

mod transition_result;

pub trait FromEnum<Enum>: Sized {
	fn from_enum(t: Enum) -> Option<Self>;
}

impl<T> FromEnum<T> for T {
	fn from_enum(t: T) -> Option<Self> { Some(t) }
}

pub trait FromEnumRef<Enum> {
	fn from_enum_ref(t: &Enum) -> Option<&Self>;
}

impl<Enum, Variant> FromEnumRef<Enum> for Variant
where for<'a> &'a Variant: FromEnum<&'a Enum>
{
	fn from_enum_ref(t: &Enum) -> Option<&Self> { <&Variant>::from_enum(t) }
}

pub trait FromEnumMut<Enum> {
	fn from_enum_mut(t: &mut Enum) -> Option<&mut Self>;
}

impl<Enum, Variant> FromEnumMut<Enum> for Variant
where for<'a> &'a mut Variant: FromEnum<&'a mut Enum>
{
	fn from_enum_mut(t: &mut Enum) -> Option<&mut Self> { <&mut Variant>::from_enum(t) }
}

#[doc(hidden)]
pub use paste::paste;

pub trait MemberOf<Table> {
	type MemberType;
	fn get_in_table(table: &Table) -> &Self::MemberType;
	fn get_in_table_mut(table: &mut Table) -> &mut Self::MemberType;
}
