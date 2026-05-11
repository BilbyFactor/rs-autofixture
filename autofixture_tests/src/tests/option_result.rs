use rs_autofixture::fixture::builder::FixtureBuilder;
use rs_autofixture::fixture::Fixture;

#[test]
fn option_creates_successfully() {
    let mut f = Fixture::new();
    let _: Option<u32> = f.create();
}

#[test]
fn option_builder_with_forces_some() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Option<u32>>();

    let result = builder
        .with(42)
        .create();

    assert_eq!(result, Some(42));
}

#[test]
fn option_builder_without_forces_none() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Option<u32>>();

    let result = builder
        .without()
        .create();

    assert_eq!(result, None);
}

#[test]
fn result_creates_successfully() {
    let mut f = Fixture::new();
    let _: Result<u32, i32> = f.create();
}

#[test]
fn result_builder_with_ok_forces_ok() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Result<u32, i32>>();

    let result = builder
        .with_ok(Some(99))
        .create();

    assert_eq!(result, Ok(99));
}

#[test]
fn result_builder_with_ok_none_creates_random_ok() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Result<u32, i32>>();

    let result = builder
        .with_ok(None)
        .create();

    assert!(result.is_ok());
}

#[test]
fn result_builder_with_err_forces_err() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Result<u32, i32>>();

    let result = builder
        .with_err(Some(-1))
        .create();

    assert_eq!(result, Err(-1));
}

#[test]
fn result_builder_with_err_none_creates_random_err() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Result<u32, i32>>();

    let result = builder
        .with_err(None)
        .create();
    
    assert!(result.is_err());
}
