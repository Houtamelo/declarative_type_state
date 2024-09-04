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

pub trait ToVariantRef<Variant> {
	fn to_variant_ref(&self) -> Option<&Variant>;
}

impl<Enum, Variant> ToVariantRef<Variant> for Enum where Variant: FromEnumRef<Enum> {
	fn to_variant_ref(&self) -> Option<&Variant> { Variant::from_enum_ref(self) }
}

pub trait ToVariantMut<Variant> {
	fn to_variant_mut(&mut self) -> Option<&mut Variant>;
}

impl<Enum, Variant> ToVariantMut<Variant> for Enum where Variant: FromEnumMut<Enum> {
	fn to_variant_mut(&mut self) -> Option<&mut Variant> { Variant::from_enum_mut(self) }
}

pub trait FromEnumRef<Enum> {
	fn from_enum_ref(t: &Enum) -> Option<&Self>;
}

impl<Enum, Variant> FromEnumRef<Enum> for Variant where for<'a> &'a Variant: FromEnum<&'a Enum> {
	fn from_enum_ref(t: &Enum) -> Option<&Self> {
		t.to_variant()
	}
}

pub trait FromEnumMut<Enum> {
	fn from_enum_mut(t: &mut Enum) -> Option<&mut Self>;
}

impl<Enum, Variant> FromEnumMut<Enum> for Variant
	where
			for<'a> &'a mut Variant: FromEnum<&'a mut Enum>
{
	fn from_enum_mut(t: &mut Enum) -> Option<&mut Self> {
		t.to_variant()
	}
}

pub trait Is {
	fn is<Variant>(&self) -> bool where for<'a> &'a Self: ToVariant<&'a Variant> {
		self.to_variant().is_some()
	}
}

impl<T> Is for T {}
