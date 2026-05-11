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
