use rs_autofixture::AutoFixture;
use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::auto_fixture::AutoFixture;

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

    assert!(
        !all_same_a,
        "expected field `a` to vary across 10 instances"
    );
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

    assert!(
        !all_same,
        "expected tuple field 2 to vary across 10 instances"
    );
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

#[test]
fn named_struct_builder_with_field_fixes_value() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();

    let instance = f
        .build::<NamedFields>()
        .with_a(42)
        .with_b(-7)
        .create();

    assert_eq!(instance.a, 42);
    assert_eq!(instance.b, -7);
}

#[test]
fn named_struct_builder_leaves_unset_fields_random() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();

    let instances: Vec<NamedFields> = (0..10)
        .map(|_| {
            f.build::<NamedFields>()
                .with_a(1)
                .create()
        })
        .collect();

    let all_same_b = instances
        .windows(2)
        .all(|w| w[0].b == w[1].b);

    assert!(
        !all_same_b,
        "expected unset field `b` to still vary across 10 instances"
    );
}

#[test]
fn tuple_struct_builder_with_field_fixes_value() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();

    let instance = f
        .build::<TupleStruct>()
        .with_0(1)
        .with_2(99)
        .create();

    assert_eq!(instance.0, 1);
    assert_eq!(instance.2, 99);
}

#[derive(AutoFixture)]
pub struct EmptyableFields {
    label: String,
    nickname: Option<String>,
    tags: Vec<String>,
}

#[derive(AutoFixture)]
pub struct EmptyableTupleFields(Option<u32>, Vec<u8>);

#[test]
fn named_struct_builder_without_string_field_is_blank() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();

    let instance = f
        .build::<EmptyableFields>()
        .without_label()
        .create();

    assert_eq!(instance.label, "");
}

#[test]
fn named_struct_builder_without_option_field_is_none() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();

    let instance = f
        .build::<EmptyableFields>()
        .without_nickname()
        .create();

    assert_eq!(instance.nickname, None);
}

#[test]
fn named_struct_builder_without_vec_field_is_empty() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();

    let instance = f
        .build::<EmptyableFields>()
        .without_tags()
        .create();

    assert!(
        instance
            .tags
            .is_empty()
    );
}

#[test]
fn named_struct_builder_combines_with_and_without() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();

    let instance = f
        .build::<EmptyableFields>()
        .with_label("fixed".to_string())
        .without_nickname()
        .without_tags()
        .create();

    assert_eq!(instance.label, "fixed");
    assert_eq!(instance.nickname, None);
    assert!(
        instance
            .tags
            .is_empty()
    );
}

#[test]
fn tuple_struct_builder_without_field_uses_empty_value() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();

    let instance = f
        .build::<EmptyableTupleFields>()
        .without_0()
        .without_1()
        .create();

    assert_eq!(instance.0, None);
    assert!(
        instance
            .1
            .is_empty()
    );
}
