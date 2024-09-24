#[macro_export]
macro_rules! unit_enum_delegate_impls {
    (
	    ENUM_IN: {
		    $enum_ident: ident {
				$( $var_ident: ident ),*
			    $(,)?
	        }
	    }
	    
	    DELEGATES: {
		    $(
		        impl $( <[ $( $trait_gen: tt )*  ]> )? 
		        trait $trait_ty: path
		        $( where [ $( $trait_bound: tt )* ] )?
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
	    $crate::unit_enum_delegate_impls! {
		    @TOKENIZE
		    $enum_ident {
		        $( $var_ident ),*
		    }
		    
		    DELEGATES: {
			    $(
			        impl $( <[ $( $trait_gen )*  ]> )? 
				    trait $trait_ty
			        $( where [ $( $trait_bound )* ] )?
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
	
	//------------------------------------------------------------------------------------------------------------------
	(@TOKENIZE
		$enum_ident: ident 
		$enum_vars: tt
	
		DELEGATES: {
			$(
				impl $( <[ $( $trait_gen: tt )*  ]> )? 
		        trait $trait_ty: path
				$( where [ $( $trait_bound: tt )* ] )?
				{
			        $( $item: tt )*
		        }
			)*
		
			$(
				impl { 
					$( $std_impl: tt )*
				}
			)?
		}
	) => {
		$(
			$crate::unit_enum_delegate_impls! {
				@TRAIT_IMPL
				$enum_ident
				$enum_vars
				
				impl $( <[ $( $trait_gen )*  ]> )?
				trait $trait_ty
				$( where [ $( $trait_bound )* ] )?
				{
			        $( $item )*
		        }
			}
		)*
		
		impl $enum_ident {
			$(
				$(
					$crate::unit_enum_delegate_impls! {
						@ITEM
						$enum_ident
						$enum_vars
						$std_impl
					}
				)*
			)?
		}
	};
	
	//------------------------------------------------------------------------------------------------------------------
	(@TRAIT_IMPL
		$enum_ident: ident
		$enum_vars: tt
	
		impl $( <[ $( $trait_gen: tt )* ]> )? 
		trait $trait_ty: path
		$( where [ $( $trait_bound: tt )* ] )?
		{
	        $( $item: tt )*
        }
	) => {
		impl$(< $( $trait_gen )* >)?
		$trait_ty
		for $enum_ident
		$( where $( $trait_bound )* )?
		{
			$(
				$crate::unit_enum_delegate_impls! {
					@ITEM
					$enum_ident
					$enum_vars
					$item
				}
			)*
		}
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// fn(self)
	(@ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $fn_vis: vis
		    $( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    (self $(, $arg_ident: ident: $arg_ty: ty )*  $(,)? )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		    $(;)?
		]
    ) => {
	    $crate::unit_enum_delegate_impls! {
		    @FN
		    $enum_ident {
			    $( $var_ident ),*
		    }
		    
		    [
			    $fn_vis 
			    $( [$( $fn_type )*] )?
			    fn $fn_ident
			    $( < [ $( $gens )* ] > )?
			    (self $(, $arg_ident: $arg_ty )* )
			    $( -> $ret_ty )?
			    $( where [ $( $where_clause )* ] )?
			]
		    ($($arg_ident),*)
	    }
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// fn(&self)
	(@ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $fn_vis: vis
		    $( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    (&self $(, $arg_ident: ident: $arg_ty: ty )*  $(,)? )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		    $(;)?
		]
    ) => {
	    $crate::unit_enum_delegate_impls! {
		    @FN
		    $enum_ident {
			    $( $var_ident ),*
		    }
		    
		    [
			    $fn_vis
			    $( [$( $fn_type )*] )?
			    fn $fn_ident
			    $( < [ $( $gens )* ] > )?
			    (&self $(, $arg_ident: $arg_ty )* )
			    $( -> $ret_ty )?
			    $( where [ $( $where_clause )* ] )?
			]
		    ($($arg_ident),*)
	    }
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// fn(&mut self)
	(@ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $fn_vis: vis
		    $( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    (&mut self $(, $arg_ident: ident: $arg_ty: ty )*  $(,)? )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		    $(;)?
		]
    ) => {
	    $crate::unit_enum_delegate_impls! {
		    @FN
		    $enum_ident {
			    $( $var_ident ),*
		    }
		    
		    [
			    $fn_vis
			    $( [$( $fn_type )*] )?
			    fn $fn_ident
			    $( < [ $( $gens )* ] > )?
			    (&mut self $(, $arg_ident: $arg_ty )* )
			    $( -> $ret_ty )?
			    $( where [ $( $where_clause )* ] )?
			]
		    ($($arg_ident),*)
	    }
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// fn() <- INVALID
	(@ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $fn_vis: vis
		    $( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    ( $( $arg_ident: ident: $arg_ty: ty ),*  $(,)? )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		    $(;)?
		]
    ) => {
	    $fn_vis 
	    $( $( $fn_type )* )?
	    fn $fn_ident
	    $( <$( $gens )*> )?
	    ( $( $arg_ident: $arg_ty ),* )
	    $( -> $ret_ty )?
	    $( where $( $where_clause )* )?
	    {
		    let types = stringify!( $( $( $fn_type )* )? );
		    let name = stringify!( $fn_ident );
		    let gens = stringify!( $( < $( $gens )* > )? );
		    let args = stringify!( ( $( $arg_ident: $arg_ty ),* ) );
		    let args_correct = stringify!( (+self+ $(, $arg_ident: $arg_ty )* ) );
		    let ret = stringify!( $( -> $ret_ty )? );
		    let clause = stringify!( $( where $( $where_clause )* )? );
		    
		    compile_error!(
			    format!("Cannot delegate a impl function to an enum without a self parameter.\n\
			     Help: add a (self | &self | &mut | self: T<Self>) parameter.\n\
			     {types} {name} {gens} {args} {ret} {clause}\n\
			     => \n\
			     {types} {name} {gens} {args_correct} {ret} {clause}"
			    ).as_str()
		    );
	    }
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// fn(self)
	(@FN
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $fn_vis: vis
		    $( [$( $fn_type: ident )*] )?
			fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    ( self $(, $arg_ident: ident: $arg_ty: ty )* )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		]
		$args: tt
    ) => {
	    $fn_vis 
		$( $( $fn_type )* )?
		fn $fn_ident
	    $( <$( $gens )*> )?
	    ( self $(, $arg_ident: $arg_ty )* )
	    $( -> $ret_ty )?
	    $( where $( $where_clause )* )?
		{
			match self {
			    $(
			        #[allow(unused_mut)]
			        Self::$var_ident => {
				        let mut var = $var_ident;
				        var.$fn_ident $args
			        }
			    )*
		    }
		}
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// fn(&self)
	(@FN
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $fn_vis: vis
			$( [$( $fn_type: ident )*] )?
			fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    ( &self $(, $arg_ident: ident: $arg_ty: ty )* )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		]
		$args: tt
    ) => {
	    $fn_vis 
		$( $( $fn_type )* )?
		fn $fn_ident
	    $( <$( $gens )*> )?
	    ( &self $(, $arg_ident: $arg_ty )* )
	    $( -> $ret_ty )?
	    $( where $( $where_clause )* )?
		{
			match self {
			    $(
			        #[allow(unused_mut)]
			        Self::$var_ident => {
				        let mut var = $var_ident;
				        var.$fn_ident $args
			        }
			    )*
		    }
		}
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// fn(&mut self)
	(@FN
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $fn_vis: vis
			$( [$( $fn_type: ident )*] )?
			fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    ( &mut self $(, $arg_ident: ident: $arg_ty: ty )* )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		]
		$args: tt
    ) => {
	    $fn_vis
	    $( $( $fn_type )* )?
	    fn $fn_ident
	    $( <$( $gens )*> )?
	    ( &mut self $(, $arg_ident: $arg_ty )* )
	    $( -> $ret_ty )?
	    $( where $( $where_clause )* )?
		{
			match self {
			    $(
			        #[allow(unused_mut)]
			        Self::$var_ident => {
				        let mut var = $var_ident;
				        var.$fn_ident $args
			        }
			    )*
		    }
		}
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// const
	(@ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[const $($token: tt)*]
	) => {
		const $($token)*;
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// type
	(@ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[type $($token: tt)*]
	) => {
		type $($token)*;
	};
}

#[allow(unused)]
#[cfg(test)]
mod test {
	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);
	
	#[derive(Copy, Clone)]
	struct Int;
	#[derive(Copy, Clone)]
	struct UInt;

	#[derive(Copy, Clone)]
	enum StateEnum {
		Int,
		UInt,
	}

	trait Tick {
		fn tick(&mut self, delta_time: f64);
		fn test(&self);
	}
	
	unit_enum_delegate_impls! {
		ENUM_IN: {
			StateEnum {
				Int,
				UInt,
			}
		}
		
		DELEGATES: { 
			impl trait Tick {
				[fn tick(&mut self, delta_time: f64)]
				
				[fn test(&self);]
			}
		}
	}

	fn test(x: &mut StateEnum) {
		x.tick(2.0);
	}

	impl Tick for Int {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}

		fn test(&self) {
			todo!()
		}
	}

	impl Tick for UInt {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}

		fn test(&self) {
			todo!()
		}
	}
}