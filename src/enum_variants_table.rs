#[macro_export]
macro_rules! enum_variants_table {
	//------------------------------------------------------------------------------------------------------------------
	// Table + Generated enum
	(
		ENUM_OUT: {
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
				$(
					$( #[$var_meta: meta] )*
					$var_ident: ident $( = $var_int: literal )?
			    ),*
			    $(,)?
		    }
	    }
		
		TABLE: {
			$( #[$table_meta: meta] )*
			$table_vis: vis struct $table_ident: ident < $gen: ident > $(;)? $({})?
		}
	) => {
		$crate::enum_variants_table! {
			ENUM_IN: $enum_ident;
			
			TABLE: {
				$( #[$table_meta] )*
				$table_vis struct $table_ident < $gen > {
				    $( $var_ident ),*
			    }
			}
		}
		
		$( #[$enum_meta] )*
		$enum_vis enum $enum_ident {
		    $(
		        $( #[$var_meta] )*
		        $var_ident $( = $var_int )?
		    ),*
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
		$crate::enum_variants_table! {
			$enum_ident {
				$( $var_ident ),*
			}
			
			$( #[$table_meta] )*
			$table_vis struct $table_ident< $gen > ;
		}
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Base impl
	(
		$enum_ident: ident {
			$( $var_ident: ident ),*
		}
		
		$( #[$table_meta: meta] )*
		$table_vis: vis struct $table_ident: ident < $gen: ident > $(;)? $({})?
	) => {
		#[allow(non_camel_case_types)]
		#[allow(non_snake_case)]
		$( #[$table_meta] )*
	    $table_vis struct $table_ident < $gen > {
	        $( $var_ident: $gen ),*
	    }
		
		macro_rules! table_from_const_fn {
			( | $$var: ident | $$( -> $$ret: ty )? $$closure: block ) => {{
				$table_ident {
					$( 
						$var_ident: { 
							let $$var = $enum_ident::$var_ident;
							$$closure
						} 
					),*
				}
			}};
			
			( |_| $$( -> $$ret: ty )? $$closure: block ) => {{
				$table_ident {
					$( $var_ident: $$closure ),*
				}
			}};
		}
		
		pub(crate) use table_from_const_fn;
		
		#[doc(hidden)]
		type _Enum = $enum_ident;
		
		#[doc(hidden)]
		pub mod new_fn {
			use super::{$table_ident, _Enum};
			
			impl< $gen > $table_ident< $gen > {
				#[allow(non_snake_case)]
				pub const fn new( $( $var_ident: $gen ),* ) -> Self {
			        Self {
			            $( $var_ident ),*
			        }
			    }
				
				#[allow(non_snake_case)]
				pub fn from_closure(f: impl Fn(_Enum) -> $gen) -> Self {
					Self {
						$( $var_ident: f(_Enum::$var_ident) ),*
					}
				}
			}
		}
		
		#[doc(hidden)]
		const TABLE_LENGTH: usize = ${count($var_ident)};
		
		impl<$gen> $table_ident<$gen> {
			pub const LENGTH: usize = TABLE_LENGTH;
			
			pub const fn get(&self, var: $enum_ident) -> & $gen {
			    match var {
			        $( $enum_ident::$var_ident => &self.$var_ident ),*    
			    }
		    }
			    
			pub fn get_mut(&mut self, var: $enum_ident) -> &mut $gen {
			    match var {
			        $( $enum_ident::$var_ident => &mut self.$var_ident ),*    
			    }
		    }
			
			#[allow(clippy::needless_lifetimes)]
			pub fn iter<'a>(&'a self) -> impl Iterator<Item = ($enum_ident, &'a $gen)> {
				[$( ($enum_ident::$var_ident, &self.$var_ident) ),* ].into_iter()
			}
			
			#[allow(clippy::needless_lifetimes)]
			pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = ($enum_ident, &'a mut $gen)> {
				[$( ($enum_ident::$var_ident, &mut self.$var_ident) ),* ].into_iter()
			}
		}
		
		impl< $gen > IntoIterator for $table_ident< $gen > {
		    type Item = ($enum_ident, $gen);
			type IntoIter = core::array::IntoIter< ($enum_ident, $gen), { TABLE_LENGTH }>;
	
			fn into_iter(self) -> Self::IntoIter {
				[ $( ($enum_ident::$var_ident, self.$var_ident) ),* ].into_iter()
			}
	    }
		
		impl< $gen > std::ops::Index<$enum_ident> for $table_ident< $gen > {
			type Output = $gen;
			
			fn index(&self, index: $enum_ident) -> &Self::Output {
				self.get(index)
			}
		}
		
		impl< $gen > std::ops::IndexMut<$enum_ident> for $table_ident< $gen > {
			fn index_mut(&mut self, index: $enum_ident) -> &mut Self::Output {
				self.get_mut(index)
			}
		}
	};
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::enum_variants_table;

	enum_variants_table! {
		ENUM_OUT: {
			#[derive(Debug, Clone)]
			pub enum Duration {
				Seconds = 1,
				DaysSeconds = 2,
				HoursMinutes = 3,
				Infinite = 4,
			}
		}
		
		TABLE: { 
			// Name of the module that will contain all the generated code
			#[derive(Debug, Clone)] // Attributes to apply on the Table
			pub struct DurationTable < Val >
		}
	}
	
	const fn test() {
		let table: DurationTable<i32> = table_from_const_fn!(|v| -> i32 { v as i32 });
	}
}