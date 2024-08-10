#[macro_export]
macro_rules! type_state_enum {
	//------------------------------------------------------------------------------------------------------------------
	// User provided state struct
    (
	    STATE: $state_ident: ident { $state_field_ident: ident }

	    ENUM_OUT: {
		    #[vars( $( $all_meta: meta ),* $(,)? )]
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
				$(
					$( [@ $ignore: ident] )?
					$( #[$var_meta: meta] )*
					$var_ident: ident $( ( $($var_tuple: tt)* ) )? $( { $($var_fields: tt)* } )?
			    ),*
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
			$( $var_ident($state_ident<$var_ident>) ),*
		}

		impl std::ops::Deref for $enum_ident {
			type Target = $state_ident<dyn std::any::Any>;

			fn deref(&self) -> &Self::Target {
				match self {
					$( $enum_ident::$var_ident(var) => var ),*
				}
			}
		}

		impl std::ops::DerefMut for $enum_ident {
			fn deref_mut(&mut self) -> &mut Self::Target {
				match self {
					$( $enum_ident::$var_ident(var) => var ),*
				}
			}
		}
	    
	    impl<Curr> $state_ident<Curr> {
		    #[allow(clippy::needless_update)]
		    pub fn transition_to<Next, Enum>(self, next: Next) 
		        -> $crate::Transition<Self, Enum> where $state_ident<Next>: Into<Enum>
		    {
			    $crate::ChangedTo(self.with_state(next).into())
		    }
		    
		    #[allow(clippy::needless_update)]
		    pub fn with_state<Next>(self, next: Next) -> $state_ident<Next> {
			    $state_ident::<Next> {
				    $state_field_ident: next,
				    ..self
			    }
		    }
	    }
		
		$crate::extract_variants! {
		    #[vars( $( $all_meta ),* )]
		    $( #[$enum_meta] )*
			$enum_vis enum $enum_ident {
			    $(
			        $( [@ $ignore] )?
			        $( #[$var_meta] )*
			        $var_ident $( ( $($var_tuple)* ) )? $( { $($var_fields)* } )?,
			    )*
		    }
	    }
	    
	    $crate::enum_variants_convert! {
		    enum $enum_ident {
			    $( $var_ident ( $state_ident<$var_ident> ) ),*
		    }
	    }
		
		    
	    $crate::enum_delegate_impls! {
		    ENUM_IN: {
			    $enum_ident {
			        $( $var_ident ( $state_ident<$var_ident> ) ),*
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
					    $( [ $( $std_impl )* ] )*
				    }
			    )?
		    }
	    }
    };
	
	//------------------------------------------------------------------------------------------------------------------
	// Generated state struct
	(
		STATE: {
			$( #[ $state_meta: meta ] )*
			$state_vis: vis struct $state_ident: ident {
				$( $state_field_ident: ident : $state_field_ty: ty ),*
				$(,)?
			}
		}
		
		ENUM_OUT: {
		    #[vars( $( $all_meta: meta ),* $(,)? )]
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
				$(
					$( [@ $ignore: ident] )?
					$( #[$var_meta: meta] )*
					$var_ident: ident $( ( $($var_tuple: tt)* ) )? $( { $($var_fields: tt)* } )?
			    ),*
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
		$( #[ $state_meta ] )*
		$state_vis struct $state_ident<T: ?Sized> {
			$( $state_field_ident : $state_field_ty ),*
			state: T,
		}
		
		$crate::type_state_enum! {
		    STATE: $state_ident { state }
			
			ENUM_OUT: {
			    #[vars( $( $all_meta ),* $(,)? )]
			    $( #[$enum_meta] )*
			    $enum_vis enum $enum_ident {
					$(
						$( [@ $ignore] )?
						$( #[$var_meta] )*
						$var_ident: $( ( $($var_tuple)* ) )? $( { $($var_fields)* } )?
				    ),*
				    $(,)?
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
					    $( [ $( $std_impl )* ] )*
				    }
			    )?
		    }
	    }
	};
}

#[cfg(test)]
#[allow(unused)]
#[allow(non_camel_case_types)]
mod test {
	use crate::transition_result::Transition;

	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized> {
		state: T,
	}

	#[derive(Debug, Clone)]
	pub struct CustomType;

	type_state_enum! {
		STATE: State { state }
		
		ENUM_OUT: {
			#[vars(derive(Clone, Debug))]
			#[derive(Debug, Clone)]
			pub enum StateEnum {
				#[derive(PartialEq)] 
				 Int { field: i32 },
				 UInt {
					 x: i32,
					 y: i32,
				 },
				 Float(f32, i32),
				 [@SKIP]
				 CustomType,
				 Unit,
			}
		}
		
		DELEGATES: { 
			impl trait Tick {
				[fn tick(&mut self, delta_time: f64);]
			}
		}
	}
	
	fn test(mut x: State<Int>) {
		let t: Transition<State<Int>, StateEnum> = x.transition_to(Unit);
	}
	
	trait Tick {
		fn tick(&mut self, delta_time: f64);
	}
	
	impl Tick for State<Int> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}
	
	impl Tick for State<UInt> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}
	
	impl Tick for State<Float> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}
	
	impl Tick for State<CustomType> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}
	
	impl Tick for State<Unit> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
	}
}