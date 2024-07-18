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
	(
	    $enum_vis: vis enum $enum_ident: ident {
		    $( $var_ident: ident ($var_ty: ty) ),*
		    $(,)?
	    }
    ) => {
		$(
			impl From<$var_ty> for $enum_ident {
				fn from(value: $var_ty) -> Self {
					$enum_ident::$var_ident(value)
				}
			}
			
			impl TryFrom<$enum_ident> for $var_ty {
				type Error = $enum_ident;
			
				fn try_from(value: $enum_ident) -> Result<Self, Self::Error> {
					if let $enum_ident::$var_ident(var) = value {
						Ok(var)
					} else {
						Err(value)
					}
				}
			}
		)*
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