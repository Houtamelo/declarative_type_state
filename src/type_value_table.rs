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
			$table_vis: vis struct $table_ident: ident < $gen: ident > $(;)? $({})?
		}
	) => {
		$crate::type_value_table! {
			ENUM_IN: $enum_ident;
			
			TABLE: {
				$( #[$table_meta] )*
				$table_vis struct $table_ident < $gen > {
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
			$table_vis: vis struct $table_ident: ident < $gen: ident > $(;)? $({})?
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
				$table_vis struct $table_ident< $gen >
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
			$table_vis: vis struct $table_ident: ident< $gen: ident > {
			    $( $var_ident: ident ),*
			    $(,)?
		    }
		}
	) => {
		$crate::type_value_table! {
			$( #[$table_meta] )*
			$table_vis struct $table_ident< $gen > {
			    $( $var_ident ),*
		    }
		}
		
		impl< $gen > IntoIterator for $table_ident< $gen > {
		    type Item = $gen;
			type IntoIter = core::array::IntoIter< $gen, { TABLE_LENGTH }>;
	
			fn into_iter(self) -> Self::IntoIter {
				[ $( self.$var_ident ),* ].into_iter()
			}
	    }
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Base impl
	(
		$( #[$table_meta: meta] )*
		$table_vis: vis struct $table_ident: ident < $gen: ident > {
		    $( $var_ident: ident ),*
		    $(,)?
	    }
	) => {
		#[allow(non_camel_case_types)]
		#[allow(non_snake_case)]
		$( #[$table_meta] )*
	    $table_vis struct $table_ident < $gen > {
	        $( $var_ident: $gen ),*
	    }
		
		#[doc(hidden)]
		#[allow(clippy::too_many_arguments)]
		pub mod new_fn {
			use super::$table_ident;
			
			impl< $gen > $table_ident< $gen > {
				#[allow(non_snake_case)]
				pub const fn new( $( $var_ident: $gen ),* ) -> Self {
			        Self {
			            $( $var_ident ),*
			        }
			    }
			}
		}
		
		#[doc(hidden)]
		const TABLE_LENGTH: usize = ${count($var_ident)};
		
		impl<$gen> $table_ident<$gen> {
			pub const LENGTH: usize = TABLE_LENGTH;
			
			pub fn get<THash: GetInTable>(&self) -> & $gen {
			    THash::get_in_table(self)
		    }
			    
			pub fn get_mut<THash: GetInTable>(&mut self) -> &mut $gen {
			    THash::get_in_table_mut(self)
		    }
			
			#[allow(clippy::needless_lifetimes)]
			pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a $gen> {
				[$( &self.$var_ident ),* ].into_iter()
			}
			
			#[allow(clippy::needless_lifetimes)]
			pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut $gen> {
				[$( &mut self.$var_ident ),* ].into_iter()
			}
		}
		
		$table_vis trait GetInTable { 
		    fn get_in_table< $gen >(table: & $table_ident< $gen >) -> & $gen;
	        fn get_in_table_mut< $gen >(table: &mut $table_ident< $gen >) -> &mut $gen;
	    }
		    
	    $(
		    impl GetInTable for $var_ident {
	            fn get_in_table< $gen >(table: & $table_ident<$gen>) -> & $gen {
	                &table.$var_ident
	            }
		        
	            fn get_in_table_mut< $gen >(table: &mut $table_ident<$gen>) -> &mut $gen {
	                &mut table.$var_ident
	            }
	        }
	    )*
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
}


#[allow(unused)]
#[cfg(test)]
mod tests_2 {
	use crate::type_value_table;
	use std::fmt::Debug;
	use std::fmt::Formatter;
	
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
			pub struct DurationTable < T >
		}
		
		DELEGATES: {
			impl trait Debug {
				[fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>]
			}
		}
	}
}

