#![allow(non_camel_case_types)]

use crate::fixture::{auto_fixture::impl_autofixture_random, builder::create_numeric_builder};

create_numeric_builder!(f32, f64);
impl_autofixture_random!(f32 => f32Builder, f64 => f64Builder);

// `f16`/`f128` don't have a `rand` random implementation yet, since they are
// still unsupported. So `create_numeric_builder!` can't be reused here.
#[cfg(feature = "nightly-float")]
pub mod nightly_float {
    use std::ops::{Bound, RangeBounds};

    use crate::fixture::{
        Fixture, FixtureExt,
        auto_fixture::AutoFixture,
        builder::{
            FixtureBuilder,
            conditions::{BuilderCondition, general::OptionsCondition},
        },
    };

    macro_rules! impl_autofixture_nightly_float {
        ($($prim:ident => $b:ident), *) => {
            $(
                paste::paste! {
                    impl AutoFixture for $prim {
                        type Builder<'b> = $b<'b>;

                        #[inline]
                        fn create(f: &mut Fixture) -> Self {
                            use rand::RngExt;

                            f.rng().random::<f64>() as $prim
                        }

                        #[inline]
                        fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
                            $b::new(f)
                        }
                    }

                    #[doc = "Builder for `" $prim "`, following the same `with_range`/`with_options` "]
                    #[doc = "shape as the other numeric builders."]
                    pub struct $b<'b> {
                        fixture: &'b mut Fixture,
                        range: Option<($prim, $prim)>,
                        options_condition: OptionsCondition<$prim>,
                    }

                    impl<'b> $b<'b> {
                        #[doc = "Specifies the builder should use a range on `create()`."]
                        pub fn with_range<R: RangeBounds<$prim>>(&mut self, range: R) -> &mut Self {
                            let start = match range.start_bound() {
                                Bound::Included(&s) => s,
                                Bound::Excluded(_) => {
                                    panic!("a starting `Bound::Excluded` cannot be parsed...")
                                }
                                Bound::Unbounded => $prim::MIN,
                            };

                            let end = match range.end_bound() {
                                Bound::Included(&e) | Bound::Excluded(&e) => e,
                                Bound::Unbounded => $prim::MAX,
                            };

                            self.range = Some((start, end));
                            self.options_condition.clear();

                            self
                        }

                        #[doc = "Specifies the builder should pick from a given set."]
                        pub fn with_options(&mut self, options: &mut Vec<$prim>) -> &mut Self {
                            self.options_condition.options(options);
                            self.range = None;

                            self
                        }
                    }

                    impl<'b> FixtureBuilder<'b> for $b<'b> {
                        type F = $prim;

                        #[inline]
                        fn new(f: &'b mut Fixture) -> Self {
                            Self {
                                fixture: f,
                                range: None,
                                options_condition: OptionsCondition::default(),
                            }
                        }

                        #[inline]
                        fn create(&mut self) -> Self::F {
                            use rand::RngExt;

                            if let Some((start, end)) = self.range {
                                let t: f64 = self.fixture.rng().random();

                                start + (end - start) * (t as $prim)
                            }
                            else if let Some(v) = self.options_condition.apply(self.fixture) {
                                v
                            }
                            else {
                                Self::F::create(self.fixture)
                            }
                        }
                    }
                }
            )*
        };
    }

    impl_autofixture_nightly_float!(f16 => f16Builder, f128 => f128Builder);
}
