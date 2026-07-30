use std::rc::Rc;
use std::sync::Arc;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
    wrapper::{FromInner, WrapperBuilder},
};

macro_rules! impl_autofixture_wrapper {
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

impl_autofixture_wrapper!(Box, Rc, Arc);
