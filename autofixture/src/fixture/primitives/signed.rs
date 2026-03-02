#![allow(non_camel_case_types)]

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
    primitives::{
        create_numeric_builder,
        impl_autofixture_random,
        impl_autofixture_random_dyn,
    },
};

impl_autofixture_random!(i8, i16, i32, i64, i128);
create_numeric_builder!(i8, i16, i32, i64, i128);

impl_autofixture_random_dyn!(isize);

pub struct isizeBuilder<'b> { fixture: &'b mut Fixture }

impl<'b> FixtureBuilder<'b> for isizeBuilder<'b> {
    type F = isize;

    fn new(f: &'b mut Fixture) -> Self {
        Self { fixture: f }
    }

    fn create(&mut self) -> Self::F {
        Self::F::create(self.fixture)
    }
}
