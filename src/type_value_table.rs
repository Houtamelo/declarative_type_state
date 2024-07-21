#[macro_export]
macro_rules! type_value_table {
	//------------------------------------------------------------------------------------------------------------------
	// Table + Generated enum
	(
		ENUM: {
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
			$table_vis: vis struct $table_ident: ident <$T: ty> $(;)? $({})?
		}
	) => {
		$crate::type_value_table! {
			ENUM: $enum_ident;
			
			TABLE: {
				$( #[$table_meta] )*
				$table_vis struct $table_ident <$T> {
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
	// Table + User enum + Delegates
	(
		ENUM: {
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
			$table_vis: vis struct $table_ident: ident <$T: ty> $(;)? $({})?
		}
		
		DELEGATES: {
		    $(
		        $trait_vis: vis trait $trait_ident: ident $( < [ $( $gens: tt )* ] > )? {
				    $( [ $( $item: tt )* ] )*
			    }
		    )*
	    }
	) => {
		$crate::type_value_table! {
			ENUM: {
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
				$table_vis struct $table_ident <$T>
			}
		}
		
		$crate::enum_delegate_impls! {
			ENUM: {
				$enum_ident {
					$( $var_ident ( $var_ident ) ),*
			    }
		    }
			
			DELEGATES: {
			    $(
			        $trait_vis trait $trait_ident $( < [ $( $gens )* ] > )? {
					    $( [ $( $item )* ] )*
				    }
			    )*
		    }
		}
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Table + User enum
	(
		ENUM: $enum_ident: ident $(;)? $({})?
		
		TABLE: {
			$( #[$table_meta: meta] )*
			$table_vis: vis struct $table_ident: ident <$T: ty> {
			    $( $var_ident: ident ),*
			    $(,)?
		    }
		}
	) => {
		$crate::type_value_table! {
			$( #[$table_meta] )*
			$table_vis struct $table_ident <$T> {
			    $( $var_ident ),*
		    }
		}
		
		impl IntoIterator for $table_ident {
		    type Item = $T;
			type IntoIter = core::array::IntoIter<$T, { $table_ident::LENGTH }>;
	
			fn into_iter(self) -> Self::IntoIter {
				[ $( self.$var_ident ),* ]
				.into_iter()
			}
	    }
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Base impl
	(
		$( #[$table_meta: meta] )*
		$table_vis: vis struct $table_ident: ident <$T: ty> {
		    $( $var_ident: ident ),*
		    $(,)?
	    }
	) => {
		#[allow(non_camel_case_types)]
		#[allow(non_snake_case)]
		$( #[$table_meta] )*
	    $table_vis struct $table_ident {
	        $($var_ident: $T),*
	    }
		
		#[allow(non_camel_case_types)]
		$table_vis enum TypeRef<'a> {
		    $($var_ident(&'a $T)),*
	    }
	    
		#[allow(non_camel_case_types)]
	    $table_vis enum TypeRefMut<'a> {
		    $($var_ident(&'a mut $T)),*
	    }
		
		#[doc(hidden)]
		type _Inner = $T;
		
		#[doc(hidden)]
		pub mod new_fn {
			use super::{$table_ident, _Inner};
			
			impl $table_ident {
				#[allow(non_snake_case)]
				pub fn new( $( $var_ident: _Inner ),* ) -> Self {
			        Self {
			            $($var_ident),*
			        }
			    }
			}
		}
		
		impl $table_ident {
			#[doc(hidden)]
			const fn ignore<T: ?Sized>() {}
			
			pub const LENGTH: usize = {
		        let mut count = 0;
		        $( 
		            { 
			            Self::ignore::<$var_ident>();
		            }
		            
		            count += 1;
		        )*
		        
		        count
	        };
			
			pub fn get<T: GetInTable>(&self) -> & $T {
			    T::get_in_table(self)
		    }
			    
			pub fn get_mut<T: GetInTable>(&mut self) -> &mut $T {
			    T::get_in_table_mut(self)
		    }
			
			#[allow(clippy::needless_lifetimes)]
			pub fn iter<'a>(&'a self) -> impl Iterator<Item = TypeRef<'a>> {
				[
					$(
						TypeRef::$var_ident(&self.$var_ident)
					),*
				].into_iter()
			}
			
			#[allow(clippy::needless_lifetimes)]
			pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = TypeRefMut<'a>> {
				[
					$(
						TypeRefMut::$var_ident(&mut self.$var_ident)
					),*
				].into_iter()
			}
		}
		
		$table_vis trait GetInTable { 
		    fn get_in_table(table: & $table_ident) -> & $T;
	        fn get_in_table_mut(table: &mut $table_ident) -> &mut $T;
	    }
		    
	    $(
		    impl GetInTable for $var_ident {
	            fn get_in_table(table: & $table_ident) -> & $T {
	                &table.$var_ident
	            }
		        
	            fn get_in_table_mut(table: &mut $table_ident) -> &mut $T {
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
		ENUM: {
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
			pub struct DurationTable < i32 >
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
		ENUM: {
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
			pub struct DurationTable <Sealed>
		}
		
		DELEGATES: {
			trait Debug {
				[fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>]
			}
		}
	}
}

