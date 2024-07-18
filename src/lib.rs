#![cfg_attr(feature = "try_impl", feature(try_trait_v2))]
#![allow(clippy::tabs_in_doc_comments)]

mod extract_variants_into_enum;
mod type_table;
mod enum_variants_convert;
mod type_state_enum;
mod enum_delegate_impls;
mod extract_single_variant;
mod extract_variants;

#[cfg(feature = "try_impl")]
pub mod transition_result {
	use std::ops::{ControlFlow, FromResidual, Try};

	pub enum Transition<TCurr, TNext> {
		Transitioned(TNext),
		Unchanged(TCurr),
	}

	impl<TCurr, TNext> FromResidual for Transition<TCurr, TNext> {
		fn from_residual(residual: <Self as Try>::Residual) -> Self {
			Transition::Transitioned(residual)
		}
	}

	impl<TCurr, TNext> Try for Transition<TCurr, TNext> {
		type Output = TCurr;
		type Residual = TNext;

		fn from_output(output: Self::Output) -> Self {
			Transition::Unchanged(output)
		}

		fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
			match self {
				Transition::Transitioned(new) => {
					ControlFlow::Break(new)
				}
				Transition::Unchanged(same) => {
					ControlFlow::Continue(same)
				}
			}
		}
	}

	pub trait ToTransition: Sized {
		fn unchanged<TNext>(self) -> Transition<Self, TNext> {
			Transition::Unchanged(self)
		}

		fn transitioned<TCurr, TNext>(self) -> Transition<TCurr, TNext>
			where
				TNext: From<Self>,
		{
			Transition::Transitioned(self.into())
		}
	}
	
	impl<T: Sized> ToTransition for T {}
}