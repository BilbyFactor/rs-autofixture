pub mod conditions;

use crate::fixture::{AutoFixture, Fixture};

/// A trait for building `AutoFixture` types.
pub trait FixtureBuilder<'f> {
    /// The type that `create()` will return a built implementation of.
    type F: AutoFixture;

    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn new(f: &'f mut Fixture) -> Self;

    /// Returns a built implementation of type `F`, following any builder rules
    /// specified beforehand.
    fn create(&mut self) -> Self::F;
}

macro_rules! create_general_builder {
    ($($prim:ident), *) => { $(
        paste::paste! {
            pub struct [< $prim Builder >]<'b> {
                fixture: &'b mut crate::fixture::Fixture,
                options_condition: crate::fixture::builder::conditions::general::OptionsCondition<$prim>,
            }

            impl<'b> [< $prim Builder >]<'b> {
                #[doc = "Specified the builder should pick from a given set."]
                #[doc = "This call is additive, so it will continue to append new"]
                #[doc = "items to the known builder set as called."]
                #[doc = ""]
                #[doc = "# Arguments"]
                #[doc = " * `options` - A `Vec<" $prim ">` of items to be added to the builder set."]
                pub fn with_options(&mut self, options: &mut Vec<$prim>) -> &mut Self {
                    self.options_condition.options(options);

                    self
                }
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
                        options_condition: crate::fixture::builder::conditions::general::OptionsCondition::default(),
                    }
                }

                #[doc = "Applies all conditions given to the builder to"]
                #[doc = "return an `" $prim "` instance with the applied rules."]
                #[inline]
                fn create(&mut self) -> Self::F {
                    use crate::fixture::builder::conditions::BuilderCondition;
                    
                    if let Some(c) = self.options_condition.apply(self.fixture) {
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

macro_rules! create_numeric_builder {
    ($($prim:ident), *) => { $(
        paste::paste! {
            pub struct [< $prim Builder >]<'b> {
                fixture: &'b mut crate::fixture::Fixture,
                range_condition: crate::fixture::builder::conditions::numeric::RandomRangeCondition<$prim>,
                options_condition: crate::fixture::builder::conditions::general::OptionsCondition<$prim>,
            }

            impl<'b> [< $prim Builder >]<'b> {
                #[doc = "Specifies the builder should use a range on `create()`."]
                #[doc = ""]
                #[doc = "# Arguments"]
                #[doc = " * `range` - a range of values in `std::Range<T>` format (`(start..end)`),"]
                #[doc = "   where `T` is the base type for the builder."]
                #[doc = ""]
                #[doc = " # Other conditions"]
                #[doc = " * `with_options` - Any options conditions given to the builder"]
                #[doc = "   are cleared when this condition is used."]
                pub fn with_range(&mut self, range: std::ops::Range<$prim>) -> &mut Self {
                    use crate::fixture::builder::conditions::BuilderCondition;

                    self.range_condition.range(range);
                    self.options_condition.clear();

                    self
                }

                #[doc = "Specified the builder should pick from a given set."]
                #[doc = "This call is additive, so it will continue to append new"]
                #[doc = "items to the known builder set as called."]
                #[doc = ""]
                #[doc = "# Arguments"]
                #[doc = " * `options` - A `Vec<" $prim ">` of items to be added to the builder set."]
                #[doc = ""]
                #[doc = " # Other conditions"]
                #[doc = " * `with_range` - Any range conditions given to the builder"]
                #[doc = "   are cleared when this condition is used."]
                pub fn with_options(&mut self, options: &mut Vec<$prim>) -> &mut Self {
                    use crate::fixture::builder::conditions::BuilderCondition;

                    self.options_condition.options(options);
                    self.range_condition.clear();

                    self
                }
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

pub(crate) use create_general_builder;
pub(crate) use create_numeric_builder;
