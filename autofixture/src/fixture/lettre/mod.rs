//! AutoFixture implementations for [`lettre`](https://docs.rs/lettre) email types.
//!
//! Provides `AutoFixture` for: `Address`, `Mailbox`, `Mailboxes`,
//! `Envelope`, `Credentials`, and `Mechanism`.
//!
//! Requires the `lettre` feature to be enabled.

use lettre::{
    Address,
    address::Envelope,
    message::Mailbox,
    message::Mailboxes,
    transport::smtp::authentication::{Credentials, Mechanism},
};
use rand::RngExt;

use crate::fixture::{
    AutoFixture, Fixture, FixtureExt,
    builder::{FixtureBuilder, create_basic_builder},
};

create_basic_builder!(
    Address => AddressBuilder,
    Mailbox => MailboxBuilder,
    Mailboxes => MailboxesBuilder,
    Envelope => EnvelopeBuilder,
    Credentials => CredentialsBuilder,
    Mechanism => MechanismBuilder
);

fn random_domain(f: &mut Fixture) -> String {
    let tlds = ["com", "org", "net", "io", "dev"];
    let tld_idx = f.rng().random_range(0..tlds.len());
    let len = f.rng().random_range(4..10u32);

    let domain: String = (0..len)
        .map(|_| (b'a' + f.rng().random_range(0..26u8)) as char)
        .collect();

    format!("{}.{}", domain, tlds[tld_idx])
}

fn random_user(f: &mut Fixture) -> String {
    let len = f.rng().random_range(4..12u32);

    (0..len)
        .map(|_| (b'a' + f.rng().random_range(0..26u8)) as char)
        .collect()
}

impl AutoFixture for Address {
    type Builder<'b> = AddressBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let user = random_user(f);
        let domain = random_domain(f);
        Address::new(user, domain).expect("generated address parts should be valid")
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        AddressBuilder::new(f)
    }
}

impl AutoFixture for Mailbox {
    type Builder<'b> = MailboxBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let has_name = f.rng().random_range(0..2u8) == 1;
        let name = if has_name {
            let len = f.rng().random_range(3..12u32);
            let n: String = (0..len)
                .map(|_| (b'a' + f.rng().random_range(0..26u8)) as char)
                .collect();
            Some(n)
        } else {
            None
        };

        let address = Address::create(f);
        Mailbox::new(name, address)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        MailboxBuilder::new(f)
    }
}

impl AutoFixture for Mailboxes {
    type Builder<'b> = MailboxesBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let count = f.rng().random_range(1..4u32);
        let mailboxes: Vec<Mailbox> = (0..count).map(|_| Mailbox::create(f)).collect();

        let mut mbs = Mailboxes::new();
        for mb in mailboxes {
            mbs.push(mb);
        }
        mbs
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        MailboxesBuilder::new(f)
    }
}

impl AutoFixture for Envelope {
    type Builder<'b> = EnvelopeBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let from = Address::create(f);
        let to = Address::create(f);
        Envelope::new(Some(from), vec![to]).expect("generated envelope should be valid")
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        EnvelopeBuilder::new(f)
    }
}

impl AutoFixture for Credentials {
    type Builder<'b> = CredentialsBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let user = random_user(f);
        let pass_len = f.rng().random_range(8..20u32);
        let password: String = (0..pass_len)
            .map(|_| (b'!' + f.rng().random_range(0..94u8)) as char)
            .collect();

        Credentials::new(user, password)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CredentialsBuilder::new(f)
    }
}

impl AutoFixture for Mechanism {
    type Builder<'b> = MechanismBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        match f.rng().random_range(0..3u8) {
            0 => Mechanism::Plain,
            1 => Mechanism::Login,
            _ => Mechanism::Xoauth2,
        }
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        MechanismBuilder::new(f)
    }
}
