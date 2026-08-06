use std::sync::{Mutex, RwLock};

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
    AtomicU32, AtomicU64, AtomicUsize,
};

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
    wrapper::{FromInner, WrapperBuilder},
};

macro_rules! impl_autofixture_lock_wrapper {
    ($($w:ident), *) => {
        $(
            impl<T> FromInner<T> for $w<T> {
                fn from_inner(value: T) -> Self {
                    $w::new(value)
                }
            }

            impl<T> AutoFixture for $w<T>
            where
                T: AutoFixture,
            {
                type Builder<'b> = WrapperBuilder<'b, $w<T>, T>;

                fn create(f: &mut Fixture) -> Self {
                    WrapperBuilder::<$w<T>, T>::new(f).create()
                }

                fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
                    WrapperBuilder::new(f)
                }
            }
        )*
    };
}

impl_autofixture_lock_wrapper!(Mutex, RwLock);

macro_rules! impl_autofixture_atomic {
    ($($w:ident => $t:ty), *) => {
        $(
            impl FromInner<$t> for $w {
                fn from_inner(value: $t) -> Self {
                    $w::new(value)
                }
            }

            impl AutoFixture for $w {
                type Builder<'b> = WrapperBuilder<'b, $w, $t>;

                fn create(f: &mut Fixture) -> Self {
                    WrapperBuilder::<$w, $t>::new(f).create()
                }

                fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
                    WrapperBuilder::new(f)
                }
            }
        )*
    };
}

impl_autofixture_atomic!(
    AtomicBool => bool,
    AtomicI8 => i8,
    AtomicI16 => i16,
    AtomicI32 => i32,
    AtomicI64 => i64,
    AtomicIsize => isize,
    AtomicU8 => u8,
    AtomicU16 => u16,
    AtomicU32 => u32,
    AtomicU64 => u64,
    AtomicUsize => usize
);
