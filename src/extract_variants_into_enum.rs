/// # Extracts all variants of the enum and creates a new enum with the extracted variants
///
/// ## Notes
/// - Generated variants inherit the visibility of the enum
/// - The fields of generated variants are all public
/// - The `#[vars]` attribute is not optional, leave it empty if you don't need it: `#[vars()]`
/// - Attributes on the input enum are optional
/// - Attributes on individual variants are optional
/// - Since this macro generates an enum with the same name as the input enum, 
///   the input enum should be merely a template, it should not be defined outside this macro
/// 
/// # Example:
/// 
/// ```
/// use declarative_type_state::extract_variants_into_enum;
/// 
/// extract_variants_into_enum! {
///     #[vars(derive(Debug, Clone))] /// REQUIRED: Attributes to apply on all variants
///     #[derive(Debug)] // OPTIONAL
///     pub enum Num {
///         #[derive(PartialEq)] // OPTIONAL
///         Int { field: i32 },      
///         UInt { x: i32, y: i32 },
///         Float(f32, i32),
///         Bool(bool),
///         Test,
///     }
/// }
/// ```
/// 
/// ### Which generates:
/// ```
/// #[derive(Debug, Clone, PartialEq)] pub struct Int { pub field: i32 }
/// #[derive(Debug, Clone)] pub struct UInt { pub x: i32, pub y: i32 }
/// #[derive(Debug, Clone)] pub struct Float(pub f32, pub i32);
/// #[derive(Debug, Clone)] pub struct Bool(pub bool);
/// #[derive(Debug, Clone)] pub struct Test;
///
/// #[derive(Debug)]
/// pub enum Num {
///     Int(Int),
///     UInt(UInt),
///     Float(Float),
///     Bool(Bool),
///     Test,
/// }
///
/// // Repeat for UInt, Float, Bool, Test
/// impl From<Int> for Num { 
///     fn from(value: Int) -> Self { 
///         Num::Int(value) 
///     } 
/// }
///
/// impl TryFrom<Num> for Int {
///     type Error = Num;
///        
///     fn try_from(value: Num) -> Result<Self, Self::Error> {
///         if let Num::Int(var) = value {
///                Ok(var)
///         } else {
///             Err(value)
///         }   
///     }
/// }
/// ```
#[macro_export]
macro_rules! extract_variants_into_enum {
	//------------------------------------------------------------------------------------------------------------------
	// Generated Enum
	(
		#[vars( $( $all_meta: meta ),* $(,)? )]
		$( #[$enum_meta: meta] )*
		$enum_vis: vis enum $enum_ident: ident
		$( <[ $( $enum_gen: tt )* ]> )?
		$( [where $( $enum_bound: tt )* ] )?
		{
		    $(
		        $( [@ $ignore: ident] )?
		        $( #[$var_meta: meta] )*
		        $var_ident: ident 
		        $( <[ $( $var_gen: tt )* ]> )?
		        $( ( $($var_tuple: tt)* ) )? 
		        $( { $($var_fields: tt)* } )?
		        $( [where $( $var_bound: tt )* ] )?
		    ),*
		    $(,)?
	    }
    ) => {
		$( #[$enum_meta] )*
	    $enum_vis enum $enum_ident 
		$( <$( $enum_gen )* > )?
		$( where $( $enum_bound )* )?
		{
		    $( 
		        $var_ident( 
			        $var_ident $( <$( $var_gen )*> )? 
		        ) 
		    ),*
	    }
		
		$crate::extract_variants! {
			#[vars( $( $all_meta ),* )]
		    $( #[$enum_meta] )*
		    $enum_vis enum $enum_ident
			$( <[ $( $enum_gen )* ]> )?
			$( [where $( $enum_bound )* ] )?
			{
			    $(
			        $( [@ $ignore ] )?
			        $( #[$var_meta] )*
			        $var_ident 
			        $( <[ $( $var_gen )* ]> )?
			        $( ( $( $var_tuple )* ) )? 
			        $( { $( $var_fields )* } )?
			        $( [where $( $var_bound )* ] )?
			    ),*
		    }
		}
		
		$crate::enum_variants_convert! {
		    $enum_vis enum $enum_ident
		    $( <[ $( $enum_gen )* ]> )?
			$( [where $( $enum_bound )* ] )?
		    {
			    $( $var_ident ( $var_ident $( <$( $var_gen )*> )? ) ),*
		    }
	    }
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// Generated Enum + Delegates
	(
		ENUM_OUT: {
			#[vars( $( $all_meta: meta ),* $(,)? )]
			$( #[$enum_meta: meta] )*
			$enum_vis: vis enum $enum_ident: ident
			$( <[ $( $enum_gen: tt )* ]> )?
			$( [where $( $enum_bound: tt )* ] )?
			{
			    $(
			        $( [@ $ignore: ident] )?
			        $( #[$var_meta: meta] )*
			        $var_ident: ident 
			        $( <[ $( $var_gen: tt )* ]> )?
			        $( ( $($var_tuple: tt)* ) )? 
			        $( { $($var_fields: tt)* } )?
			        $( [where $( $var_bound: tt )* ] )?
			    ),*
			    $(,)?
		    }
		}
		
		DELEGATES: {
		    $(
		        impl $( <[ $( $trait_gen: tt )*  ]> )? 
		        trait $trait_ty: path
		        $( [where $( $trait_bound: tt )* ] )?
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
		$crate::extract_variants_into_enum! {
			#[vars( $( $all_meta ),* )]
			$( #[$enum_meta] )*
			$enum_vis enum $enum_ident
			$( <[ $( $enum_gen )* ]> )?
			$( [where $( $enum_bound )* ] )?
			{
			    $(
			        $( [@ $ignore ] )?
			        $( #[$var_meta] )*
			        $var_ident 
			        $( <[ $( $var_gen )* ]> )?
			        $( ( $( $var_tuple )* ) )? 
			        $( { $( $var_fields )* } )?
			        $( [where $( $var_bound )* ] )?
			    ),*
		    }
		}
		
		$crate::enum_delegate_impls! {
			ENUM_IN: {
				$enum_ident
				$( <[ $( $enum_gen )* ]> )?
				$( [where $( $enum_bound )* ] )?
				{
				    $( $var_ident ( $var_ident $( <$( $var_gen )*> )? ) ),*
			    }
			}
			
			DELEGATES: {
			    $(
			        impl $( <[ $( $trait_gen )*  ]> )? 
				    trait $trait_ty
			        $( [where $( $trait_bound )* ] )?
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
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::extract_variants_into_enum;

	#[derive(Debug)]
	pub struct Bool(bool);

	extract_variants_into_enum! {
		#[vars(derive(Debug, Clone))] // REQUIRED: Attributes to apply on all variants
		#[derive(Debug)]              // IGNORED - OPTIONAL
		pub enum Num {                // Enum name: IGNORED - OPTIONAL
			#[derive(PartialEq)]      // OPTIONAL
			Int { field: i32 },      
			UInt { x: i32, y: i32 },
			Float(f32, i32),
			[@SKIP]
			Bool(bool),
			Test,
		}
	}
}

#[allow(unused)]
#[cfg(test)]
mod test_generics {
	use crate::extract_variants_into_enum;

	#[derive(Debug)]
	pub struct Bool(bool);
	
	extract_variants_into_enum! {
		#[vars(derive(Debug, Clone))]
		#[derive(Debug)]
		pub enum Num<['a, T]> [where T: Clone] {
			#[derive(PartialEq)]
			Int <['a, T]> { field: &'a T } [where T: Clone],
			UInt { x: i32, y: i32 },
			Float(f32, i32),
			[@SKIP]
			Bool(bool),
			Test,
		}
	}
}

#[allow(unused)]
#[cfg(test)]
mod test_generics_with_delegates {
	use crate::extract_variants_into_enum;

	#[derive(Debug)]
	pub struct Bool(bool);

	extract_variants_into_enum! {
		ENUM_OUT: { 
			#[vars(derive(Debug, Clone))]
			#[derive(Debug)]
			pub enum Num<['a, T]> [where T: Clone] {
				#[derive(PartialEq)]
				Int <['a, T]> { field: &'a T } [where T: Clone],
				UInt { x: i32, y: i32 },
				Float(f32, i32),
				[@SKIP]
				Bool(bool),
				Test,
			}
		}
		
		DELEGATES: {
			impl<['a, T]> trait MyTrait [where T: Clone] {
				[fn print(&self)]
			}
		}
	}
	
	trait MyTrait {
		fn print(&self);
	}

	impl<'a, T: Clone> MyTrait for Int<'a, T> {
		fn print(&self) {
			todo!()
		}
	}
	
	impl MyTrait for UInt {
		fn print(&self) {
			todo!()
		}
	}

	impl MyTrait for Float {
		fn print(&self) {
			todo!()
		}
	}

	impl MyTrait for Bool {
		fn print(&self) {
			todo!()
		}
	}

	impl MyTrait for Test {
		fn print(&self) {
			todo!()
		}
	}

	#[allow(clippy::needless_lifetimes)]
	fn test<'a, T: Clone>(input: Num<'a, T>) {
		input.print();
	}
}

#[allow(unused)]
#[cfg(test)]
mod test_generics_with_delegates_trait_generics {
	use crate::extract_variants_into_enum;

	#[derive(Debug)]
	pub struct Bool(bool);

	extract_variants_into_enum! {
		ENUM_OUT: { 
			#[vars(derive(Debug, Clone))]
			#[derive(Debug)]
			pub enum Num<['a, T]> [where T: Clone] {
				#[derive(PartialEq)]
				Int <['a, T]> { field: &'a T } [where T: Clone],
				UInt { x: i32, y: i32 },
				Float(f32, i32),
				[@SKIP]
				Bool(bool),
				Test,
			}
		}
		
		DELEGATES: {
			impl<['a, T, G]> trait MyTrait<G> [where T: Clone] {
				[fn print(&self, d: G)]
			}
		}
	}

	trait MyTrait<G> {
		fn print(&self, d: G);
	}

	impl<'a, T: Clone, G> MyTrait<G> for Int<'a, T> {
		fn print(&self, d: G) {
			todo!()
		}
	}

	impl<G> MyTrait<G> for UInt {
		fn print(&self, d: G) {
			todo!()
		}
	}

	impl<G> MyTrait<G> for Float {
		fn print(&self, d: G) {
			todo!()
		}
	}

	impl<G> MyTrait<G> for Bool {
		fn print(&self, d: G) {
			todo!()
		}
	}

	impl<G> MyTrait<G> for Test {
		fn print(&self, d: G) {
			todo!()
		}
	}

	#[allow(clippy::needless_lifetimes)]
	fn test<'a, T: Clone>(input: Num<'a, T>) {
		input.print(0.0);
	}
}