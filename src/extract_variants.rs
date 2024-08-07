/// # Extracts all variants of input enum
///
/// ## Notes
/// - Generated variants inherit the visibility of the enum
/// - The fields of generated variants are all public
/// - This macro only parses the input enum, it does not generate the enum definition
///
/// ---
///
/// # Full Example
///
/// ```
/// use declarative_type_state::extract_variants;
///
/// extract_variants! {
///     #[vars(derive(Debug, Clone))] // REQUIRED: Attributes to apply on all variants
///     #[derive(Debug)]              // IGNORED - OPTIONAL
///     pub enum Num {                // Enum name: IGNORED - OPTIONAL
///         #[derive(PartialEq)]      // OPTIONAL
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
/// ```
///
/// ---
///
/// # Human-Readable Input Structure
///
/// ```ignore
/// #[vars( [variants_attributes] )]
/// #[enum_attributes]
/// [visibility] enum [name] {
///     [variants]
/// }
/// ```
///
/// ---
///
/// ## \[variants_attributes\] - Required
/// - Attributes to be applied on all extracted variants
/// - Separated by commas
/// - If you do not wish to apply any attributes, leave it as "#\[vars()\]"
///
/// ##### Example
/// ```ignore no_run
/// #[vars( derive(Clone), repr(transparent) )]
/// ```
///
/// will add `#[derive(Clone)]` and `#[repr(transparent)]`
/// to every generated variant:
///
/// ```
/// #[derive(Clone)]
/// #[repr(transparent)]
/// struct Var_1(i64);
///
/// #[derive(Clone)]
/// #[repr(transparent)]
/// struct Var_2 { pub value: f64 }
/// ```
///
/// ---
///
/// ## \[enum_attributes\] - Ignored
/// - Present only to facilitate usage
/// - Any number of attributes can be applied
/// - Syntax is the same as Rust's
///
/// ---
///
/// ## \[visibility\] - Optional
/// - Any kind of visibility token : `pub`, `pub(crate)`, .. 
/// - This visibility is applied to all variants
///
/// #### Example
/// ```ignore no_run
/// pub(in crate)
/// ```
///
/// will set the visibility of all variants to `pub(in crate)`:
///
/// ```
/// pub(in crate) struct Var_1 { pub t: (i32, i64) }
/// pub(in crate) struct Var_2 { pub x: f64, pub y: f64 }
/// ```
///
/// ---
///
/// ## \[name\] - Ignored
/// - Ignored by the macro
/// - Present only to facilitate usage
///
/// ---
///
/// ## \[variants\] - Required
/// - Any number of variants is supported
/// - Enum variant syntax is fully supported: "Variant, Variant(i32, ..), Variant { field: i32, .. }"
/// - You may add attributes to individual variants
///
/// #### Example
///
/// ```ignore no_run
/// Int { value: i32 },
/// UInt(u32, u64),
/// #[derive(Debug)]
/// Empty,
/// ```
///
/// Will generate:
/// ```
/// struct Int { pub value: i32 }
/// struct UInt(pub u32, pub u64);
/// #[derive(Debug)] struct Empty;
/// ```
#[macro_export]
macro_rules! extract_variants {
    (
	    #[vars( $( $all_meta: meta ),* $(,)? )]
		$( #[$enum_meta: meta] )*
		$enum_vis: vis enum $enum_ident: ident
	    $( <[ $( $generic: tt )* ]> )?  // Ignored
		$( [where $( $bound: tt )* ] )? // Ignored
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
	    $crate::extract_variants! {
		    @TOKENIZE 
		    { $( #[$all_meta] )* }
		    $( #[$enum_meta] )*
		    $enum_vis enum $enum_ident {
		        $(
		            $( [@ $ignore] )?
		            $( #[$var_meta] )*
		            $var_ident
		            <[ $( $( $var_gen )* )? ]>
		            $( ( $( $var_tuple )* ) )? 
		            $( { $( $var_fields )* } )?
		            [ $( $( $var_bound )* )? ]
		        ),*
		    }
	    }
    };
	
	(@TOKENIZE
		$all_meta_tt: tt
		$( #[$enum_meta: meta] )*
		$enum_vis: vis enum $enum_ident: ident {
		    $(
		        $( [@ $ignore: ident] )?
		        $( #[$var_meta: meta] )*
		        $var_ident: ident 
		        <[ $( $var_gen: tt )* ]>
		        $( ( $($var_tuple: tt)* ) )? 
		        $( { $($var_fields: tt)* } )?
		        [ $( $var_bound: tt )* ]
		    ),*
	    }
	) => {
		$(
			$crate::extract_single_variant! {
				$all_meta_tt
				$( [@ $ignore] )?
				$( #[$var_meta] )*
		        $enum_vis 
				$var_ident 
				{ $( $var_gen )* }
				$( ( $($var_tuple)* ) )? 
				$( { $($var_fields)* } )?
				{ $( $var_bound )* }
			}
		)*
	};
}


#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::extract_variants;

	extract_variants! {
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
	use crate::extract_variants;

	extract_variants! {
		#[vars(derive(Debug, Clone))]
		#[derive(Debug)]
		pub enum Num {
			#[derive(PartialEq)]
			Int <['a, T: Clone]> { field: &'a T },
			UInt { x: i32, y: i32 },
			Float(f32, i32),
			[@SKIP]
			Bool(bool),
			Test,
		}
	}
}