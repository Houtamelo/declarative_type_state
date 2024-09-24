#[macro_export]
macro_rules! unit_enum_delegated {
    (
	    ENUM_OUT: {
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
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
	    $( #[$enum_meta] )*
	    $enum_vis enum $enum_ident {
		    $( $var_ident ),*
	    }
	    
	    $crate::unit_enum_variants_convert! {
		    $enum_vis enum $enum_ident {
			    $( $var_ident ),*
		    }
	    }
	    
	    $crate::unit_enum_delegate_impls! {
		    ENUM_IN: {
			    $enum_ident {
					$( $var_ident ),*
		        }
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
					    $([ $( $std_impl )* ])*
				    }
			    )?   
		    }
	    }
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

	trait Tick {
		fn tick(&mut self, delta_time: f64);

		fn test(&self);
	}


	unit_enum_delegated! {
		ENUM_OUT: {
			#[derive(Copy, Clone, Debug)]
			enum StateEnum {
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

#[allow(unused)]
#[cfg(test)]
mod test_generics_on_trait {
	use std::marker::PhantomData;

	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);

	trait Tick<G> {
		fn tick(&mut self, delta_time: G);
	}
	
	#[derive(Copy, Clone)]
	struct Int;
	#[derive(Copy, Clone)]
	struct UInt;

	unit_enum_delegated! {
		ENUM_OUT: {
			#[derive(Copy, Clone, Debug)]
			enum StateEnum {
				Int,
				UInt,
			}
		}
		
		DELEGATES: { 
			impl<['a, G]> trait Tick<G> {
				[fn tick(&mut self, delta_time: G)]
			}
		}
	}

	#[allow(clippy::needless_lifetimes)]
	fn test(x: &mut StateEnum) {
		x.tick(2.0);
	}

	impl<G> Tick<G> for Int {
		fn tick(&mut self, delta_time: G) {
			todo!()
		}
	}

	impl<G> Tick<G> for UInt {
		fn tick(&mut self, delta_time: G) {
			todo!()
		}
	}
}