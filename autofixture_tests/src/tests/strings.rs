use std::rc::Rc;
use std::sync::Arc;

use rs_autofixture::fixture::builder::FixtureBuilder;
use rs_autofixture::fixture::Fixture;

#[test]
fn string_creates_successfully() {
    let mut f = Fixture::new();
    let s: String = f.create();
    assert!(!s.is_empty());
}

#[test]
fn string_values_vary() {
    let mut f = Fixture::new();
    let a: String = f.create();
    let b: String = f.create();
    assert_ne!(a, b);
}

#[test]
fn string_default_is_uuid_format() {
    let mut f = Fixture::new();
    let s: String = f.create();
    // UUID v4 format: 8-4-4-4-12 hex chars
    assert_eq!(s.len(), 36);
    assert_eq!(s.chars().filter(|c| *c == '-').count(), 4);
}

#[test]
fn string_builder_alphabetic_generator() {
    let mut f = Fixture::new();
    let mut builder = f.build::<String>();

    let s = builder
        .with_alphabetic_generator()
        .create();

    assert!(s.chars().all(|c| c.is_alphabetic()));
}

#[test]
fn string_builder_alphanumeric_generator() {
    let mut f = Fixture::new();
    let mut builder = f.build::<String>();

    let s = builder
        .with_alphanumeric_generator()
        .create();

    assert!(s.chars().all(|c| c.is_alphanumeric()));
}

#[test]
fn string_builder_with_size() {
    let mut f = Fixture::new();
    let mut builder = f.build::<String>();

    let s = builder
        .with_alphabetic_generator()
        .with_size(32)
        .create();

    assert_eq!(s.len(), 32);
}

#[test]
fn string_builder_uuid_v4_generator() {
    let mut f = Fixture::new();
    let mut builder = f.build::<String>();

    let s = builder
        .with_uuid_v4_generator()
        .create();
    
    assert_eq!(s.len(), 36);
}

#[test]
fn box_str_creates_successfully() {
    let mut f = Fixture::new();
    let s: Box<str> = f.create();
    assert!(!s.is_empty());
}

#[test]
fn arc_str_creates_successfully() {
    let mut f = Fixture::new();
    let s: Arc<str> = f.create();
    assert!(!s.is_empty());
}

#[test]
fn rc_str_creates_successfully() {
    let mut f = Fixture::new();
    let s: Rc<str> = f.create();
    assert!(!s.is_empty());
}
