/// # Implements for each variant's type:
/// - From<Variant> for Enum
/// - TryFrom<Enum> for Variant
///
/// # Input
/// - `ENUM`: Defines the input enum with its visibility, name, generics, where clauses, and variants.
///
/// ## ENUM:
///
/// ```pseudo
/// [enum_vis] enum [name]<[generics]> [where [bounds]] {
///     [var_name_A]([var_type_A]),
///     [var_name_B]([var_type_B]),
/// }
/// ```
///
/// - `[enum_vis]`: Visibility level of the enum. (e.g., `pub`)
/// - `[name]`: Identifier (name) of the enum. (e.g., `MyEnum`)
/// - `[generics]`: Optional generics for the enum, must be placed inside brackets. (e.g., `<[T]>`)
/// - `where [bounds]`: Optional where clause for the enum, must be placed inside brackets. (e.g., `where [T: SomeTrait]`)
/// - `[var_name]([var_type])`: Variants of the enum along with their types. (e.g., `VariantOne(TypeOne), VariantTwo(TypeTwo)`)
///
/// # Example
///
/// ```rust
/// use declarative_type_state::enum_variants_convert;
///
/// #[derive(Debug, PartialEq, Eq)]
/// pub enum Integer {
///     Int(i32),
///     UInt(u32),
///     Long(i64),
///     ULong(u64),
/// }
///
/// enum_variants_convert! {
///     enum Integer {
///         Int(i32),
///         UInt(u32),
///         Long(i64),
///         ULong(u64),
///     }
/// }
///
/// let integer = Integer::from(5_i32);
/// assert_eq!(integer, Integer::Int(5));
/// assert_eq!(Ok(5_i32), i32::try_from(integer));
/// ```
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
		
		impl<$( $generic )*> $crate::FromEnum<$enum_ident<$( $generic )*>> for $var_ty where $( $bound )* {
			fn from_enum(value: $enum_ident<$( $generic )*>) -> Option<Self> {
				if let $enum_ident::$var_ident(var) = value {
					Some(var)
				} else {
					None
				}
			}
		}
		
		impl<'__a, $( $generic )*> $crate::FromEnum<&'__a $enum_ident<$( $generic )*>> for &'__a $var_ty where $( $bound )* {
			fn from_enum(value: &'__a $enum_ident<$( $generic )*>) -> Option<Self> {
				if let $enum_ident::$var_ident(var) = value {
					Some(var)
				} else {
					None
				}
			}
		}
		
		impl<'__a, $( $generic )*> $crate::FromEnum<&'__a mut $enum_ident<$( $generic )*>> for &'__a mut $var_ty where $( $bound )* {
			fn from_enum(value: &'__a mut $enum_ident<$( $generic )*>) -> Option<Self> {
				if let $enum_ident::$var_ident(var) = value {
					Some(var)
				} else {
					None
				}
			}
		}
	};
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::Is;

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
	
	fn test(input: Num) {
		if input.is::<i32>() {
			println!("is i32");
		}
	}
}

#[allow(unused)]
#[cfg(test)]
mod test_generics {
	use std::marker::PhantomData;
	use crate::Is;

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
	
	fn test<T>(input: Num<T>) {
		if input.is::<u32>() {
			print!("is u32");
		} else if input.is::<Wrapper<T>>() {
			print!("is Wrapper");
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