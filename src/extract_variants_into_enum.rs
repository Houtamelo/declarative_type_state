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
		$enum_vis: vis enum $enum_ident: ident {
		    $(
		        $( #[$var_meta: meta] )*
		        $var_ident: ident $( ( $($var_tuple: tt)* ) )? $( { $($var_fields: tt)* } )?
		    ),*
		    $(,)?
	    }
    ) => {
		$( #[$enum_meta] )*
	    $enum_vis enum $enum_ident {
		    $( $var_ident ($var_ident) ),*
	    }
		
		$crate::extract_variants! {
			#[vars( $( $all_meta ),* )]
		    $( #[$enum_meta] )* 
		    $enum_vis enum $enum_ident {
			    $(
			        $( #[$var_meta] )*
			        $var_ident $( ( $($var_tuple)* ) )? $( { $($var_fields)* } )?
			    ),*
		    }
		}
		
		$crate::enum_variants_convert! {
		    $enum_vis enum $enum_ident {
			    $( $var_ident ($var_ident) ),*
		    }
	    }
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// Generated Enum + Delegates
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
		$crate::extract_variants_into_enum! {
			#[vars( $( $all_meta ),* $(,)? )]
			$( #[$enum_meta] )*
			$enum_vis enum $enum_ident {
			    $(
			        $( #[$var_meta] )*
			        $var_ident $( ( $($var_tuple)* ) )? $( { $($var_fields)* } )?
			    ),*
		    }
		}
		
		$crate::enum_delegate_impls! {
			ENUM_IN: {
				$enum_vis enum $enum_ident {
				    $( $var_ident ($var_ident) ),*
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
}