#[macro_export]
macro_rules! delegated_enum {
    (
	    ENUM: {
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
			    $( $var_ident: ident ($var_ty: ty) ),*
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
	    $( #[$enum_meta] )*
	    $enum_vis enum $enum_ident {
		    $( $var_ident ($var_ty) ),*
	    }
	    
	    $crate::enum_delegate_impls! {
		    ENUM: {
			    $enum_ident {
					$( $var_ident ($var_ty) ),*
		        }
		    }
		    
		    DELEGATES: {
			    $(
			        $trait_vis trait $trait_ident $( < [ $( $gens )* ] > )? {
					    $( [ $( $item )* ] )*
				    }
			    )*
		    }
	    }
    };
}

#[allow(unused)]
#[cfg(test)]
mod test {
	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);

	trait Tick {
		fn tick(&mut self, delta_time: f64);

		fn test(&self);
	}


	delegated_enum! {
		ENUM: {
			#[derive(Clone, Debug)]
			enum StateEnum {
				Int(State<i32>),
				UInt(State<u32>),
			}
		}
		
		DELEGATES: { 
			trait Tick {
				[fn tick(&mut self, delta_time: f64)]
				
				[fn test(&self);]
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

		fn test(&self) {
			todo!()
		}
	}

	impl Tick for State<u32> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}

		fn test(&self) {
			todo!()
		}
	}
}