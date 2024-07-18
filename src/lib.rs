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
pub enum TransitionResult<TSelf, TEnum> {
	Transitioned(TEnum),
	Unchanged(TSelf),
}

#[cfg(feature = "try_impl")]
impl<TSelf, TEnum> std::ops::FromResidual for TransitionResult<TSelf, TEnum> {
	fn from_residual(residual: <Self as std::ops::Try>::Residual) -> Self {
		TransitionResult::Transitioned(residual)
	}
}

#[cfg(feature = "try_impl")]
impl<TSelf, TEnum> std::ops::Try for TransitionResult<TSelf, TEnum> {
	type Output = TSelf;
	type Residual = TEnum;

	fn from_output(output: Self::Output) -> Self {
		TransitionResult::Unchanged(output)
	}

	fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
		match self {
			TransitionResult::Transitioned(new) => {
				std::ops::ControlFlow::Break(new)
			}
			TransitionResult::Unchanged(same) => {
				std::ops::ControlFlow::Continue(same)
			}
		}
	}
}