#[macro_export]
macro_rules! enum_delegate_impls {
    (
	    ENUM: {
		    $enum_ident: ident {
				$($var_ident: ident $( ( $($var_fields: tt)* ) )? ),*
			    $(,)?
	        }
	    }
	    
	    DELEGATES: {
		    $(
		        $trait_vis: vis trait $trait_ident: ident $( < [ $( $gens: tt )* ] > )? {
				    $( [ $( $item: tt )* ] )*
			    }
		    )*
	    }
    ) => {
	    $crate::enum_delegate_impls! {
		    @TRAITS_ENTRY
		    $enum_ident {
		        $( $var_ident ),*
		    }
		    
		    $(
			    $trait_ident $( < [ $( $gens )* ] > )? {
				    $( [ $( $item )* ] )*
			    }
		    )*
	    }
    };
	
	//------------------------------------------------------------------------------------------------------------------
	(@TRAITS_ENTRY
		$enum_ident: ident 
		$enum_vars: tt
		
		$(
			$trait_ident: ident $( < [ $( $gens: tt )* ] > )? {
		        $( $item: tt )*
	        }
		)*
	) => {
		$(
			impl $trait_ident for $enum_ident {
				$crate::enum_delegate_impls! {
					@TRAIT_ITEM
					$enum_ident
					$enum_vars
					$( $item )*
				}
			}
		)*
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// fn(self)
	(@TRAIT_ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    (self $(, $arg_ident: ident: $arg_ty: ty )*  $(,)? )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		    $(;)?
		]
    ) => {
	    $crate::enum_delegate_impls! {
		    @FN
		    $enum_ident {
			    $( $var_ident ),*
		    }
		    
		    [
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
	(@TRAIT_ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    (&self $(, $arg_ident: ident: $arg_ty: ty )*  $(,)? )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		    $(;)?
		]
    ) => {
	    $crate::enum_delegate_impls! {
		    @FN
		    $enum_ident {
			    $( $var_ident ),*
		    }
		    
		    [
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
	(@TRAIT_ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    (&mut self $(, $arg_ident: ident: $arg_ty: ty )*  $(,)? )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		    $(;)?
		]
    ) => {
	    $crate::enum_delegate_impls! {
		    @FN
		    $enum_ident {
			    $( $var_ident ),*
		    }
		    
		    [
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
	(@TRAIT_ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[
		    $( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    ( $( $arg_ident: ident: $arg_ty: ty ),*  $(,)? )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		    $(;)?
		]
    ) => {
	    $( $( $fn_type )* )?
	    fn $fn_ident
	    $( < $( $gens )* > )?
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
			$( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    ( self $(, $arg_ident: ident: $arg_ty: ty )* )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		]
		$args: tt
    ) => {
		$( [$( $fn_type )*] )?
	    fn $fn_ident
	    $( < [ $( $gens )* ] > )?
	    ( self $(, $arg_ident: $arg_ty )* )
	    $( -> $ret_ty )?
	    $( where [ $( $where_clause )* ] )?
		{
			match self {
			    $(
			        $enum_ident::$var_ident(var) => { var.$fn_ident $args }
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
			$( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    ( &self $(, $arg_ident: ident: $arg_ty: ty )* )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		]
		$args: tt
    ) => {
		$( [$( $fn_type )*] )?
	    fn $fn_ident
	    $( < [ $( $gens )* ] > )?
	    ( &self $(, $arg_ident: $arg_ty )* )
	    $( -> $ret_ty )?
	    $( where [ $( $where_clause )* ] )?
		{
			match self {
			    $(
			        $enum_ident::$var_ident(var) => { var.$fn_ident $args }
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
			$( [$( $fn_type: ident )*] )?
		    fn $fn_ident: ident
		    $( < [ $( $gens: tt )* ] > )?
		    ( &mut self $(, $arg_ident: ident: $arg_ty: ty )* )
		    $( -> $ret_ty: ty )?
		    $( where [ $( $where_clause: tt )* ] )?
		]
		$args: tt
    ) => {
		$( [$( $fn_type )*] )?
	    fn $fn_ident
	    $( < [ $( $gens )* ] > )?
	    ( &mut self $(, $arg_ident: $arg_ty )* )
	    $( -> $ret_ty )?
	    $( where [ $( $where_clause )* ] )?
		{
			match self {
			    $(
			        $enum_ident::$var_ident(var) => { var.$fn_ident $args }
			    )*
		    }
		}
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// const
	(@TRAIT_ITEM 
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[const $const_ident: ident: $const_ty: ty = $const_val: expr $(;)?]
	) => {
		const $const_ident: $const_ty = $const_val;
	};
	
	//------------------------------------------------------------------------------------------------------------------
	// type
	(@TRAIT_ITEM
		$enum_ident: ident {
		    $( $var_ident: ident ),*
	    }
	
		[type $type_ident: ident = $type_ty: ty $(;)?]
	) => {
		type $type_ident = $type_ty;
	};
}

#[allow(unused)]
#[cfg(test)]
mod test {
	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);
	
	enum StateEnum {
		Int(State<i32>),
		UInt(State<u32>),
		Float(State<f64>),
		Bool(State<bool>),
		Unit(State<()>),
	}
	
	trait Tick {
		fn tick(&mut self, delta_time: f64);
	}

	enum_delegate_impls! {
		ENUM: {
			StateEnum {
				Int(State<i32>),
				UInt(State<u32>),
				Float(State<f64>),
				Bool(State<bool>),
				Unit(State<()>),
			}
		}
		
		DELEGATES: { 
			trait Tick {
				[fn tick(&mut self, delta_time: f64);]
			}
		}
	}
	
	fn test(x: &mut StateEnum) {
		x.tick(2.0);
	}

	impl Tick for State<i32> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}

	impl Tick for State<u32> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}

	impl Tick for State<f64> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}

	impl Tick for State<bool> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}

	impl Tick for State<()> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}
}