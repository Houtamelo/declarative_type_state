use Transition::{Unchanged, ChangedTo};
use std::ops::{ControlFlow, FromResidual, Try};

pub enum Transition<TCurr, TNext> {
	Unchanged(TCurr),
	ChangedTo(TNext),
}

impl<TCurr, TNext> FromResidual for Transition<TCurr, TNext> {
	fn from_residual(residual: <Self as Try>::Residual) -> Self {
		ChangedTo(residual)
	}
}

impl<TCurr, TNext> Try for Transition<TCurr, TNext> {
	type Output = TCurr;
	type Residual = TNext;

	fn from_output(output: Self::Output) -> Self {
		Unchanged(output)
	}

	fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
		match self {
			Unchanged(same) => {
				ControlFlow::Continue(same)
			}
			ChangedTo(new) => {
				ControlFlow::Break(new)
			}
		}
	}
}
