/// Implements the specified traits and methods for the enum, 
/// where each variant's type also implements the traits/methods.
///
/// # Input
/// - `ENUM_IN`: Defines the input enum with its name, generics, where clauses, and variants.
/// - `DELEGATES`: Specifies the traits and methods to be implemented for the enum.
///   All variants of the enum must implement the traits/methods.
///
/// ## ENUM_IN:
///
/// ```pseudo
/// [enum_ident] <[generics]> [where_clause] {
///     [var_name_A]([var_type_A]),
///     [var_name_B]([var_type_B]),
/// }
/// ```
///
/// - `[enum_ident]`: Identifier (name) of the enum. (e.g., `MyEnum`)
/// - `[generics]`: Optional generics for the enum, must be placed inside brackets. (e.g., `<[T]>`)
/// - `[where_clause]`: Optional where clause for the enum, must be placed inside brackets. (e.g., `where [T: SomeTrait]`)
/// - `[var_name]([var_type])`: Variants of the enum along with their types. (e.g., `VariantOne(TypeOne), VariantTwo(TypeTwo)`)
///
/// ## DELEGATES - Traits:
/// Specifies the traits to be implemented for the enum.
/// All variants of the enum must implement the traits.
///
/// All items inside the trait must be placed inside brackets.
///
/// ```pseudo
/// impl<[generics]> trait [trait_type] [where_clause] {
///     [ type [associated_type_name] = [associated_type] ]
///     [ const [associated_const_name]: [const_type] = [expr] ]
///
///     [ fn [method_name]<[generics]>([self_type], [args]) -> [return_type] ]
/// }
/// ```
///
/// - `[generics]`: Optional generics for the trait implementation, must be placed inside brackets.
/// - `[trait_type]`: The path of the trait to be implemented.
/// - `[where_clause]`: Optional `where` clause for the trait implementation, bounds must be placed inside brackets.
/// - Methods:
///     - `[method_name]`: Method name.
///     - `<[generics]>`: Optional generics for the method, must be placed inside brackets.
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
/// - `<[generics]>`: Optional generics for the method, must be placed inside brackets.
/// - `([self_type], [args])`: Method parameters including self-reference.
/// - `-> [return_type]`: Return type of the method (optional if return type is `()`).
/// 
/// # Example
/// 
/// ```rust
/// use declarative_type_state::enum_delegate_impls;
/// use std::fmt::{Debug, Formatter};
///
/// enum IDebug {
///     Int(i32),
///     Bool(bool),
///     String(String),
/// }
///
///
/// enum_delegate_impls! {
///     ENUM_IN: {
///         IDebug {
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
macro_rules! enum_delegate_impls {
    (
	    ENUM_IN: {
		    $enum_ident: ident
		    $( <[ $( $enum_gen: tt )* ]> )?
			$( where [ $( $enum_bound: tt )* ] )?
		    {
				$( $var_ident: ident ( $var_ty: ty ) ),* // var_ty is ignored
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
	    $crate::enum_delegate_impls! {
		    @TOKENIZE
		    { $( $( $enum_gen )* )? }
			{ $( $( $enum_bound )* )? }
		    { $( $( $enum_gen )* )? }
			{ $( $( $enum_bound )* )? }
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
		$generics_tt: tt
		$bounds_tt: tt
		{ $( $generic: tt )* }
		{ $( $bound: tt )* }
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
			$crate::enum_delegate_impls! {
				@TRAIT_IMPL
				$generics_tt
				$bounds_tt
				$generics_tt
				$bounds_tt
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
		
		impl<$( $generic )*> 
		$enum_ident<$( $generic )*> 
		where $( $bound )*
		{
			$(
				$(
					$crate::enum_delegate_impls! {
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
		$generics_tt: tt
		$bounds_tt: tt
		{ $( $enum_gen: tt )* }
		{ $( $enum_bound: tt )* }
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
		for $enum_ident<$( $enum_gen )*>
		$( where $( $trait_bound )* )?
		{
			$(
				$crate::enum_delegate_impls! {
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
	    $crate::enum_delegate_impls! {
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
	    $crate::enum_delegate_impls! {
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
	    $crate::enum_delegate_impls! {
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
			        Self::$var_ident(var) => { var.$fn_ident $args }
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
			        Self::$var_ident(var) => { var.$fn_ident $args }
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
			        Self::$var_ident(var) => { var.$fn_ident $args }
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

	enum StateEnum {
		Int(State<i32>),
		UInt(State<u32>),
	}

	trait Tick {
		fn tick(&mut self, delta_time: f64);

		fn test(&self);
	}

	
	enum_delegate_impls! {
		ENUM_IN: {
			StateEnum {
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
mod test_2 {
	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);

	enum StateEnum {
		Int(State<i32>),
		UInt(State<u32>),
	}

	trait Tick {
		fn tick(&mut self, delta_time: f64);

		fn test(&self);
	}


	enum_delegate_impls! {
		ENUM_IN: {
			StateEnum {
				Int(State<i32>),
				UInt(State<u32>),
			}
		}
		
		DELEGATES: {
			impl {
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
mod test_3 {
	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);

	enum StateEnum {
		Int(State<i32>),
		UInt(State<u32>),
	}

	enum_delegate_impls! {
		ENUM_IN: {
			StateEnum {
				Int(State<i32>),
				UInt(State<u32>),
			}
		}
		
		DELEGATES: { 
			impl { 
				[fn tick(&mut self, delta_time: f64)]
				[pub fn test(&self)]
			}
		}
	}

	fn test(x: &mut StateEnum) {
		x.tick(2.0);
	}

	impl State<i32> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}

		fn test(&self) {
			todo!()
		}
	}

	impl State<u32> {
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
mod test_4 {
	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);

	enum StateEnum {
		Int(State<i32>),
		UInt(State<u32>),
	}

	trait Tick {
		fn tick(&mut self, delta_time: f64);

		fn test(&self);
	}
	
	enum_delegate_impls! {
		ENUM_IN: {
			StateEnum {
				Int(State<i32>),
				UInt(State<u32>),
			}
		}
		
		DELEGATES: { 
			impl trait Tick {
				[fn tick(&mut self, delta_time: f64)]
				
				[fn test(&self);]
			}
			
			impl {
				[pub fn other(&self) -> i64]
				[pub(crate) fn and_so(&mut self)]
			}
		}
	}

	fn test(x: &mut StateEnum) {
		x.tick(2.0);
		x.other();
		x.and_so();
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
	
	impl State<i32> {
		fn other(&self) -> i64 {
			todo!()
		}
		
		fn and_so(&mut self) {
			todo!()
		}
	}
	
	impl State<u32> {
		fn other(&self) -> i64 {
			todo!()
		}
		
		fn and_so(&mut self) {
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

	pub struct Dummy<'a, S>(PhantomData<&'a S>);
	
	enum StateEnum<'a, S> {
		Int(State<i32>),
		UInt(State<u32>),
		Empty(Dummy<'a, S>)
	}

	trait Tick {
		fn tick(&mut self, delta_time: f64);

		fn test(&self);
	}

	enum_delegate_impls! {
		ENUM_IN: {
			StateEnum<['a, S]> {
				Int(State<i32>),
				UInt(State<u32>),
				Empty(Dummy<'a, S>),
			}
		}
		
		DELEGATES: { 
			impl<['a, S]> trait Tick {
				[fn tick(&mut self, delta_time: f64)]
				
				[fn test(&self);]
			}
			
			impl {
				[pub fn other(&self) -> i64]
				[pub(crate) fn and_so(&mut self)]
			}
		}
	}

	fn test<S>(x: &mut StateEnum<S>) {
		x.tick(2.0);
		x.other();
		x.and_so();
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
	
	impl<'a, S> Tick for Dummy<'a, S> {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}
		
		fn test(&self) {
			todo!()
		}
	}

	impl State<i32> {
		fn other(&self) -> i64 {
			todo!()
		}

		fn and_so(&mut self) {
			todo!()
		}
	}

	impl State<u32> {
		fn other(&self) -> i64 {
			todo!()
		}

		fn and_so(&mut self) {
			todo!()
		}
	}
	
	impl<'a, S> Dummy<'a, S> {
		fn other(&self) -> i64 {
			todo!()
		}
		
		fn and_so(&mut self) {
			todo!()
		}
	}
}

#[allow(unused)]
#[cfg(test)]
mod test_generics_many {
	use std::marker::PhantomData;

	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);

	pub struct Dummy<'a, 'b, S, T>(PhantomData<(&'a S, &'b T)>) where 'b: 'a, T: Sized;

	enum StateEnum<'a, 'b, S, T> where 'b: 'a, T:Sized {
		Int(State<i32>),
		UInt(State<u32>),
		Empty(Dummy<'a, 'b, S, T>)
	}

	trait Tick {
		fn tick(&mut self, delta_time: f64);

		fn test(&self);
	}

	enum_delegate_impls! {
		ENUM_IN: {
			StateEnum<['a, 'b, S, T]> where [ 'b: 'a, T: Sized] {
				Int(State<i32>),
				UInt(State<u32>),
				Empty(Dummy<'a, 'b, S, T>),
			}
		}
		
		DELEGATES: { 
			impl<['a, 'b, S, T]> trait Tick where [ 'b: 'a, T: Sized] {
				[fn tick(&mut self, delta_time: f64)]
				
				[fn test(&self);]
			}
			
			impl {
				[pub [async unsafe] fn other(&self) -> i64]
				[pub(crate) fn and_so<[D]>(&mut self, x: D)]
			}
		}
	}

	fn test<'a, 'b, S, T>(x: &mut StateEnum<'a, 'b, S, T>) where 'b: 'a, T: Sized {
		x.tick(2.0);
		unsafe { x.other() };
		x.and_so::<i32>(5);
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

	impl<'a, 'b, S, T> Tick for Dummy<'a, 'b, S, T> where 'b: 'a, T: Sized {
		fn tick(&mut self, delta_time: f64) {
			todo!()
		}

		fn test(&self) {
			todo!()
		}
	}

	impl State<i32> {
		fn other(&self) -> i64 {
			todo!()
		}

		fn and_so<T>(&mut self, x: T) {
			todo!()
		}
	}

	impl State<u32> {
		fn other(&self) -> i64 {
			todo!()
		}

		fn and_so<T>(&mut self, x: T) {
			todo!()
		}
	}

	impl<'a, 'b, S, T> Dummy<'a, 'b, S, T> where 'b: 'a, T: Sized {
		fn other(&self) -> i64 {
			todo!()
		}

		fn and_so<D>(&mut self, x: D) {
			todo!()
		}
	}
}

#[allow(unused)]
#[cfg(test)]
mod test_generics_many_with_trait_generics {
	use std::marker::PhantomData;

	#[derive(Clone, Debug)]
	pub struct State<T: ?Sized>(T);

	pub struct Dummy<'a, 'b, S, T>(PhantomData<(&'a S, &'b T)>)
		where
			'b: 'a,
			T: Sized;

	enum StateEnum<'a, 'b, S, T>
		where
			'b: 'a,
			T: Sized
	{
		Int(State<i32>),
		UInt(State<u32>),
		Empty(Dummy<'a, 'b, S, T>)
	}

	trait Tick<T: ?Sized> {
		const IGNORED: i32;
		type Ignored<'a>;
		
		fn tick(&mut self, delta_time: T);
	}

	enum_delegate_impls! {
		ENUM_IN: {
			StateEnum<['a, 'b, S, T]> where [ 'b: 'a, T: Sized] {
				Int(State<i32>),
				UInt(State<u32>),
				Empty(Dummy<'a, 'b, S, T>),
			}
		}
		
		DELEGATES: { 
			impl<['a, 'b, S, T, G]> trait Tick<G> {
				[const IGNORED: i32 = 5]
				[type Ignored<'s> = i32]
				[fn tick(&mut self, delta_time: G)]
			}
			
			impl {
				[pub fn other(&self) -> i64]
				[pub(crate) fn and_so(&mut self)]
			}
		}
	}

	fn test<'a, 'b, S, T>(x: &mut StateEnum<'a, 'b, S, T>)
		where
			'b: 'a,
			T: Sized
	{
		x.tick(2.0);
		x.other();
		x.and_so();
	}

	impl<G> Tick<G> for State<i32> {
		const IGNORED: i32 = 5;
		type Ignored<'a> = i32;
		
		fn tick(&mut self, delta_time: G) {
			todo!()
		}
	}

	impl<G> Tick<G> for State<u32> {
		const IGNORED: i32 = 5;
		type Ignored<'a> = i32;
		
		fn tick(&mut self, delta_time: G) {
			todo!()
		}
	}

	impl<'a, 'b, S, T, G> Tick<G> for Dummy<'a, 'b, S, T>
		where
			'b: 'a,
			T: Sized
	{
		const IGNORED: i32 = 10;
		type Ignored<'s> = i32;
		
		fn tick(&mut self, delta_time: G) {
			todo!()
		}
	}

	impl State<i32> {
		fn other(&self) -> i64 {
			todo!()
		}

		fn and_so(&mut self) {
			todo!()
		}
	}

	impl State<u32> {
		fn other(&self) -> i64 {
			todo!()
		}

		fn and_so(&mut self) {
			todo!()
		}
	}

	impl<'a, 'b, S, T> Dummy<'a, 'b, S, T>
		where
			'b: 'a,
			T: Sized
	{
		fn other(&self) -> i64 {
			todo!()
		}

		fn and_so(&mut self) {
			todo!()
		}
	}
}