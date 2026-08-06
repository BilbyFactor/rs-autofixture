use rs_autofixture::fixture::Fixture;
use rust_decimal::{Decimal, RoundingStrategy};

// --- Decimal ---

#[test]
fn decimal_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<Decimal> = f.create_many(10).collect();

    let all_same = values.windows(2).all(|w| w[0] == w[1]);

    assert!(!all_same, "expected Decimal values to vary");
}

// --- RoundingStrategy ---

#[test]
fn rounding_strategy_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<RoundingStrategy> = f.create_many(10).collect();

    let all_same = values.windows(2).all(|w| w[0] == w[1]);

    assert!(!all_same, "expected RoundingStrategy values to vary");
}

#[test]
fn freeze_decimal_repeats_on_subsequent_creates() {
    let mut f = Fixture::new();

    let frozen: Decimal = f.freeze();
    let created: Decimal = f.create();

    assert_eq!(frozen, created);
}
