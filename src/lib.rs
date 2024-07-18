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

	pub enum TransitionResult<TSelf, TEnum> {
		Transitioned(TEnum),
		Unchanged(TSelf),
	}

	impl<TSelf, TEnum> FromResidual for TransitionResult<TSelf, TEnum> {
		fn from_residual(residual: <Self as Try>::Residual) -> Self {
			TransitionResult::Transitioned(residual)
		}
	}

	impl<TSelf, TEnum> Try for TransitionResult<TSelf, TEnum> {
		type Output = TSelf;
		type Residual = TEnum;

		fn from_output(output: Self::Output) -> Self {
			TransitionResult::Unchanged(output)
		}

		fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
			match self {
				TransitionResult::Transitioned(new) => {
					ControlFlow::Break(new)
				}
				TransitionResult::Unchanged(same) => {
					ControlFlow::Continue(same)
				}
			}
		}
	}

	pub trait ToTransitionResult: Sized {
		fn to_result_self<T>(self) -> TransitionResult<Self, T> {
			TransitionResult::Unchanged(self)
		}

		fn to_result_enum<T>(self) -> TransitionResult<Self, T>
			where
				T: From<Self>,
		{
			TransitionResult::Transitioned(self.into())
		}
	}
	
	impl<T: Sized> ToTransitionResult for T {}
}