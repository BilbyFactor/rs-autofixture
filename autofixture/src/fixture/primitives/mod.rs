pub mod bool;
pub mod char;
pub mod floating_point;
pub mod signed;
pub mod unsigned;

macro_rules! impl_autofixture_random {
    ($($prim:ty), *) => { $(
        impl crate::fixture::auto_fixture::AutoFixture for $prim {
            #[inline]
            fn create(f: &mut crate::fixture::Fixture) -> Self {
                use rand::RngExt;

                f.rng().random()
            }
        }
    )* };
}

macro_rules! impl_autofixture_random_dyn {
    ($($prim:ty), *) => { $(
        impl crate::fixture::auto_fixture::AutoFixture for $prim {
            #[inline]
            fn create(f: &mut crate::fixture::Fixture) -> Self {
                use rand::RngExt;

                <$prim>::from_ne_bytes(f.rng().random())
            }
        }
    )* };
}

macro_rules! create_numeric_builder {
    ($($prim:ident), *) => { $(
        paste::paste! {
            pub struct [< $prim Builder >]<'b> {
                fixture: &'b mut crate::fixture::Fixture,
                range_condition: crate::fixture::builder::conditions::numeric::RandomRangeCondition<$prim>,
                options_condition: crate::fixture::builder::conditions::general::OptionsCondition<$prim>,
            }

            impl<'b> crate::fixture::builder::FixtureBuilder<'b> for [< $prim Builder >]<'b> {
                type F = $prim;

                #[doc = "Creates a new instance of `" [< $prim Builder >] "`, "]
                #[doc = "which can be used to apply conditions, "]
                #[doc = "making it possible to then create an `" $prim "`"]
                #[doc = "instance from said applied conditions."]
                #[doc = ""]
                #[doc = "# Arguments"]
                #[doc = " * `f` - The base fixture instance."]
                #[inline]
                fn new(f: &'b mut crate::fixture::Fixture) -> Self {
                    Self {
                        fixture: f,
                        range_condition: crate::fixture::builder::conditions::numeric::RandomRangeCondition::default(),
                        options_condition: crate::fixture::builder::conditions::general::OptionsCondition::default(),
                    }
                }

                #[doc = "Applies all conditions given to the builder to"]
                #[doc = "return an `" $prim "` instance with the applied rules."]
                #[inline]
                fn create(&mut self) -> Self::F {
                    use crate::fixture::builder::conditions::BuilderCondition;
                    
                    if let Some(c) = self.range_condition.apply(self.fixture) {
                        c
                    }
                    else if let Some(c) = self.options_condition.apply(self.fixture) {
                        c
                    }
                    else {
                        use crate::fixture::AutoFixture;

                        Self::F::create(self.fixture)
                    }
                }
            }
        }
    )* };
}

pub(crate) use impl_autofixture_random;
pub(crate) use impl_autofixture_random_dyn;
pub(crate) use create_numeric_builder;
