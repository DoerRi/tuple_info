pub trait TupleInfo {
    const LEN: usize;
    type DeconstructedReference<'a>;
    type MutDeconstructedReference<'a>
    where
        Self: 'a;
    fn len() -> usize {
        Self::LEN
    }
    fn index<T: 'static>() -> Option<usize>;
    fn types() -> Vec<std::any::TypeId>;
    fn deconstruct<'a>(&'a self) -> Self::DeconstructedReference<'a>;
    fn mut_deconstruct<'a>(&'a mut self) -> Self::MutDeconstructedReference<'a>;
    fn try_deconstruction<'a>(
        anies: &[&'a dyn std::any::Any],
    ) -> Option<Self::DeconstructedReference<'a>>;
    fn try_mut_deconstruction<'a>(
        anies: Vec<&'a mut dyn std::any::Any>,
    ) -> Option<Self::MutDeconstructedReference<'a>>;
    fn get(&self, index: usize) -> Option<&dyn std::any::Any>;
    fn get_mut(&mut self, index: usize) -> Option<&mut dyn std::any::Any>;
    fn foreach_type<'a, T: ForeachTypeStrategie<'a>>(input: T::Input)
    -> (Vec<T::Output>, T::Input);
    fn map_type<'a, S: MapTypeStrategie<'a>>(self, input: S::Input) -> (Vec<S::Output>, S::Input);
    fn as_anyies<'a>(&'a self) -> Vec<&'a dyn std::any::Any>;
    fn as_mut_anyies<'a>(&'a mut self) -> Vec<&'a mut dyn std::any::Any>;
}
pub trait ForeachTypeStrategie<'a> {
    type Output;
    type Input;
    fn action<T: 'static>(input: Self::Input, type_index: usize) -> (Self::Output, Self::Input);
}
pub trait MapTypeStrategie<'a> {
    type Output;
    type Input;
    fn map<T: 'static>(
        element: T,
        input: Self::Input,
        type_index: usize,
    ) -> (Self::Output, Self::Input);
}

/// ## Sample Implementation
/// ```
impl<A: 'static, B: 'static> TupleInfo for (A, B) {
    const LEN: usize = 2;

    type DeconstructedReference<'a> = (&'a A, &'a B);

    type MutDeconstructedReference<'a>
        = (&'a mut A, &'a mut B)
    where
        Self: 'a;

    fn types() -> Vec<std::any::TypeId> {
        vec![std::any::TypeId::of::<A>(), std::any::TypeId::of::<B>()]
    }

