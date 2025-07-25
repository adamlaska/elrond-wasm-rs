use super::*;
use crate::arrayvec::ArrayVec;
use alloc::{
    boxed::Box,
    format,
    string::{String, ToString},
    vec::Vec,
};

impl TypeAbiFrom<()> for () {}

impl TypeAbi for () {
    type Unmanaged = Self;

    /// No another exception from the 1-type-1-output-abi rule:
    /// the unit type produces no output.
    fn output_abis(_output_names: &[&'static str]) -> OutputAbis {
        Vec::new()
    }
}

impl<T, U> TypeAbiFrom<&U> for &T where T: TypeAbiFrom<U> {}

impl<T: TypeAbi> TypeAbi for &T {
    type Unmanaged = T::Unmanaged;

    fn type_name() -> TypeName {
        T::type_name()
    }

    fn type_name_rust() -> TypeName {
        T::type_name_rust()
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T, U> TypeAbiFrom<Box<U>> for Box<T> where T: TypeAbiFrom<U> {}

impl<T: TypeAbi> TypeAbi for Box<T> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        T::type_name()
    }

    fn type_name_rust() -> TypeName {
        format!("Box<{}>", T::type_name_rust())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T, U> TypeAbiFrom<&[T]> for &[U] where T: TypeAbiFrom<U> {}

impl<T: TypeAbi> TypeAbi for &[T] {
    type Unmanaged = Vec<T::Unmanaged>;

    fn type_name() -> TypeName {
        let t_name = T::type_name();
        if t_name == "u8" {
            return "bytes".into();
        }
        let mut repr = TypeName::from("List<");
        repr.push_str(t_name.as_str());
        repr.push('>');
        repr
    }

    fn type_name_rust() -> TypeName {
        // we need to convert to an owned type
        format!("Box<[{}]>", T::type_name_rust())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T, U> TypeAbiFrom<Vec<T>> for Vec<U> where T: TypeAbiFrom<U> {}

impl<T: TypeAbi> TypeAbi for Vec<T> {
    type Unmanaged = Vec<T::Unmanaged>;

    fn type_name() -> TypeName {
        <&[T]>::type_name()
    }

    fn type_name_rust() -> TypeName {
        format!("Vec<{}>", T::type_name_rust())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi, const CAP: usize> TypeAbiFrom<ArrayVec<T, CAP>> for ArrayVec<T, CAP> {}

impl<T: TypeAbi, const CAP: usize> TypeAbi for ArrayVec<T, CAP> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        <&[T]>::type_name()
    }

    fn type_name_rust() -> TypeName {
        format!("ArrayVec<{}, {}usize>", T::type_name_rust(), CAP)
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T> TypeAbiFrom<Box<[T]>> for Box<[T]> {}

impl<T: TypeAbi> TypeAbi for Box<[T]> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        <&[T]>::type_name()
    }

    fn type_name_rust() -> TypeName {
        format!("Box<[{}]>", T::type_name_rust())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl TypeAbiFrom<String> for String {}
impl TypeAbiFrom<&String> for String {}
impl TypeAbiFrom<&str> for String {}
impl TypeAbiFrom<Box<str>> for String {}

impl TypeAbi for String {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "utf-8 string".into()
    }
}

impl TypeAbiFrom<&'static str> for &'static str {}

impl TypeAbi for &'static str {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        String::type_name()
    }

    fn type_name_rust() -> TypeName {
        "&'static str".into()
    }
}

impl TypeAbiFrom<Box<str>> for Box<str> {}
impl TypeAbiFrom<&str> for Box<str> {}
impl TypeAbiFrom<String> for Box<str> {}

impl TypeAbi for Box<str> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        String::type_name()
    }

    fn type_name_rust() -> TypeName {
        "Box<str>".into()
    }
}

macro_rules! type_abi_name_only {
    ($ty:ty, $name:expr) => {
        impl TypeAbiFrom<$ty> for $ty {}
        impl TypeAbiFrom<&$ty> for $ty {}

        impl TypeAbi for $ty {
            type Unmanaged = Self;

            fn type_name() -> TypeName {
                TypeName::from($name)
            }

            fn provide_type_descriptions<TDC: TypeDescriptionContainer>(_: &mut TDC) {}
        }
    };
}

type_abi_name_only!(u8, "u8");
type_abi_name_only!(u16, "u16");
type_abi_name_only!(u32, "u32");
type_abi_name_only!(usize, "u32");
type_abi_name_only!(u64, "u64");
type_abi_name_only!(u128, "u128");

type_abi_name_only!(i8, "i8");
type_abi_name_only!(i16, "i16");
type_abi_name_only!(i32, "i32");
type_abi_name_only!(isize, "i32");
type_abi_name_only!(i64, "i64");

type_abi_name_only!(core::num::NonZeroUsize, "NonZeroUsize");
type_abi_name_only!(bool, "bool");
type_abi_name_only!(f64, "f64");

// Unsigned integer types: the contract can return a smaller capacity result and and we can interpret it as a larger capacity type.

impl TypeAbiFrom<u64> for u128 {}
impl TypeAbiFrom<usize> for u128 {}
impl TypeAbiFrom<u32> for u128 {}
impl TypeAbiFrom<u16> for u128 {}
impl TypeAbiFrom<u8> for u128 {}

impl TypeAbiFrom<usize> for u64 {}
impl TypeAbiFrom<u32> for u64 {}
impl TypeAbiFrom<u16> for u64 {}
impl TypeAbiFrom<u8> for u64 {}

impl TypeAbiFrom<usize> for u32 {}
impl TypeAbiFrom<u16> for u32 {}
impl TypeAbiFrom<u8> for u32 {}

impl TypeAbiFrom<u32> for usize {}
impl TypeAbiFrom<u16> for usize {}
impl TypeAbiFrom<u8> for usize {}

impl TypeAbiFrom<u8> for u16 {}

// Signed, the same.

impl TypeAbiFrom<isize> for i64 {}
impl TypeAbiFrom<i32> for i64 {}
impl TypeAbiFrom<i16> for i64 {}
impl TypeAbiFrom<i8> for i64 {}

impl TypeAbiFrom<isize> for i32 {}
impl TypeAbiFrom<i16> for i32 {}
impl TypeAbiFrom<i8> for i32 {}

impl TypeAbiFrom<i32> for isize {}
impl TypeAbiFrom<i16> for isize {}
impl TypeAbiFrom<i8> for isize {}

impl TypeAbiFrom<i8> for i16 {}

impl<T, U> TypeAbiFrom<Option<U>> for Option<T> where T: TypeAbiFrom<U> {}

impl<T> TypeAbi for Option<T>
where
    T: TypeAbi,
{
    type Unmanaged = Option<T::Unmanaged>;

    fn type_name() -> TypeName {
        format!("Option<{}>", T::type_name())
    }

    fn type_name_rust() -> TypeName {
        format!("Option<{}>", T::type_name_rust())
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

impl<T: TypeAbi, E> TypeAbiFrom<Self> for Result<T, E> {}

impl<T: TypeAbi, E> TypeAbi for Result<T, E> {
    type Unmanaged = Result<T::Unmanaged, E>;

    fn type_name() -> TypeName {
        T::type_name()
    }

    fn type_name_rust() -> TypeName {
        format!(
            "Result<{}, {}>",
            T::type_name_rust(),
            core::any::type_name::<E>()
        )
    }

    /// Similar to the SCResult implementation.
    fn output_abis(output_names: &[&'static str]) -> OutputAbis {
        T::output_abis(output_names)
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}

macro_rules! tuple_impls {
    ($($len:expr => ($($n:tt $name:ident)+))+) => {
        $(
            impl<$($name),+> TypeAbiFrom<Self> for ($($name,)+)
            where
                $($name: TypeAbi,)+
            {}

            impl<$($name),+> TypeAbi for ($($name,)+)
            where
                $($name: TypeAbi,)+
            {
                type Unmanaged = ($($name::Unmanaged,)+);

                fn type_name() -> TypeName {
                    let mut repr = TypeName::from("tuple<");
                    $(
                        if $n > 0 {
                            repr.push(',');
                        }
                        repr.push_str($name::type_name().as_str());
                    )+
                    repr.push('>');
                    repr
                }

                fn type_name_rust() -> TypeName {
                    let mut repr = TypeName::from("(");
                    $(
                        if $n > 0 {
                            repr.push_str(", ");
                        }
                        repr.push_str($name::type_name_rust().as_str());
                    )+
                    repr.push(')');
                    repr
                }

                fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
                    $(
                        $name::provide_type_descriptions(accumulator);
                    )+
                }
            }
        )+
    }
}

tuple_impls! {
    1  => (0 T0)
    2  => (0 T0 1 T1)
    3  => (0 T0 1 T1 2 T2)
    4  => (0 T0 1 T1 2 T2 3 T3)
    5  => (0 T0 1 T1 2 T2 3 T3 4 T4)
    6  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5)
    7  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6)
    8  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7)
    9  => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8)
    10 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9)
    11 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10)
    12 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11)
    13 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12)
    14 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13)
    15 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14)
    16 => (0 T0 1 T1 2 T2 3 T3 4 T4 5 T5 6 T6 7 T7 8 T8 9 T9 10 T10 11 T11 12 T12 13 T13 14 T14 15 T15)
}

impl<T, U, const N: usize> TypeAbiFrom<[U; N]> for [T; N] where T: TypeAbiFrom<U> {}

impl<T: TypeAbi, const N: usize> TypeAbi for [T; N] {
    type Unmanaged = [T::Unmanaged; N];

    fn type_name() -> TypeName {
        let mut repr = TypeName::from("array");
        repr.push_str(N.to_string().as_str());
        repr.push('<');
        repr.push_str(T::type_name().as_str());
        repr.push('>');
        repr
    }

    fn type_name_rust() -> TypeName {
        let mut repr = TypeName::from("[");
        repr.push_str(T::type_name_rust().as_str());
        repr.push_str("; ");
        repr.push_str(N.to_string().as_str());
        repr.push(']');
        repr
    }

    fn provide_type_descriptions<TDC: TypeDescriptionContainer>(accumulator: &mut TDC) {
        T::provide_type_descriptions(accumulator);
    }
}
