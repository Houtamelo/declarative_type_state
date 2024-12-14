#[macro_export]
macro_rules! type_value_table {
	//------------------------------------------------------------------------------------------------------------------
	// Table + Generated enum
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
			$table_vis: vis struct $table_ident: ident < $gen: ident >
			$( where [ $( $bounds: tt )* ] )?
			$(;)? $({})?
		}
	) => {
		$crate::type_value_table! {
			ENUM_IN: $enum_ident;

			TABLE: {
				$( #[$table_meta] )*
				$table_vis struct $table_ident < $gen > $( where [ $( $bounds )* ] )? {
				    $( $var_ident ),*
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
			$table_vis: vis struct $table_ident: ident < $gen: ident >
			$( where [ $( $bounds: tt )* ] )?
			$(;)? $({})?
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
		$crate::type_value_table! {
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
				$table_vis struct $table_ident< $gen > $( where [ $( $bounds )* ] )?
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
			$table_vis: vis struct $table_ident: ident< $gen: ident >
			$( where [ $( $bounds: tt )* ] )?
			{
			    $( $var_ident: ident ),*
			    $(,)?
		    }
		}
	) => {
		$crate::type_value_table! {
			@TABLE_INTERNAL
			$enum_ident

			$( #[$table_meta] )*
			$table_vis struct $table_ident< $gen >
			[$( $( $bounds )* )?]
			$( where [ $( $bounds )* ] )?
			{
			    $( $var_ident ),*
		    }
		}
	};

	//------------------------------------------------------------------------------------------------------------------
	// Base impl
	(
		@TABLE_INTERNAL
		$enum_ident: ident

		$( #[$table_meta: meta] )*
		$table_vis: vis struct $table_ident: ident < $gen: ident >
		$token_bounds: tt
		$( where [ $( $bounds: tt )* ] )?
		{
		    $( $var_ident: ident ),*
		    $(,)?
	    }
	) => {
		$crate::paste! {
			$( #[$table_meta] )*
		    $table_vis struct $table_ident<$gen>
			$( where $( $bounds )*  )?
			{
		        $([<$var_ident:snake:lower>]: $gen),*
		    }

			$table_vis enum [<$enum_ident Ref>]<'a, $gen>
			$( where $( $bounds )*  )?
			{
			    $($var_ident(&'a $gen)),*
		    }

		    $table_vis enum [<$enum_ident Mut>]<'a, $gen>
			$( where $( $bounds )*  )?
			{
			    $($var_ident(&'a mut $gen)),*
		    }

		    $(
			    $crate::type_value_table! {
				    @MEMBER_OF_IMPL
				    $gen;
				    $var_ident;
				    $table_ident;
				    $token_bounds
			    }
		    )*

			impl<$gen> $table_ident<$gen>
			$( where $( $bounds )*  )?
			{
				pub const LENGTH: usize = ${count($var_ident)};

				pub fn get<Member: $crate::MemberOf<Self>>(&self) -> &Member::MemberType {
				    Member::get_in_table(self)
			    }

				pub fn get_mut<Member: $crate::MemberOf<Self>>(&mut self) -> &mut Member::MemberType {
				    Member::get_in_table_mut(self)
			    }

				#[allow(clippy::too_many_arguments)]
				pub const fn new( $( [<$var_ident:snake:lower>]: $gen ),* ) -> Self {
			        Self {
			            $( [<$var_ident:snake:lower>] ),*
			        }
			    }

				#[allow(clippy::needless_lifetimes)]
				pub fn iter<'a>(&'a self) -> impl Iterator<Item = [<$enum_ident Ref>]<'a, $gen>> {
					[
						$( [<$enum_ident Ref>]::$var_ident(&self.[<$var_ident:snake:lower>]) ),*
					].into_iter()
				}

				#[allow(clippy::needless_lifetimes)]
				pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = [<$enum_ident Mut>]<'a, $gen>> {
					[
						$( [<$enum_ident Mut>]::$var_ident(&mut self.[<$var_ident:snake:lower>]) ),*
					].into_iter()
				}
			}

			impl<$gen> IntoIterator for $table_ident<$gen>
			$( where $( $bounds )*  )?
			{
			    type Item = $gen;
				type IntoIter = core::array::IntoIter<$gen, { $table_ident::<()>::LENGTH }>;

				fn into_iter(self) -> Self::IntoIter {
					[ $( self.[<$var_ident:snake:lower>] ),* ].into_iter()
				}
		    }
		}
	};

	// MemberOf impl
	(@MEMBER_OF_IMPL
		$gen: ident;
		$var_ident: ident;
		$table_ident: ident;
		[$( $bounds: tt )*]
	) => {
		$crate::paste! {
			impl<$gen> $crate::MemberOf<$table_ident<$gen>> for $var_ident
			where $( $bounds )*
		    {
			    type MemberType = $gen;

		        fn get_in_table(table: & $table_ident<$gen>) -> &Self::MemberType {
		            &table.[<$var_ident:snake:lower>]
		        }

		        fn get_in_table_mut(table: &mut $table_ident<$gen>) -> &mut Self::MemberType {
		            &mut table.[<$var_ident:snake:lower>]
		        }
		    }
		}
	};
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::type_value_table;

	type_value_table! {
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
			// Name of the module that will contain all the generated code
			#[derive(Debug, Clone)] // Attributes to apply on the Table
			pub struct DurationTable < Val >
		}
	}

	#[test]
	fn test() {
		let table = DurationTable::<i32>::new(5, 5, 3, 4);
		let seconds: &i32 = table.get::<Seconds>();
		let days_seconds: &i32 = table.get::<DaysSeconds>();
		let hours_minutes: &i32 = table.get::<HoursMinutes>();
		let infinite: &i32 = table.get::<Infinite>();

		assert_eq!(*seconds, 5);
		assert_eq!(*days_seconds, 5);
		assert_eq!(*hours_minutes, 3);
		assert_eq!(*infinite, 4);
	}
}

#[allow(unused)]
#[cfg(test)]
mod tests_2 {
	use std::fmt::{Debug, Formatter};

	use crate::type_value_table;

	#[derive(Debug, Clone)]
	pub struct Sealed;

	type_value_table! {
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
			// Name of the module that will contain all the generated code
			#[derive(Debug, Clone)] // Attributes to apply on the Table
			pub struct DurationTable < T > where [T: Copy]
		}

		DELEGATES: {
			impl trait Debug {
				[fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>]
			}
		}
	}
}
