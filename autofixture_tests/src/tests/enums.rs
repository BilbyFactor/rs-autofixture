use rs_autofixture::fixture::auto_fixture::AutoFixture;
use rs_autofixture::fixture::Fixture;
use rs_autofixture::AutoFixture;

#[derive(AutoFixture, Debug, PartialEq)]
pub enum SimpleEnum {
    A,
    B,
    C,
}

#[test]
fn simple_enum_creates_successfully() {
    let mut f = Fixture::new();
    let _instance: SimpleEnum = f.create();
}

#[test]
fn simple_enum_produces_multiple_variants() {
    let mut f = Fixture::new();
    let instances: Vec<SimpleEnum> = (0..50).map(|_| f.create()).collect();

    let has_a = instances.iter().any(|v| *v == SimpleEnum::A);
    let has_b = instances.iter().any(|v| *v == SimpleEnum::B);
    let has_c = instances.iter().any(|v| *v == SimpleEnum::C);

    assert!(
        has_a && has_b && has_c,
        "expected all variants to appear in 50 samples"
    );
}

#[derive(AutoFixture)]
pub enum DataEnum {
    Named { x: u32, y: bool },
    Tuple(i16, u8),
    Unit,
}

#[test]
fn data_enum_creates_successfully() {
    let mut f = Fixture::new();
    for _ in 0..20 {
        let _instance: DataEnum = f.create();
    }
}

#[derive(AutoFixture)]
pub enum SingleVariant {
    Only(u64),
}

#[test]
fn single_variant_enum_creates_successfully() {
    let mut f = Fixture::new();
    let _instance: SingleVariant = f.create();
}

#[test]
fn enum_builder_creates_successfully() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();
    let mut builder = SimpleEnum::build(&mut f);
    let _instance = builder.create();
}
