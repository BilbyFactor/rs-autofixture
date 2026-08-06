use std::cell::{Cell, RefCell};

use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

#[test]
fn cell_creates_successfully() {
    let mut f = Fixture::new();
    let c: Cell<u32> = f.create();
    let _value = c.get();
}

#[test]
fn cell_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<u32> = (0..10)
        .map(|_| {
            let c: Cell<u32> = f.create();
            c.get()
        })
        .collect();

    let all_same = values.windows(2).all(|w| w[0] == w[1]);

    assert!(!all_same, "expected Cell<u32> values to vary");
}

#[test]
fn cell_builder_creates_successfully() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Cell<u32>>();
    let _value = builder.create();
}

#[test]
fn refcell_creates_successfully() {
    let mut f = Fixture::new();
    let c: RefCell<u32> = f.create();
    let _value = *c.borrow();
}

#[test]
fn refcell_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<u32> = (0..10)
        .map(|_| {
            let c: RefCell<u32> = f.create();
            *c.borrow()
        })
        .collect();

    let all_same = values.windows(2).all(|w| w[0] == w[1]);

    assert!(!all_same, "expected RefCell<u32> values to vary");
}

#[test]
fn refcell_builder_creates_successfully() {
    let mut f = Fixture::new();
    let mut builder = f.build::<RefCell<u32>>();
    let _value = builder.create();
}
