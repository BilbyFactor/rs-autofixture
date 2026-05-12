# rs-autofixture
Rust equivalent of the popular .NET [AutoFixture](https://github.com/AutoFixture/AutoFixture/) library for quickly generating randomised test data fixtures.

## Quick Start

```rust
use rs_autofixture::fixture::Fixture;

let mut fixture = Fixture::new();

// Create a random primitive:
let value: u32 = fixture.create();

// Create many at once:
let values: Vec<i64> = fixture
    .create_many(5)
    .collect();
```

## Builder Pattern

Use builders to constrain generated values:

```rust
use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

let mut fixture = Fixture::new();

// Numeric range constraint:
let age: u8 = fixture
    .build::<u8>()
    .with_range(18..=65)
    .create();

// String with specific generator and size:
let token: String = fixture
    .build::<String>()
    .with_alphanumeric_generator()
    .with_size(64)
    .create();

// Option forced to Some:
let always_some: Option<u32> = fixture
    .build::<Option<u32>>()
    .with(42)
    .create();

// Collection with custom size:
let items: Vec<u32> = fixture
    .build::<Vec<u32>>()
    .with_size(10)
    .create();

// Create many from a builder:
let tokens: Vec<String> = fixture
    .build::<String>()
    .with_alphanumeric_generator()
    .with_size(16)
    .create_many(5)
    .collect();
```

## Derive Macro

With the `derive` feature enabled, you can automatically implement `AutoFixture`
for your own structs, enums, and unions:

```rust
use rs_autofixture::AutoFixture;
use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

#[derive(AutoFixture)]
pub struct Address {
    street: String,
    city: String,
    postcode: String,
}

#[derive(AutoFixture)]
pub struct Customer {
    name: String,
    age: u8,
    address: Address,
    loyalty_points: Option<u32>,
    tags: Vec<String>,
}

#[derive(AutoFixture)]
pub enum PaymentMethod {
    CreditCard { number: String, expiry: String },
    BankTransfer(String),
    Cash,
}

let mut fixture = Fixture::new();

// Fully populated automatically, including nested types:
let customer: Customer = fixture.create();
let method: PaymentMethod = fixture.create();

// Or build with constrained fields:
let customer = Customer {
    name: fixture
        .build::<String>()
        .with_alphabetic_generator()
        .with_size(12)
        .create(),
    age: fixture
        .build::<u8>()
        .with_range(18..=99)
        .create(),
    address: fixture.create(),
    loyalty_points: fixture
        .build::<Option<u32>>()
        .with(500)
        .create(),
    tags: fixture
        .build::<Vec<String>>()
        .with_size(2)
        .create(),
};
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `derive` | Enables `#[derive(AutoFixture)]` for structs, enums, and unions. |
| `chrono` | Adds `AutoFixture` implementations for [`chrono`](https://docs.rs/chrono) types: `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `DateTime<Utc>`, `DateTime<FixedOffset>`, `DateTime<Local>`, `FixedOffset`, `TimeDelta`, `Weekday`, `Month`, `Days`, and `Months`. |
| `uuid-extra` | Extends the `Uuid` builder with version selection (`with_v1`, `with_v3`, `with_v5`, `with_v6`, `with_v7`, `with_v8`). Without this feature the builder only produces v4 (random) UUIDs. |
| `lettre` | Adds `AutoFixture` implementations for [`lettre`](https://docs.rs/lettre) types: `Address`, `Mailbox`, `Mailboxes`, `Envelope`, `Credentials`, and `Mechanism`. |
| `nightly-float` | Enables `f16` and `f128` support (requires nightly Rust). |
| `double-tuples` | Adds `AutoFixture` for tuples up to 16 elements. |
| `tripple-tuples` | Adds `AutoFixture` for tuples up to 32 elements. |
| `way-too-many-tuples` | Adds `AutoFixture` for tuples up to 128 elements. |

## Planned TODO:
 * ~~Finish main implementation of primitives~~
 * ~~Create derive macro for AutoFixture~~
 * ~~Unit testing~~
 * ~~Docstrings~~
 * ~~Cargo crate submission~~
 * Github Actions release pipeline
 * AutoMoq implementation
 * Fixture freezing

## License
MIT License
