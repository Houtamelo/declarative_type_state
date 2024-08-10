/// # Given an enum, implements for each variant field's type:
/// - From<Variant> for Enum
/// - TryFrom<Enum> for Variant
///
/// This macro does not declare the enum, you should do it yourself outside its invocation
///
/// # Example
///
/// ```
/// #![feature(macro_metavar_expr)]
/// use declarative_type_state::enum_variants_convert;
///
/// #[derive(Debug, PartialEq, Eq)]
/// pub enum Num {
///     Int(i32),
///     UInt(u32),
///     Bool(bool),
///     Empty(()),
/// }
///
/// enum_variants_convert! {
///     // Attributes not required
///     enum Num { // visibility not required
///         Int(i32),
///         UInt(u32),
///         Bool(bool),
///         Empty(()),
///     }
/// }
///
/// // After invoking the macro, you should be able to seamlessly convert from a variant's type to the enum
/// let int = 5;
/// let num: Num = int.into();
/// assert_eq!(num, Num::Int(5));
/// assert_eq!(Ok(5), i32::try_from(num));
/// ```
///
/// # Restrictions
/// - All variants must be tuples with a single field
/// - All variants' type must be unique
#[macro_export]
macro_rules! enum_variants_convert {
	//------------------------------------------------------------------------------------------------------------------
	// User input
	(   
		$enum_vis: vis enum $enum_ident: ident
		$( <[ $( $enum_gen: tt )* ]> )?
		$( where [ $( $enum_bound: tt )* ] )?
		{
		    $( 
		        $var_ident: ident 
		        ( $var_ty: ty )
		    ),*
		    $(,)?
	    }
    ) => {
		$crate::enum_variants_convert! {
			@TOKENIZE 
			{ $( $( $enum_gen )* )? }
			{ $( $( $enum_bound )* )? }
			$enum_vis enum $enum_ident {
				$( $var_ident ( $var_ty ) ),*
			}
		}
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// Converting generics into single token tree
	(@TOKENIZE
		$enum_gens_tt: tt
		$enum_bounds_tt: tt
		$enum_vis: vis enum $enum_ident: ident {
		    $( $var_ident: ident ($var_ty: ty) ),*
		    $(,)?
	    }
	) => {
		$(
			$crate::enum_variants_convert! { 
				@SINGLE
				$enum_gens_tt
				$enum_bounds_tt
				$enum_vis
				$enum_ident
				$var_ident
				$var_ty
			}
		)*
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Using single token tree on each variant
	(@SINGLE
		{ $( $generic: tt )* }
		{ $( $bound: tt )* }
		$enum_vis: vis
		$enum_ident: ident
		$var_ident: ident
		$var_ty: ty
	) => {
		impl<$( $generic )*> From<$var_ty> for $enum_ident<$( $generic )*> where $( $bound )* {
			fn from(value: $var_ty) -> Self {
				Self::$var_ident(value)
			}
		}
		
		impl<$( $generic )*> TryFrom<$enum_ident<$( $generic )*>> for $var_ty where $( $bound )* {
			type Error = $enum_ident<$( $generic )*>;
		
			fn try_from(value: $enum_ident<$( $generic )*>) -> Result<Self, Self::Error> {
				if let $enum_ident::$var_ident(var) = value {
					Ok(var)
				} else {
					Err(value)
				}
			}
		}
	};
}


#[cfg(test)]
mod tests {
	#[derive(Debug, PartialEq, Eq)]
	pub enum Num {
		Int(i32),
		UInt(u32),
		Bool(bool),
		Empty(()),
	}

	enum_variants_convert! {
	     // Attributes not required
	     enum Num { // visibility not required
	         Int(i32),
	         UInt(u32),
	         Bool(bool),
	         Empty(()),
	     }
	}
}

#[cfg(test)]
mod test_generics {
	use std::marker::PhantomData;

	#[derive(Debug, PartialEq, Eq)]
	pub enum Num<'a, T> {
		Int(i32),
		UInt(u32),
		Bool(bool),
		Empty(Wrapper<'a, T>),
	}

	#[derive(Debug, PartialEq, Eq)]
	pub struct Wrapper<'a, T>(PhantomData<&'a T>);

	enum_variants_convert! {
	     // Attributes not required
	     enum Num<['a, T]> { // visibility not required
	         Int(i32),
	         UInt(u32),
	         Bool(bool),
	         Empty(Wrapper<'a, T>),
	     }
	}
}

#[cfg(test)]
mod test_many_generics {
	use std::marker::PhantomData;

	#[derive(Debug, PartialEq, Eq)]
	pub enum Num<'a, 'b, T1, T2> where T1: Clone {
		Int(i32),
		UInt(u32),
		Bool(bool),
		Empty(Wrapper<'a, 'b, T1, T2>),
	}

	#[derive(Debug, PartialEq, Eq)]
	pub struct Wrapper<'a, 'b, T1, T2>(PhantomData<(&'a T1, &'b T2)>);

	enum_variants_convert! {
	     // Attributes not required
	     enum Num<['a, 'b, T1, T2]> where [ T1: Clone] { // visibility not required
	         Int(i32),
	         UInt(u32),
	         Bool(bool),
	         Empty(Wrapper<'a, 'b, T1, T2>),
	     }
	}
}