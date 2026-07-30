use std::rc::Rc;
use std::sync::Arc;

use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

#[test]
fn box_creates_successfully() {
    let mut f = Fixture::new();
    let _value: Box<u32> = f.create();
}

#[test]
fn box_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<Box<u32>> = f
        .create_many(10)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0] == w[1]);

    assert!(!all_same, "expected Box<u32> values to vary");
}

#[test]
fn box_builder_creates_successfully() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Box<u32>>();
    let _value = builder.create();
}

#[test]
fn rc_creates_successfully() {
    let mut f = Fixture::new();
    let _value: Rc<u32> = f.create();
}

#[test]
fn rc_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<Rc<u32>> = f
        .create_many(10)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0] == w[1]);

    assert!(!all_same, "expected Rc<u32> values to vary");
}

#[test]
fn arc_creates_successfully() {
    let mut f = Fixture::new();
    let _value: Arc<u32> = f.create();
}

#[test]
fn arc_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<Arc<u32>> = f
        .create_many(10)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0] == w[1]);

    assert!(!all_same, "expected Arc<u32> values to vary");
}

#[test]
fn nested_box_creates_successfully() {
    let mut f = Fixture::new();
    let _value: Box<Vec<u32>> = f.create();
}
