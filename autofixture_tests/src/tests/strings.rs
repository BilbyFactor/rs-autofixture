use std::rc::Rc;
use std::sync::Arc;

use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

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

    // Check UUID v4 format:
    assert_eq!(s.len(), 36);
    assert_eq!(
        s.chars()
            .filter(|c| *c == '-')
            .count(),
        4
    );
}

#[test]
fn string_builder_alphabetic_generator() {
    let mut f = Fixture::new();

    let s = f
        .build::<String>()
        .with_alphabetic_generator()
        .create();

    assert!(
        s.chars()
            .all(|c| c.is_alphabetic())
    );
}

#[test]
fn string_builder_alphanumeric_generator() {
    let mut f = Fixture::new();

    let s = f
        .build::<String>()
        .with_alphanumeric_generator()
        .create();

    assert!(
        s.chars()
            .all(|c| c.is_alphanumeric())
    );
}

#[test]
fn string_builder_with_size() {
    let mut f = Fixture::new();

    let s = f
        .build::<String>()
        .with_alphabetic_generator()
        .with_size(32)
        .create();

    assert_eq!(s.len(), 32);
}

#[test]
fn string_builder_uuid_v4_generator() {
    let mut f = Fixture::new();

    let s = f
        .build::<String>()
        .with_uuid_v4_generator()
        .create();

    assert_eq!(s.len(), 36);
}

#[test]
fn string_builder_domain_generator() {
    let mut f = Fixture::new();

    let s = f
        .build::<String>()
        .with_domain_generator()
        .create();

    assert!(s.contains('.'), "domain should contain a dot: {s}");

    let parts: Vec<&str> = s
        .splitn(2, '.')
        .collect();
    assert!(
        parts[0]
            .chars()
            .all(|c| c.is_ascii_lowercase())
    );
    assert!(["com", "org", "net", "io", "dev"].contains(&parts[1]));
}

#[test]
fn string_builder_domain_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<String> = f
        .build::<String>()
        .with_domain_generator()
        .create_many(10)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0] == w[1]);
    assert!(!all_same, "expected domain values to vary");
}

#[test]
fn string_builder_url_generator() {
    let mut f = Fixture::new();

    let s = f
        .build::<String>()
        .with_url_generator()
        .create();

    assert!(
        s.starts_with("http://") || s.starts_with("https://"),
        "url should start with a scheme: {s}",
    );

    assert!(s.contains('.'), "url should contain a domain dot: {s}");
    assert_eq!(
        s.matches('/')
            .count(),
        3,
        "url should have scheme + path slashes: {s}"
    );
}

#[test]
fn string_builder_url_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<String> = f
        .build::<String>()
        .with_url_generator()
        .create_many(10)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0] == w[1]);
    assert!(!all_same, "expected url values to vary");
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

#[test]
fn freeze_string_repeats_on_subsequent_creates() {
    let mut f = Fixture::new();

    let frozen: String = f.freeze();
    let created: String = f.create();

    assert_eq!(frozen, created);
}

#[test]
fn freeze_rc_str_shares_same_instance() {
    let mut f = Fixture::new();

    let frozen: Rc<str> = f.freeze();
    let created: Rc<str> = f.create();

    assert!(Rc::ptr_eq(&frozen, &created));
}
