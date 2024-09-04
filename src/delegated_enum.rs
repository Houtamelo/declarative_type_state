/// Generates an enum with the specified variants, and implements the specified traits and methods for the enum, 
/// where each variant's type also implements the traits/methods.
/// 
/// ## Also implements for each variant's type:
/// - From<Variant> for Enum
/// - TryFrom<Enum> for Variant
///
/// # Input
/// - `ENUM_OUT`: Defines the output enum with its metadata, visibility, 
///   name, generics, and variants.
/// - `DELEGATES`: Specifies the traits and methods to be implemented for the enum. 
///   All variants of the enum must implement the traits/methods.
///
/// ## ENUM_OUT:
/// 
/// ```pseudo
/// [enum_meta]
/// [visibility] enum [name]<[generics]> [where [bounds]] {
///     [var_name_A]([var_type_A]),
///     [var_name_B]([var_type_B]),
/// }
/// 
/// ```
/// 
/// - `[enum_meta]`: Metadata attributes to be applied to the enum. (e.g., `#[derive(Debug, Clone)]`)
/// - [visibility]: Visibility level of the enum. (e.g., `pub`)
/// - [name]: Identifier (name) of the enum. (e.g., `MyEnum`)
/// - [generics]: Optional generics for the enum, must be placed inside brackets. (e.g., `<[T]>`)
/// - where [bounds]: Optional where clause for the enum, must be placed inside brackets. (e.g., `where [T: SomeTrait]`)
/// - [var_name]([var_type]): Variants of the enum along with their types. (e.g., `VariantOne(TypeOne), VariantTwo(TypeTwo)`)
/// 
/// ## DELEGATES - Traits:
/// Specifies the traits to be implemented for the enum.
/// All variants of the enum must implement the traits.
/// 
/// All items inside the trait must be placed inside brackets.
/// 
/// ```pseudo
/// impl<[generics]> trait [trait_type] [where [bounds]] {
///     [ type [associated_type_name] = [associated_type] ]
///     [ const [associated_const_name]: [const_type] = [expr] ]
/// 
///     [ fn [method_name]<[generics]>([self_type], [args]) -> [return_type] ]
/// }
/// ```
/// 
/// - `[generics]`: Optional generics for the trait implementation. Must be placed inside brackets.
/// - `[trait_type]`: The path of the trait to be implemented.
/// - `where [bounds]`: Optional `where` clause for the trait implementation. Bounds must be placed inside brackets.
/// - Methods:
///     - `[method_name]`: Method name.
///     - `<[generics]>`: Optional generics for the method. Must be placed inside brackets.
///     - `([self_type], [args])`: Method parameters including self-reference.
///     - `-> [return_type]`: Return type of the method (optional if return type is `()`). 
/// - Associated types:
///     - `[associated_type_name]`: Name of an associated type.
///     - `= [associated_type]`: Type associated with the name.
/// - Associated constants:
///     - `[associated_const_name]`: Name of an associated constant.
///     - `: [const_type]`: Type of the associated constant.
///     - `= [expr]`: Value of the associated constant.
///
/// ## DELEGATES - Methods:
/// Additional method implementations.
/// All variants of the enum must have methods with the same name and number of arguments.
/// 
/// ```pseudo
/// impl {
///     [fn [method_name]<[generics]>([self_type], [args]) -> [return_type]]
/// }
/// ```
///
/// - `[method_name]`: Method name.
/// - `<[generics]>`: Optional generics for the method. Must be placed inside brackets.
/// - `([self_type], [args])`: Method parameters including self-reference.
/// - `-> [return_type]`: Return type of the method (optional if return type is `()`).
/// # Example
///
/// ```rust
/// use declarative_type_state::delegated_enum;
/// use std::fmt::{Debug, Formatter};
///
/// delegated_enum! {
///     ENUM_OUT: {
///         enum IDebug {
///             Int(i32),
///             Bool(bool),
///             String(String),
///         }
///     }
///
///     DELEGATES: {
///         impl trait Debug {
///             [fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error>]
///         }
/// 
///         impl {
///             [fn to_string(&self) -> String]
///         }
///     }
/// }
///
/// let debug = IDebug::Int(5);
/// assert_eq!(format!("{debug:?}"), format!("{:?}", 5_i32));
/// assert_eq!(debug.to_string(), 5.to_string());
/// ```
#[macro_export]
macro_rules! delegated_enum {
    (
	    ENUM_OUT: {
		    $( #[$enum_meta: meta] )*
		    $enum_vis: vis enum $enum_ident: ident
		    $( <[ $( $enum_gen: tt )* ]> )?
			$( where [ $( $enum_bound: tt )* ] )?
		    {
			    $( $var_ident: ident ($var_ty: ty) ),*
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
	    $enum_vis enum $enum_ident
	    $( <$( $enum_gen )*> )?
	    $( where $( $enum_bound )* )?
	    {
		    $( $var_ident ( $var_ty ) ),*
	    }
	    
	    $crate::enum_variants_convert! {
		    $enum_vis enum $enum_ident
		    $( <[ $( $enum_gen )* ]> )?
			$( where [ $( $enum_bound )* ] )?
		    {
			    $( $var_ident ( $var_ty ) ),*
		    }
	    }
	    
	    $crate::enum_delegate_impls! {
		    ENUM_IN: {
			    $enum_ident
			    $( <[ $( $enum_gen )* ]> )?
				$( where [ $( $enum_bound )* ] )? 
			    {
					$( $var_ident ($var_ty) ),*
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