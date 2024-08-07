#[macro_export]
macro_rules! delegated_enum {
    (
	    ENUM_OUT: {
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident
		    $( <[ $( $enum_gen: tt )* ]> )?
			$( [where $( $enum_bound: tt )* ] )?
		    {
			    $( $var_ident: ident ($var_ty: ty) ),*
			    $(,)?
		    }
	    }
	    
	    DELEGATES: {
		    $(
		        impl $( <[ $( $trait_gen: tt )*  ]> )? 
		        trait $trait_ty: path
		        $( [where $( $trait_bound: tt )* ] )?
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
	    $enum_vis enum $enum_ident
	    $( <$( $enum_gen )*> )?
	    $( where $( $enum_bound )* )?
	    {
		    $( $var_ident ( $var_ty ) ),*
	    }
	    
	    $crate::enum_variants_convert! {
		    $enum_vis enum $enum_ident
		    $( <[ $( $enum_gen )* ]> )?
			$( [where $( $enum_bound )* ] )?
		    {
			    $( $var_ident ( $var_ty ) ),*
		    }
	    }
	    
	    $crate::enum_delegate_impls! {
		    ENUM_IN: {
			    $enum_ident
			    $( <[ $( $enum_gen )* ]> )?
				$( [where $( $enum_bound )* ] )? 
			    {
					$( $var_ident ($var_ty) ),*
		        }
		    }
		    
		    DELEGATES: {
			    $(
			        impl $( <[ $( $trait_gen )*  ]> )? 
				    trait $trait_ty
			        $( [where $( $trait_bound )* ] )?
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

	trait Tick {
		fn tick(&mut self, delta_time: f64);

		fn test(&self);
	}


	delegated_enum! {
		ENUM_OUT: {
			#[derive(Clone, Debug)]
			enum StateEnum {
				Int(State<i32>),
				UInt(State<u32>),
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

#[allow(unused)]
#[cfg(test)]
mod test_generics {
	use std::marker::PhantomData;

	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);

	trait Tick {
		fn tick(&mut self, delta_time: f64);

		fn test(&self);
	}

	delegated_enum! {
		ENUM_OUT: {
			#[derive(Clone, Debug)]
			enum StateEnum<['a, S]> {
				Int(State<i32>),
				Empty(PhantomData<&'a S>),
			}
		}
		
		DELEGATES: { 
			impl<['a, S]> trait Tick {
				[fn tick(&mut self, delta_time: f64)]
				
				[fn test(&self);]
			}
		}
	}

	#[allow(clippy::needless_lifetimes)]
	fn test<'a, S>(x: &mut StateEnum<'a, S>) {
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

	impl <'a, S> Tick for PhantomData<&'a S> {
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

	delegated_enum! {
		ENUM_OUT: {
			#[derive(Clone, Debug)]
			enum StateEnum<['a, S]> {
				Int(State<i32>),
				Empty(PhantomData<&'a S>),
			}
		}
		
		DELEGATES: { 
			impl<['a, S, G]> trait Tick<G> {
				[fn tick(&mut self, delta_time: G)]
			}
		}
	}

	#[allow(clippy::needless_lifetimes)]
	fn test<'a, S>(x: &mut StateEnum<'a, S>) {
		x.tick(2.0);
	}

	impl<G> Tick<G> for State<i32> {
		fn tick(&mut self, delta_time: G) {
			todo!()
		}
	}

	impl<'a, S, G> Tick<G> for PhantomData<&'a S> {
		fn tick(&mut self, delta_time: G) {
			todo!()
		}
	}
}