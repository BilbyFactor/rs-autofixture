use rs_autofixture::fixture::Fixture;
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
    let values: Vec<Uuid> = (0..10).map(|_| f.create()).collect();
    let all_same = values.windows(2).all(|w| w[0] == w[1]);
    assert!(!all_same, "expected Uuid values to vary");
}

#[test]
fn create_many_uuids() {
    let mut f = Fixture::new();
    let ids: Vec<Uuid> = f.create_many(5).collect();
    assert_eq!(ids.len(), 5);
}
