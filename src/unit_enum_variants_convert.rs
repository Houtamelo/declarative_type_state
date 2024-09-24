#[macro_export]
macro_rules! unit_enum_variants_convert {
	//------------------------------------------------------------------------------------------------------------------
	// User input
	(   
		$enum_vis: vis enum $enum_ident: ident {
		    $($var_ident: ident),*
		    $(,)?
	    }
    ) => {
		impl $enum_ident {
			pub fn into_variant<Variant>(self) -> Option<Variant> where Variant: $crate::FromEnum<Self> {
				Variant::from_enum(self)
			}
			
			pub fn is<Variant>(&self) -> bool where Variant: $crate::FromEnum<Self> {
				self.into_variant::<Variant>().is_some()
			}
		}
		
		$(
			impl From<$var_ident> for $enum_ident {
				fn from(value: $var_ident) -> Self {
					Self::$var_ident
				}
			}
			
			impl TryFrom<$enum_ident> for $var_ident {
				type Error = ();
			
				fn try_from(value: $enum_ident) -> Result<Self, Self::Error> {
					if let $enum_ident::$var_ident = value {
						Ok($var_ident)
					} else {
						Err(())
					}
				}
			}
			
			impl $crate::FromEnum<$enum_ident> for $var_ident {
				fn from_enum(value: $enum_ident) -> Option<Self> {
					Self::try_from(value).ok()
				}
			}
		)*
    };
}

#[allow(unused)]
#[cfg(test)]
mod tests {
	use crate::{FromEnum, FromEnumMut, FromEnumRef};

	#[derive(Debug, PartialEq, Eq, Copy, Clone)]
	struct Int;
	#[derive(Debug, PartialEq, Eq, Copy, Clone)]
	struct UInt;
	#[derive(Debug, PartialEq, Eq, Copy, Clone)]
	struct Bool;

	#[derive(Debug, PartialEq, Eq, Copy, Clone)]
	pub enum Num {
		Int,
		UInt,
		Bool,
	}

	unit_enum_variants_convert! {
	     // Attributes not required
	     enum Num { // visibility not required
	         Int,
	         UInt,
	         Bool,
	     }
	}
	
	fn test(mut input: Num) {
		if input.is::<Int>() {
			println!("is i32");
		}
		else if let Some(var) = input.into_variant::<Bool>() {
			println!("is bool: {var:?}");
		}
		else if let Some(var) = input.into_variant::<UInt>() {
			println!("is u32: {var:?}");
		}
		else if let Some(var) = input.into_variant::<Int>() {
			println!("is i32: {var:?}");
		}
	}
}