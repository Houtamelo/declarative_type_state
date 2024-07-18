#[doc(hidden)]
#[macro_export]
macro_rules! extract_single_variant {
	//------------------------------------------------------------------------------------------------------------------
	// Tuple
    (
		{ $( #[$all_meta: meta] )* }
		$( #[$var_meta: meta] )*
		$var_vis: vis $var_ident: ident ( 
			$( $field_ty: ty ),* 
			$(,)? 
		)
	) => {
		$( #[$all_meta] )*
		$( #[$var_meta] )*
		$var_vis struct $var_ident { pub t: ($( $field_ty, )*) }
	    
	    impl std::ops::Deref for $var_ident {
			type Target = ( $( $field_ty, )* );
			
			fn deref(&self) -> &Self::Target {
				&self.t
			}
		}
	    
		impl std::ops::DerefMut for $var_ident {
			fn deref_mut(&mut self) -> &mut Self::Target {
				&mut self.t
			}
		}
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Struct
	(
		{ $( #[$all_meta: meta] )* }
		$( #[$var_meta: meta] )*
		$var_vis: vis $var_ident: ident {
			$( $field_name: ident : $field_ty: ty ),*
			$(,)?
		}
	) => {
		$( #[$all_meta] )*
		$( #[$var_meta] )*
		$var_vis struct $var_ident {
			$( pub $field_name: $field_ty, )*
		}
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Unit
	(
		{ $( #[$all_meta: meta] )* }
		$( #[$var_meta: meta] )*
		$var_vis: vis $var_ident: ident
	) => {
		$( #[$all_meta] )*
		$( #[$var_meta] )*
		$var_vis struct $var_ident;
	};
}