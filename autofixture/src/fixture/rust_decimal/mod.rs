//! Adds support for the [`rust_decimal`](https://docs.rs/rust_decimal/latest/rust_decimal) types.
//!
//! Provides `Autopfixture` for: `Decimal` and `RoundingStrategy`.
//!
//! Requires the `rust-decimal` feature to be enabled.

use rand::{Rng, RngExt, prelude::IndexedRandom};
use rust_decimal::{Decimal, RoundingStrategy};

use crate::fixture::{
    AutoFixture, Fixture, FixtureExt,
    builder::{FixtureBuilder, create_basic_builder},
};

create_basic_builder!(
    Decimal => DecimalBuilder,
    RoundingStrategy => RoundingStrategyBuilder
);

impl AutoFixture for Decimal {
    type Builder<'b> = DecimalBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        if let Some(frozen) = f.frozen::<Self>() {
            return frozen;
        }

        Decimal::from_parts(
            f.rng()
                .next_u32(),
            f.rng()
                .next_u32(),
            f.rng()
                .next_u32(),
            f.rng()
                .random(),
            f.rng()
                .random_range(0..=Decimal::MAX_SCALE),
        )
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        DecimalBuilder::new(f)
    }
}

impl AutoFixture for RoundingStrategy {
    type Builder<'b> = RoundingStrategyBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        if let Some(frozen) = f.frozen::<Self>() {
            return frozen;
        }

        *[
            RoundingStrategy::MidpointNearestEven,
            RoundingStrategy::MidpointAwayFromZero,
            RoundingStrategy::MidpointTowardZero,
            RoundingStrategy::ToZero,
            RoundingStrategy::AwayFromZero,
            RoundingStrategy::ToNegativeInfinity,
            RoundingStrategy::ToPositiveInfinity,
        ]
        .choose(&mut f.rng)
        .unwrap_or(&RoundingStrategy::MidpointNearestEven)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        RoundingStrategyBuilder::new(f)
    }
}
