use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;
use uuid::Uuid;

#[test]
fn uuid_creates_successfully() {
    let mut f = Fixture::new();
    let id: Uuid = f.create();
    assert_eq!(id.get_version_num(), 4);
}

#[test]
fn uuid_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<Uuid> = f.create_many(10).collect();

    let all_same = values.windows(2).all(|w| w[0] == w[1]);

    assert!(!all_same, "expected Uuid values to vary");
}

#[test]
fn create_many_uuids() {
    let mut f = Fixture::new();

    let ids: Vec<Uuid> = f.create_many(5).collect();

    assert_eq!(ids.len(), 5);
}

#[test]
fn uuid_builder_default_is_v4() {
    let mut f = Fixture::new();
    let id = f.build::<Uuid>().create();
    assert_eq!(id.get_version_num(), 4);
}

#[test]
fn uuid_builder_v1() {
    let mut f = Fixture::new();

    let id = f.build::<Uuid>().with_v1().create();

    assert_eq!(id.get_version_num(), 1);
}

#[test]
fn uuid_builder_v3() {
    let mut f = Fixture::new();
    let ns = Uuid::NAMESPACE_DNS;

    let id = f.build::<Uuid>().with_v3(ns, b"example.com").create();

    assert_eq!(id.get_version_num(), 3);
}

#[test]
fn uuid_builder_v5() {
    let mut f = Fixture::new();
    let ns = Uuid::NAMESPACE_URL;

    let id = f
        .build::<Uuid>()
        .with_v5(ns, b"https://example.com")
        .create();

    assert_eq!(id.get_version_num(), 5);
}

#[test]
fn uuid_builder_v6() {
    let mut f = Fixture::new();

    let id = f.build::<Uuid>().with_v6().create();

    assert_eq!(id.get_version_num(), 6);
}

#[test]
fn uuid_builder_v7() {
    let mut f = Fixture::new();

    let id = f.build::<Uuid>().with_v7().create();

    assert_eq!(id.get_version_num(), 7);
}

#[test]
fn uuid_builder_v8() {
    let mut f = Fixture::new();

    let buf = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
        0x10,
    ];

    let id = f.build::<Uuid>().with_v8(buf).create();

    assert_eq!(id.get_version_num(), 8);
}

#[test]
fn freeze_uuid_repeats_on_subsequent_creates() {
    let mut f = Fixture::new();

    let frozen: Uuid = f.freeze();
    let created: Uuid = f.create();

    assert_eq!(frozen, created);
}
