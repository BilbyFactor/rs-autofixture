use rs_autofixture::fixture::auto_fixture::AutoFixture;
use rs_autofixture::fixture::Fixture;
use rs_autofixture::AutoFixture;

#[derive(AutoFixture, Clone, Copy)]
pub union SimpleUnion {
    a: u32,
    b: i32,
}

#[test]
fn simple_union_creates_successfully() {
    let mut f = Fixture::new();
    let _instance: SimpleUnion = f.create();
}

#[test]
fn simple_union_fields_vary() {
    let mut f = Fixture::new();
    let instances: Vec<SimpleUnion> = (0..10).map(|_| f.create()).collect();
    let all_same = instances.windows(2).all(|w| unsafe { w[0].a == w[1].a });
    assert!(!all_same, "expected union values to vary across 10 instances");
}

#[derive(AutoFixture, Clone, Copy)]
pub union SingleFieldUnion {
    only: u64,
}

#[test]
fn single_field_union_creates_successfully() {
    let mut f = Fixture::new();
    let _instance: SingleFieldUnion = f.create();
}

#[test]
fn union_builder_creates_successfully() {
    use rs_autofixture::fixture::builder::FixtureBuilder;

    let mut f = Fixture::new();
    let mut builder = SimpleUnion::build(&mut f);
    let _instance = builder.create();
}