    fn deconstruct<'a>(&'a self) -> Self::DeconstructedReference<'a> {
        let (a, b) = self;
        (a, b)
    }

    fn mut_deconstruct<'a>(&'a mut self) -> Self::MutDeconstructedReference<'a> {
        let (a, b) = self;
        (a, b)
    }

    fn get(&self, index: usize) -> Option<&dyn std::any::Any> {
        match index {
            0 => Some(&self.0 as &dyn std::any::Any),
            1 => Some(&self.1 as &dyn std::any::Any),
            _ => None,
        }
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut dyn std::any::Any> {
        match index {
            0 => Some(&mut self.0 as &mut dyn std::any::Any),
            1 => Some(&mut self.1 as &mut dyn std::any::Any),
            _ => None,
        }
    }

    fn try_deconstruction<'a>(
        anies: &[&'a dyn std::any::Any],
    ) -> Option<Self::DeconstructedReference<'a>> {
        Some((anies[0].downcast_ref::<A>()?, anies[1].downcast_ref::<B>()?))
    }

    fn try_mut_deconstruction<'a>(
        mut anies: Vec<&'a mut dyn std::any::Any>,
    ) -> Option<Self::MutDeconstructedReference<'a>> {
        anies.reverse();
        Some((
            anies.pop()?.downcast_mut::<A>()?,
            anies.pop()?.downcast_mut::<B>()?,
        ))
    }
    fn foreach_type<'a, T: ForeachTypeStrategie<'a>>(
        input: T::Input,
    ) -> (Vec<T::Output>, T::Input) {
        let mut output = Vec::new();
        let (out, input) = T::action::<A>(input, 0);
        output.push(out);
        let (out, input) = T::action::<B>(input, 1);
        output.push(out);
        (output, input)
    }

    fn map_type<'a, S: MapTypeStrategie<'a>>(self, input: S::Input) -> (Vec<S::Output>, S::Input) {
        let mut output = Vec::new();
        let (out, input) = S::map(self.0, input, 0);
        output.push(out);
        let (out, input) = S::map(self.1, input, 1);
        output.push(out);
        (output, input)
    }

    fn index<T: 'static>() -> Option<usize> {
        match std::any::TypeId::of::<T>() {
            t if t == std::any::TypeId::of::<A>() => Some(0),
            t if t == std::any::TypeId::of::<B>() => Some(1),
            _ => None,
        }
    }

    fn as_anyies<'a>(&'a self) -> Vec<&'a dyn std::any::Any> {
        vec![&self.0 as &dyn std::any::Any, &self.1 as &dyn std::any::Any]
    }

    fn as_mut_anyies<'a>(&'a mut self) -> Vec<&'a mut dyn std::any::Any> {
        vec![
            &mut self.0 as &mut dyn std::any::Any,
            &mut self.1 as &mut dyn std::any::Any,
        ]
    }
}
/// ```
macro_rules! impl_tuple_info {
    ($($name:ident : $idx:tt),+) => {
        impl<$($name: 'static),+> TupleInfo for ($($name,)+) {
            const LEN: usize = count_idents!($($name),+);

            type DeconstructedReference<'a> = ($(&'a $name,)+);
            type MutDeconstructedReference<'a> = ($(&'a mut $name,)+) where Self: 'a;

            fn index<T:'static>()->Option<usize>{
                match std::any::TypeId::of::<T>(){
                    $(t if t == std::any::TypeId::of::<$name>() => Some($idx),)*
                    _ => None
                }
            }

            fn types() -> Vec< std::any::TypeId> {
                vec![$( std::any::TypeId::of::<$name>()),+]
            }


            fn deconstruct<'a>(&'a self) -> Self::DeconstructedReference<'a> {
                ($( &self.$idx, )+)
            }

            fn mut_deconstruct<'a>(&'a mut self) -> Self::MutDeconstructedReference<'a> {
                ($( &mut self.$idx, )+)
            }

            fn get(&self, index: usize) -> Option<&dyn  std::any::Any> {
                match index {
                    $($idx => Some(&self.$idx as &dyn  std::any::Any),)+
                    _ => None,
                }
            }

            fn get_mut(&mut self, index: usize) -> Option<&mut dyn  std::any::Any> {
                match index {
                    $($idx => Some(&mut self.$idx as &mut dyn  std::any::Any),)+
                    _ => None,
                }
            }

            fn try_deconstruction<'a>(anies: &[&'a dyn  std::any::Any]) -> Option<Self::DeconstructedReference<'a>> {
                Some((
                    $(anies[$idx].downcast_ref::<$name>()?,)+
                ))
            }

            fn try_mut_deconstruction<'a>(
                mut anies: Vec<&'a mut dyn  std::any::Any>,
            ) -> Option<Self::MutDeconstructedReference<'a>> {
                anies.reverse();
                Some((
                    $(anies.pop()?.downcast_mut::<$name>()?,)+
                ))
            }


            fn foreach_type<'a,T: ForeachTypeStrategie<'a>>(input:T::Input) -> (Vec<T::Output>,T::Input){
                let mut output = vec![];
                $(
                    let (out,input) = T::action::<$name>( input, $idx);
                    output.push(out);
                )*

                (output,input)
            }


            fn map_type<'a, S: MapTypeStrategie<'a>>(self, input: S::Input) -> (Vec<S::Output>,S::Input){
                let mut output = vec![];

                $(
                    let (out,input) = S::map(self.$idx, input, $idx);
                    output.push(out);
                )*

                (output,input)
            }

            fn as_anyies<'a>(&'a self) -> Vec<&'a dyn  std::any::Any>{
                vec![
                    $(
                        (&self.$idx) as & dyn  std::any::Any
                    ),*
                ]
            }
            fn as_mut_anyies<'a>(&'a mut self) -> Vec<&'a mut dyn  std::any::Any>{
                vec![
                    $(
                        (&mut self.$idx) as &mut dyn  std::any::Any
                    ),*
                ]
            }
        }
    };
}

// count helper
macro_rules! count_idents {
    ($($name:ident),+) => { <[()]>::len(&[$(count_idents!(@sub $name)),+]) };
    (@sub $x:ident) => { () };
}

// Generate up to N = 15
//impl_tuple_info!(A:0, B:1);
impl_tuple_info!(A:0, B:1, C:2);
impl_tuple_info!(A:0, B:1, C:2, D:3);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14);
impl_tuple_info!(A:0, B:1, C:2, D:3, E:4, F:5, G:6, H:7, I:8, J:9, K:10, L:11, M:12, N:13, O:14, P:15);
