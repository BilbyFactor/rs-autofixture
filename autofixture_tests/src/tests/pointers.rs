use std::rc::Rc;
use std::sync::Arc;

use rs_autofixture::AutoFixture;
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

#[derive(AutoFixture)]
pub struct Author {
    name: String,
}

#[derive(AutoFixture)]
pub struct Book {
    title: String,
    author: Rc<Author>,
}

#[test]
fn freeze_rc_shares_same_instance() {
    let mut f = Fixture::new();

    let frozen_author: Rc<Author> = f.freeze();

    let book1: Book = f.create();
    let book2: Book = f.create();

    assert!(Rc::ptr_eq(&frozen_author, &book1.author));
    assert!(Rc::ptr_eq(&frozen_author, &book2.author));
    assert!(Rc::ptr_eq(&book1.author, &book2.author));
}

#[test]
fn freeze_arc_shares_same_instance() {
    let mut f = Fixture::new();

    let frozen: Arc<u32> = f.freeze();

    let a: Arc<u32> = f.create();
    let b: Arc<u32> = f.create();

    assert!(Arc::ptr_eq(&frozen, &a));
    assert!(Arc::ptr_eq(&frozen, &b));
}
