#[doc(hidden)]
#[macro_export]
macro_rules! extract_single_variant {
	//------------------------------------------------------------------------------------------------------------------
	// Tuple
    (
		{ $( #[$all_meta: meta] )* }
		$( #[$var_meta: meta] )*
		$var_vis: vis 
		$var_ident: ident
		{ $( $generic: tt )* }
		( 
			$( $field_ty: ty ),* 
			$(,)? 
		)
	    { $( $bound: tt )* }
	) => {
		$( #[$all_meta] )*
		$( #[$var_meta] )*
		$var_vis struct $var_ident
		<$( $generic )*>
		( $( pub $field_ty, )* )
		where $( $bound )* ;
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Struct
	(
		{ $( #[$all_meta: meta] )* }
		$( #[$var_meta: meta] )*
		$var_vis: vis 
		$var_ident: ident
		{ $( $generic: tt )* }
		{
			$( $field_name: ident : $field_ty: ty ),*
			$(,)?
		}
		{ $( $bound: tt )* }
	) => {
		$( #[$all_meta] )*
		$( #[$var_meta] )*
		$var_vis struct $var_ident
		<$( $generic )*>
		where $( $bound )*
		{
			$( pub $field_name: $field_ty, )*
		}
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Unit
	(
		{ $( #[$all_meta: meta] )* }
		$( #[$var_meta: meta] )*
		$var_vis: vis 
		$var_ident: ident
		{ $( $generic: tt )* } // Ignored
		{ $( $bound: tt )* }   // Ignored
	) => {
		$( #[$all_meta] )*
		$( #[$var_meta] )*
		$var_vis struct $var_ident;
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Ignored Tuple
    (
		{ $( #[$all_meta: meta] )* }
		[@SKIP]
		$( #[$var_meta: meta] )*
		$var_vis: vis 
		$var_ident: ident
		{ $( $generic: tt )* }
		( 
			$( $field_ty: ty ),* 
			$(,)? 
		)
	    { $( $bound: tt )* }
	) => {
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Ignored Struct
	(
		{ $( #[$all_meta: meta] )* }
		[@SKIP]
		$( #[$var_meta: meta] )*
		$var_vis: vis 
		$var_ident: ident
		{ $( $generic: tt )* }
		{
			$( $field_name: ident : $field_ty: ty ),*
			$(,)?
		}
		{ $( $bound: tt )* }
	) => {
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// Ignored Unit
	(
		{ $( #[$all_meta: meta] )* }
		[@SKIP]
		$( #[$var_meta: meta] )*
		$var_vis: vis 
		$var_ident: ident
		{ $( $generic: tt )* }
		{ $( $bound: tt )* }
	) => {
	};
}