use rs_autofixture::fixture::auto_fixture::AutoFixture;
use rs_autofixture::fixture::Fixture;
use rs_autofixture::AutoFixture;

#[derive(AutoFixture)]
pub struct NamedFields {
    a: u32,
    b: i64,
    c: bool,
}

#[test]
fn named_struct_creates_successfully() {
    let mut f = Fixture::new();
    let _instance: NamedFields = f.create();
}

#[test]
fn named_struct_fields_vary() {
    let mut f = Fixture::new();

    let instances: Vec<NamedFields> = f
        .create_many(10)
        .collect();

    let all_same_a = instances
        .windows(2)
        .all(|w| w[0].a == w[1].a);

    assert!(!all_same_a, "expected field `a` to vary across 10 instances");
}

#[derive(AutoFixture)]
pub struct TupleStruct(u8, u16, u32);

#[test]
fn tuple_struct_creates_successfully() {
    let mut f = Fixture::new();
    let _instance: TupleStruct = f.create();
}

#[test]
fn tuple_struct_fields_vary() {
    let mut f = Fixture::new();

    let instances: Vec<TupleStruct> = f
        .create_many(10)
        .collect();

    let all_same = instances
        .windows(2)
        .all(|w| w[0].2 == w[1].2);
    
    assert!(!all_same, "expected tuple field 2 to vary across 10 instances");
}

#[derive(AutoFixture)]
pub struct UnitStruct;

#[test]
fn unit_struct_creates_successfully() {
    let mut f = Fixture::new();
    let _instance: UnitStruct = f.create();
}

#[derive(AutoFixture)]
pub struct Nested {
    inner: TupleStruct,
    flag: bool,
}

#[test]
fn nested_struct_creates_successfully() {
    let mut f = Fixture::new();
    let _instance: Nested = f.create();
}

#[test]
fn create_many_returns_correct_count() {
    let mut f = Fixture::new();

    let items: Vec<NamedFields> = f
        .create_many(5)
        .collect();

    assert_eq!(items.len(), 5);
}

#[test]
fn builder_creates_successfully() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();
    let mut builder = NamedFields::build(&mut f);
    let _instance = builder.create();
}
