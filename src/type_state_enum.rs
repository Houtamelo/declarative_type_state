#[macro_export]
macro_rules! type_state_enum {
	//------------------------------------------------------------------------------------------------------------------
	// User provided state struct
    (
	    STATE: $state_ident: ident { $state_field_ident: ident }

	    ENUM: {
		    #[vars( $( $all_meta: meta ),* $(,)? )]
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
				$(
					$( #[$var_meta: meta] )*
					$var_ident: ident $( ( $($var_tuple: tt)* ) )? $( { $($var_fields: tt)* } )?
			    ),*
			    $(,)?
		    }
	    }
	    
	    DELEGATES: {
		    $(
		        $trait_vis: vis trait $trait_ident: ident $( < [ $( $gens: tt )* ] > )? {
				    $( [ $( $trait_item: tt )* ] )*
			    }
		    )*
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
		    pub fn transition_to<Next>(self, next: Next) -> $state_ident<Next> {
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
		    ENUM: {
			    $enum_ident {
			        $( $var_ident ),*
			    }
		    }
		    
		    DELEGATES: { 
			    $(
			        $trait_vis trait $trait_ident $( < [ $( $gens )* ] > )? {
					    $( [ $( $trait_item )* ] )*
				    }
		        )*
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
		
		ENUM: {
		    #[vars( $( $all_meta: meta ),* $(,)? )]
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident {
				$(
					$( #[$var_meta: meta] )*
					$var_ident: ident $( ( $($var_tuple: tt)* ) )? $( { $($var_fields: tt)* } )?
			    ),*
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
		$( #[ $state_meta ] )*
		$state_vis struct $state_ident<T: ?Sized> {
			$( $state_field_ident : $state_field_ty ),*
			state: T,
		}
		
		$crate::type_state_enum! {
		    STATE: $state_ident { state }
			
			ENUM: {
			    #[vars( $( $all_meta ),* $(,)? )]
			    $( #[$enum_meta] )*
			    $enum_vis enum $enum_ident {
					$(
						$( #[$var_meta] )*
						$var_ident: $( ( $($var_tuple)* ) )? $( { $($var_fields)* } )?
				    ),*
				    $(,)?
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

#[cfg(test)]
#[allow(unused)]
mod test {
	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized> {
		state: T,
	}

	type_state_enum! {
		STATE: State { state }
		
		ENUM: {
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
				 Bool(bool),
				 Unit(),
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
	
	impl Tick for State<Bool> {
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