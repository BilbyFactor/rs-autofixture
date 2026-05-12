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
    AutoFixture,
    Fixture,
    FixtureExt,
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

impl AutoFixture for Address {
    type Builder<'b> = AddressBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let size = f.rng().random_range(4..12);

        let user: String = f
            .build::<String>()
            .with_alphabetic_generator()
            .with_size(size)
            .create()
            .to_lowercase();

        let domain: String = f
            .build::<String>()
            .with_domain_generator()
            .create();

        Address::new(user, domain)
            .expect("generated address parts should be valid")
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        AddressBuilder::new(f)
    }
}

impl AutoFixture for Mailbox {
    type Builder<'b> = MailboxBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let has_name = f.rng().random_range(0..2) == 1;

        let name = if has_name {
            let size = f.rng().random_range(3..12);

            Some(
                f
                    .build::<String>()
                    .with_alphabetic_generator()
                    .with_size(size)
                    .create()
            )
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
        let count = f.rng().random_range(1..4);
        let mut mbs = Mailboxes::new();

        for _ in 0..count {
            mbs.push(Mailbox::create(f));
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

        Envelope::new(Some(from), vec![to])
            .expect("generated envelope should be valid")
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        EnvelopeBuilder::new(f)
    }
}

impl AutoFixture for Credentials {
    type Builder<'b> = CredentialsBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let user_size = f.rng().random_range(4..12);
        let pass_size = f.rng().random_range(8..20);

        let user = f
            .build::<String>()
            .with_alphabetic_generator()
            .with_size(user_size)
            .create()
            .to_lowercase();
        
        let password = f
            .build::<String>()
            .with_alphanumeric_generator()
            .with_size(pass_size)
            .create();

        Credentials::new(user, password)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        CredentialsBuilder::new(f)
    }
}

impl AutoFixture for Mechanism {
    type Builder<'b> = MechanismBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        match f.rng().random_range(0..3) {
            0 => Mechanism::Plain,
            1 => Mechanism::Login,
            _ => Mechanism::Xoauth2,
        }
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        MechanismBuilder::new(f)
    }
}
