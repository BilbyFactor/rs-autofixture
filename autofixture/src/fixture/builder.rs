use std::ops::Range;

use rand::{RngExt, distr::uniform::SampleUniform, seq::IndexedRandom};

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

/// A trait for allowing the application of custom builder conditions
/// by a `FixtureBuilder`.
/// 
/// # Type constraints
/// * `T` - must implement `AutoFixture`.
///   This already includes all Rust primitive types,
///   and can be derived on a struct or enum with `#[derive(AutoFixture)]`.
pub trait BuilderCondition<T>: Default
where
    T: AutoFixture,
{
    /// Returns a built instance of `T` following any builder rules specified
    /// before `apply()` was called.
    /// 
    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn apply(&self, f: &mut Fixture) -> T;

    /// Clears all specifies conditions.
    fn clear(&mut self);
}

/// Condition for allowing the builder call `with_range(1..10)` before apply
/// in order to constrain the random value output on `apply()`.
pub struct RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture,
{
    range: Option<Range<T>>,
}

impl<T> Default for RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture,
{
    fn default() -> Self {
        Self { range: None }
    }
}

impl<T: AutoFixture> RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture,
{
    /// Specifies the builder should use a range on `apply()`.
    /// 
    /// # Arguments
    /// * `range` - a range of values in `std::Range<T>` format (`(start..end)`),
    ///   where `T` is the base type for the builder.
    /// 
    /// # Examples
    /// `range(1..10)`, `range(..10)`, `range(-10..)`, `range(1.123..5.678)`, etc
    pub fn range(&mut self, range: Range<T>) {
        self.range = Some(range);
    }
}

impl<T> BuilderCondition<T> for RandomRangeCondition<T>
where
    T: SampleUniform + AutoFixture + PartialOrd + Clone,
{
    fn apply(&self, f: &mut Fixture) -> T {
        if let Some(range) = &self.range {
            f.rng().random_range(range.clone())
        }
        else {
            T::create(f)
        }
    }

    fn clear(&mut self) {
        self.range = None;
    }
}

/// Condition for allowing the builder call `with_options([1, 2, 3])` before apply
/// in order to constrain the output of `apply()` to be selected from one of the
/// given options.
pub struct OptionsCondition<T>
where
    T: AutoFixture,
{
    options: Vec<T>,
}

impl<T> Default for OptionsCondition<T>
where
    T: AutoFixture,
{
    fn default() -> Self {
        Self { options: vec![] }
    }
}

impl<T: AutoFixture> OptionsCondition<T>
where
    T: AutoFixture + Clone,
{
    /// Specifies the builder pick from a set of given options on `apply()`.
    /// 
    /// # Arguments
    /// * `options` - a mutable `Vec` of `T` values to add as options,
    ///   where `T` is the base type for the builder.
    /// 
    /// # Examples
    /// `options(vec![1, 3, 5, 9])`
    pub fn options(&mut self, options: &mut Vec<T>) {
        self.options.append(options);
    }
}

impl<T> BuilderCondition<T> for OptionsCondition<T>
where
    T: AutoFixture + Clone,
{
    fn apply(&self, f: &mut Fixture) -> T {
        self.options
            .choose(f.rng())
            .unwrap_or(&T::create(f))
            .to_owned()
    }

    fn clear(&mut self) {
        self.options.clear();
    }
}
