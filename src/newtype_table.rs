#[macro_export]
macro_rules! newtype_table {
	//------------------------------------------------------------------------------------------------------------------
	// Table + Enum + Delegates
	(
		ENUM: {
		    #[vars( $( $all_meta: meta ),* $(,)? )]
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
				$(
					$( #[$var_meta: meta] )*
					$var_ident: ident ($var_ty: ty)
			    ),*
			    $(,)?
		    }
	    }

		TABLE: {
			$( #[$table_meta: meta] )*
			$table_vis: vis struct $table_ident: ident $(;)? $({})?
		}

		DELEGATES: {
		    $(
		        impl $( <[ $( $trait_gen: tt )*  ]> )?
		        trait $trait_ty: path
		        $( where [ $( $trait_bound: tt )* ] )?
		        {
				    $( [ $( $item: tt )* ] )*
			    }
		    )*

		    $(
			    impl {
			        $( [ $( $std_impl: tt )* ] )*
			    }
		    )?
	    }
	) => {
		$crate::newtype_table! {
			ENUM: {
			    #[vars( $( $all_meta ),* )]
			    $( #[$enum_meta] )*
			    $enum_vis enum $enum_ident {
					$(
						$( #[$var_meta] )*
						$var_ident($var_ty)
				    ),*
			    }
		    }

			TABLE: {
				$( #[$table_meta] )*
				$table_vis struct $table_ident
			}
		}

		$crate::enum_delegate_impls! {
			ENUM_IN: {
				$enum_ident {
					$( $var_ident ( $var_ty ) ),*
			    }
		    }

			DELEGATES: {
			    $(
			        impl $( <[ $( $trait_gen )*  ]> )?
				    trait $trait_ty
			        $( where [ $( $trait_bound )* ] )?
			        {
					    $( [ $( $item )* ] )*
				    }
			    )*

			    $(
				    impl {
					    $( [ $( $std_impl )* ] )*
				    }
			    )?
		    }
		}
	};

	//------------------------------------------------------------------------------------------------------------------
	// Table + Enum
	(
		ENUM: {
		    #[vars( $( $all_meta: meta ),* $(,)? )]
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
				$(
					$( #[$var_meta: meta] )*
					$var_ident: ident ($var_ty: ty)
			    ),*
			    $(,)?
		    }
	    }

		TABLE: {
			$( #[$table_meta: meta] )*
			$table_vis: vis struct $table_ident: ident $(;)? $({})?
		}
	) => {
		$crate::extract_variants! {
			#[vars( $( $all_meta ),* )]
			$( #[$enum_meta] )*
			$enum_vis enum $enum_ident {
			    $(
			        $( #[$var_meta] )*
			        $var_ident($var_ty)
			    ),*
		    }
		}

		$( #[$enum_meta] )*
		$enum_vis enum $enum_ident {
		    $(
		        $( #[$var_meta] )*
		        $var_ident($var_ty)
		    ),*
	    }

		$crate::type_table! {
			@TABLE_INTERNAL
			$enum_ident

			$( #[$table_meta] )*
			$table_vis struct $table_ident {
			    $($var_ident: $var_ty),*
			}
		}
	};
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::newtype_table;

	newtype_table! {
		ENUM: {
			#[vars(derive(Debug, Clone))]
			pub enum Duration {
				Seconds(f64),
				Days(isize),
				Hours(isize),
				Infinite(()),
			}
		}

		TABLE: {
			#[derive(Debug, Clone)]
			pub struct DurationTable;
		}
	}

	#[test]
	fn test() {
		let table = DurationTable::new(0.0, 4, 4, ());
		let seconds: &f64 = table.get::<Seconds>();
		let days: &isize = table.get::<Days>();
		let hours: &isize = table.get::<Hours>();
		let infinite: &() = table.get::<Infinite>();

		assert_eq!(*seconds, 0.0);
		assert_eq!(*days, 4);
		assert_eq!(*hours, 4);
		assert_eq!(infinite, &());
	}
}

/// ```
///
/// ```
#[allow(unused)]
#[cfg(test)]
mod tests_2 {
	use std::fmt::{Debug, Formatter};

	use crate::newtype_table;

	newtype_table! {
		ENUM: {
			#[vars(derive(Debug, Clone))]
			pub enum Duration {
				Seconds(f64),
				Days(isize),
				Hours(isize),
				Infinite(()),
			}
		}

		TABLE: {
			#[derive(Debug, Clone)]
			pub struct DurationTable;
		}

		DELEGATES: {
			impl trait Debug {
				[fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>]
			}
		}
	}
}
