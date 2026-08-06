use lettre::Address;
use lettre::address::Envelope;
use lettre::message::{Mailbox, Mailboxes};
use lettre::transport::smtp::authentication::{Credentials, Mechanism};
use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

#[test]
fn address_creates_successfully() {
    let mut f = Fixture::new();
    let addr: Address = f.create();

    assert!(
        !addr
            .user()
            .is_empty()
    );
    assert!(
        !addr
            .domain()
            .is_empty()
    );
}

#[test]
fn address_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<Address> = f
        .create_many(10)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0] == w[1]);
    assert!(!all_same, "expected Address values to vary");
}

#[test]
fn address_has_valid_domain() {
    let mut f = Fixture::new();
    let addr: Address = f.create();

    assert!(
        addr.domain()
            .contains('.'),
        "domain should contain a dot: {}",
        addr.domain(),
    );
}

#[test]
fn address_builder_creates_successfully() {
    let mut f = Fixture::new();
    let addr = f
        .build::<Address>()
        .create();

    assert!(
        !addr
            .user()
            .is_empty()
    );
}

#[test]
fn create_many_addresses() {
    let mut f = Fixture::new();

    let addrs: Vec<Address> = f
        .create_many(5)
        .collect();

    assert_eq!(addrs.len(), 5);
}

#[test]
fn create_many_addresses_from_builder() {
    let mut f = Fixture::new();

    let addrs: Vec<Address> = f
        .build::<Address>()
        .create_many(5)
        .collect();

    assert_eq!(addrs.len(), 5);
}

#[test]
fn mailbox_creates_successfully() {
    let mut f = Fixture::new();

    let mb: Mailbox = f.create();

    assert!(
        !mb.email
            .user()
            .is_empty()
    );
}

#[test]
fn mailbox_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<Mailbox> = f
        .create_many(10)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0].email == w[1].email);

    assert!(!all_same, "expected Mailbox values to vary");
}

// --- Mailboxes ---

#[test]
fn mailboxes_creates_successfully() {
    let mut f = Fixture::new();
    let mbs: Mailboxes = f.create();
    assert!(
        !mbs.into_iter()
            .collect::<Vec<_>>()
            .is_empty()
    );
}

// --- Envelope ---

#[test]
fn envelope_creates_successfully() {
    let mut f = Fixture::new();
    let env: Envelope = f.create();
    assert!(
        env.from()
            .is_some()
    );
    assert!(
        !env.to()
            .is_empty()
    );
}

#[test]
fn envelope_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<Envelope> = f
        .create_many(10)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0].from() == w[1].from());

    assert!(!all_same, "expected Envelope values to vary");
}

#[test]
fn credentials_creates_successfully() {
    let mut f = Fixture::new();
    let _creds: Credentials = f.create();
}

#[test]
fn mechanism_creates_successfully() {
    let mut f = Fixture::new();
    let _mech: Mechanism = f.create();
}

#[test]
fn mechanism_values_vary() {
    let mut f = Fixture::new();

    let values: Vec<Mechanism> = f
        .create_many(20)
        .collect();

    let all_same = values
        .windows(2)
        .all(|w| w[0] == w[1]);

    assert!(
        !all_same,
        "expected Mechanism values to vary across 20 instances"
    );
}

#[test]
fn freeze_address_repeats_on_subsequent_creates() {
    let mut f = Fixture::new();

    let frozen: Address = f.freeze();
    let created: Address = f.create();

    assert_eq!(frozen, created);
}
