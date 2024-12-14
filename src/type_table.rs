#[macro_export]
macro_rules! type_table {
	//------------------------------------------------------------------------------------------------------------------
	// Table + Generated enum
	(
		ENUM_OUT: {
		    #[vars( $( $all_meta: meta ),* $(,)? )]
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
				$(
					$( #[$var_meta: meta] )*
					$var_ident: ident
					$( ( $($var_tuple: tt)* ) )?
					$( { $($var_fields: tt)* } )?
			    ),*
			    $(,)?
		    }
	    }

		TABLE: {
			$( #[$table_meta: meta] )*
			$table_vis: vis struct $table_ident: ident $(;)? $({})?
		}
	) => {
		$crate::type_table! {
			ENUM_IN: $enum_ident;

			TABLE: {
				$( #[$table_meta] )*
				$table_vis struct $table_ident {
				    $( $var_ident: $var_ident ),*
			    }
			}
		}

		$crate::extract_variants_into_enum! {
			#[vars( $( $all_meta ),* )]
			$( #[$enum_meta] )*
			$enum_vis enum $enum_ident {
			    $(
			        $( #[$var_meta] )*
			        $var_ident $( ( $($var_tuple)* ) )? $( { $($var_fields)* } )?
			    ),*
		    }
		}
	};

	//------------------------------------------------------------------------------------------------------------------
	// Table + Generated enum + Delegates
	(
		ENUM_OUT: {
		    #[vars( $( $all_meta: meta ),* $(,)? )]
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
				$(
					$( #[$var_meta: meta] )*
					$var_ident: ident $( ( $($var_tuple: tt)* ) )? $( { $($var_fields: tt)* } )?
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
		$crate::type_table! {
			ENUM_OUT: {
			    #[vars( $( $all_meta ),* )]
			    $( #[$enum_meta] )*
			    $enum_vis enum $enum_ident {
					$(
						$( #[$var_meta] )*
						$var_ident $( ( $($var_tuple)* ) )? $( { $($var_fields)* } )?
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
					$( $var_ident ( $var_ident ) ),*
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
	// Table + User enum
	(
		ENUM_IN: $enum_ident: ident $(;)? $({})?

		TABLE: {
			$( #[$table_meta: meta] )*
			$table_vis: vis struct $table_ident: ident {
			    $( $var_ident: ident: $var_ty: ty ),*
			    $(,)?
		    }
		}
	) => {
		$crate::type_table! {
			@TABLE_INTERNAL
			$enum_ident

			$( #[$table_meta] )*
			$table_vis struct $table_ident {
			    $( $var_ident: $var_ty ),*
		    }
		}

		$crate::paste! {
			impl IntoIterator for $table_ident {
			    type Item = $enum_ident;
				type IntoIter = core::array::IntoIter<$enum_ident, { $table_ident::LENGTH }>;

				fn into_iter(self) -> Self::IntoIter {
					[ $( $enum_ident::$var_ident(self.[<$var_ident:snake:lower>]) ),* ].into_iter()
				}
		    }
		}
	};

	// Table only
	(
		$( #[$table_meta: meta] )*
		$table_vis: vis struct $table_ident: ident {
		    $( $var_ident: ident: $var_ty: ty ),*
		    $(,)?
	    }
	) => {
		$crate::paste! {
			$crate::type_table! {
				@TABLE_INTERNAL
				[<$table_ident Member>]

				$( #[$table_meta] )*
				$table_vis struct $table_ident {
				    $( $var_ident: $var_ty ),*
			    }
			}
		}
	};

	//------------------------------------------------------------------------------------------------------------------
	// Base impl
	(
		@TABLE_INTERNAL
		$enum_ident: ident

		$( #[$table_meta: meta] )*
		$table_vis: vis struct $table_ident: ident {
		    $( $var_ident: ident: $var_ty: ty ),*
		    $(,)?
	    }
	) => {
		$crate::paste! {
			$( #[$table_meta] )*
		    $table_vis struct $table_ident {
		        $([<$var_ident:snake:lower>]: $var_ty),*
		    }

			$table_vis enum [<$enum_ident Ref>]<'a> {
			    $($var_ident(&'a $var_ty)),*
		    }

		    $table_vis enum [<$enum_ident Mut>]<'a> {
			    $($var_ident(&'a mut $var_ty)),*
		    }

		    $(
			    impl $crate::MemberOf<$table_ident> for $var_ident {
				    type MemberType = $var_ty;

		            fn get_in_table(table: & $table_ident) -> &Self::MemberType {
		                &table.[<$var_ident:snake:lower>]
		            }

		            fn get_in_table_mut(table: &mut $table_ident) -> &mut Self::MemberType {
		                &mut table.[<$var_ident:snake:lower>]
		            }
		        }
		    )*

			impl $table_ident {
				pub const LENGTH: usize = ${count($var_ident)};

				pub fn get<Member: $crate::MemberOf<Self>>(&self) -> &Member::MemberType {
				    Member::get_in_table(self)
			    }

				pub fn get_mut<Member: $crate::MemberOf<Self>>(&mut self) -> &mut Member::MemberType {
				    Member::get_in_table_mut(self)
			    }

				#[allow(clippy::too_many_arguments)]
				pub const fn new( $( [<$var_ident:snake:lower>]: $var_ty ),* ) -> Self {
			        Self {
			            $( [<$var_ident:snake:lower>] ),*
			        }
			    }

				#[allow(clippy::needless_lifetimes)]
				pub fn iter<'a>(&'a self) -> impl Iterator<Item = [<$enum_ident Ref>]<'a>> {
					[
						$( [<$enum_ident Ref>]::$var_ident(&self.[<$var_ident:snake:lower>]) ),*
					].into_iter()
				}

				#[allow(clippy::needless_lifetimes)]
				pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = [<$enum_ident Mut>]<'a>> {
					[
						$( [<$enum_ident Mut>]::$var_ident(&mut self.[<$var_ident:snake:lower>]) ),*
					].into_iter()
				}
			}
		}
	};
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::type_table;

	type_table! {
		ENUM_OUT: {
			#[vars(derive(Debug, Clone))]
			pub enum Duration {
				Seconds(f64),
				DaysSeconds(isize, f64),
				HoursMinutes { hours: i32, minutes: i64 },
				Infinite,
			}
		}

		TABLE: {
			#[derive(Debug, Clone)]
			pub struct DurationTable;
		}
	}
}

/// ```
///
/// ```
#[allow(unused)]
#[cfg(test)]
mod tests_2 {
	use std::fmt::{Debug, Formatter};

	use crate::type_table;

	type_table! {
		ENUM_OUT: {
			#[vars(derive(Debug, Clone, PartialEq))]
			pub enum Duration {
				Seconds(f64),
				DaysSeconds(isize, f64),
				HoursMinutes { hours: i32, minutes: i64 },
				Infinite,
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

	#[test]
	fn test() {
		let table = DurationTable::new(
			Seconds(0.0),
			DaysSeconds(4, 2.0),
			HoursMinutes {
				hours:   3,
				minutes: 6,
			},
			Infinite,
		);
		let seconds: &Seconds = table.get::<Seconds>();
		let days_seconds: &DaysSeconds = table.get::<DaysSeconds>();
		let hours_minutes: &HoursMinutes = table.get::<HoursMinutes>();
		let infinite: &Infinite = table.get::<Infinite>();

		assert_eq!(*seconds, Seconds(0.0));
		assert_eq!(*days_seconds, DaysSeconds(4, 2.0));
		assert_eq!(
			*hours_minutes,
			HoursMinutes {
				hours:   3,
				minutes: 6,
			}
		);
		assert_eq!(infinite, &Infinite);
	}
}
