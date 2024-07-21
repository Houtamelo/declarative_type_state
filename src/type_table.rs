/// # Generates a collection type that contains a single value of each type
/// The types must be unique!
///
/// For a combination of this and [variant_types](crate::extract_variants_into_enum), see [variant_types_table](crate::variant_types_table)
///
/// # Usage
/// ```
/// mod table {
/// 
/// use declarative_type_state::type_table;
///
/// type_table! {
///     pub struct DurationTable {
///         seconds: f32,
///         hours: i32,
///         infinite: (),
///     }
/// }
///
/// }
/// 
/// // The values can be accessed by using `get::<T>` or `get_mut::<T>`:
/// let mut table = table::DurationTable::new(2.0, 5, ());
///
/// let seconds = table.get::<f32>();
/// let hours = table.get_mut::<i32>();
/// ```
///
/// ## The Table also implements:
/// - fn iter(&self) -> Iterator<Item = Ref<T>>
/// - fn iter_mut(&mut self) -> Iterator<Item = RefMut<T>>
/// - fn into_iter(self) -> Iterator<Item = Owned<T>>
///
/// # Example
///
/// ```
/// mod table {
///
/// declarative_type_state::type_table! {
///     #[derive(Debug, Clone)]
///     pub struct DurationTable {
///         seconds: f32,
///         hours: i32,
///         infinite: (),
///     }
/// }
/// 
/// }
/// ```
#[macro_export]
macro_rules! type_table {
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
			$table_vis: vis struct $table_ident: ident $(;)? $({})?
		}
	) => {
		$crate::type_table! {
			ENUM: $enum_ident;
			
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
			$table_vis: vis struct $table_ident: ident $(;)? $({})?
		}
		
		DELEGATES: {
		    $(
		        trait $trait_ident: ident $( < [ $( $gens: tt )* ] > )? {
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
				$table_vis struct $table_ident
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
				    trait $trait_ident $( < [ $( $gens )* ] > )? {
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
		ENUM: $enum_ident: ident $(;)? $({})?
		
		TABLE: {
			$( #[$table_meta: meta] )*
			$table_vis: vis struct $table_ident: ident {
			    $( $var_ident: ident: $var_ty: ty ),*
			    $(,)?
		    }
		}
	) => {
		$crate::type_table! {
			$( #[$table_meta] )*
			$table_vis struct $table_ident {
			    $( $var_ident: $var_ty ),*
		    }
		}
		
		impl IntoIterator for $table_ident {
		    type Item = $enum_ident;
			type IntoIter = core::array::IntoIter<$enum_ident, { $table_ident::LENGTH }>;
	
			fn into_iter(self) -> Self::IntoIter {
				[ 
					$( $enum_ident::$var_ident(self.$var_ident) ),* 
				].into_iter()
			}
	    }
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Base impl
	(
		$( #[$table_meta: meta] )*
		$table_vis: vis struct $table_ident: ident {
		    $( $var_ident: ident: $var_ty: ty ),*
		    $(,)?
	    }
	) => {
		#[allow(non_camel_case_types)]
		#[allow(non_snake_case)]
		$( #[$table_meta] )*
	    $table_vis struct $table_ident {
	        $($var_ident: $var_ty),*
	    }
		
		#[allow(non_camel_case_types)]
		$table_vis enum TypeRef<'a> {
		    $($var_ident(&'a $var_ty)),*
	    }
	    
		#[allow(non_camel_case_types)]
	    $table_vis enum TypeRefMut<'a> {
		    $($var_ident(&'a mut $var_ty)),*
	    }
		
		#[doc(hidden)]
		#[allow(non_snake_case)]
		pub mod new_fn {
			use super::*;
			use super::$table_ident;
			
			$(
				#[doc(hidden)]
				mod $var_ident {
					use super::super::*;
					pub type Ty = $var_ty;
				} 
			)*
			
			pub mod inner {
				use super::$table_ident;
				
				impl $table_ident {
					#[allow(non_snake_case)]
					pub fn new( $( $var_ident: super::$var_ident::Ty ),* ) -> Self {
				        Self {
				            $($var_ident),*
				        }
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
			            Self::ignore::<$var_ty>();
		            }
		            
		            count += 1;
		        )*
		        
		        count
	        };
			
			pub fn get<T: GetInTable>(&self) -> &T {
			    T::get_in_table(self)
		    }
			    
			pub fn get_mut<T: GetInTable>(&mut self) -> &mut T {
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
		    fn get_in_table(table: & $table_ident) -> &Self;
	        fn get_in_table_mut(table: &mut $table_ident) -> &mut Self;
	    }
		    
	    $(
		    impl GetInTable for $var_ty {
	            fn get_in_table(table: & $table_ident) -> &Self {
	                &table.$var_ident
	            }
		        
	            fn get_in_table_mut(table: &mut $table_ident) -> &mut Self {
	                &mut table.$var_ident
	            }
	        }
	    )*
	};
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::type_table;

	type_table! {
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
	use crate::type_table;
	use std::fmt::Debug;
	use std::fmt::Formatter;

	type_table! {
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
			pub struct DurationTable;
		}
		
		DELEGATES: {
			trait Debug {
				[fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>]
			}
		}
	}
}