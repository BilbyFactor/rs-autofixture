use tokio::sync::{Mutex, RwLock};

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
    wrapper::{FromInner, WrapperBuilder},
};

macro_rules! impl_autofixture_tokio_lock_wrapper {
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

impl_autofixture_tokio_lock_wrapper!(Mutex, RwLock);
