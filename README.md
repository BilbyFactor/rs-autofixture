# rs-autofixture

![GitHub Release](https://img.shields.io/github/v/release/BilbyFactor/rs-autofixture)
[![Publish release](https://github.com/BilbyFactor/rs-autofixture/actions/workflows/publish.yml/badge.svg)](https://github.com/BilbyFactor/rs-autofixture/actions/workflows/publish.yml)
[![Validation](https://github.com/BilbyFactor/rs-autofixture/actions/workflows/pr-main.yml/badge.svg)](https://github.com/BilbyFactor/rs-autofixture/actions/workflows/pr-main.yml)

[![Crates.io License](https://img.shields.io/crates/l/rs-autofixture)](LICENSE)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/rs-autofixture)](https://crates.io/crates/rs-autofixture)

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

Smart pointers, interior mutability wrappers, and synchronisation primitives
around any `AutoFixture` type are supported out of the box too, no feature
flag required:

```rust
use std::cell::RefCell;
use std::num::NonZeroU32;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicU32;

use rs_autofixture::fixture::Fixture;

let mut fixture = Fixture::new();

let boxed: Box<u32> = fixture.create();
let shared: Arc<Vec<String>> = fixture.create();
let locked: Mutex<u32> = fixture.create();
let atomic: AtomicU32 = fixture.create();
let cell: RefCell<u32> = fixture.create();

// Guaranteed never zero:
let nonzero: NonZeroU32 = fixture.create();
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

// Fully populate a struct automatically, including nested types:
let customer: Customer = fixture.create();

// Populate a random enum variant:
let method: PaymentMethod = fixture.create();

// Populate with one or more constrained fields:
let customer = Customer {
    name: fixture
        .build::<String>()
        .with_alphabetic_generator()
        .with_size(12)
        .create(),
    ..fixture.create()
};

// Or use the derived builder directly, which gets a `with_<field>` setter
// for every field to fix it to a specific value (any unset fields are
// still randomly generated):
let customer = fixture
    .build::<Customer>()
    .with_name("Bob Katter".to_string())
    .with_age(30)
    .create();

// Fields with a well-defined "empty" value (`Option<T>`, `String`,
// and standard collections like `Vec<T>`) also get a `without_<field>`
// setter to force that empty value instead:
let customer = fixture
    .build::<Customer>()
    .without_loyalty_points() // -> None
    .without_tags()           // -> vec![]
    .create();
```

## Fixture Freezing

Freeze a value once, and every later `create()` of that type clones it
instead of creating a new random instance.

```rust
use rs_autofixture::AutoFixture;
use rs_autofixture::fixture::Fixture;

#[derive(AutoFixture, Clone)]
#[fixture(can_freeze)]
pub struct Author {
    name: String,
    age: u8,
}

#[derive(AutoFixture)]
pub struct Book {
    title: String,
    author: Author,
}

let mut fixture = Fixture::new();

let frozen_author: Author = fixture.freeze();

let book1: Book = fixture.create();
let book2: Book = fixture.create();

// Both books have the same Author instance:
assert_eq!(book1.author.name, frozen_author.name);
assert_eq!(book1.author.age, frozen_author.age);

assert_eq!(book2.author.name, frozen_author.name);
assert_eq!(book2.author.age, frozen_author.age);

// `Book.title` isn't frozen, so it still varies normally:
assert_ne!(book1.title, book2.title);
```

A few things to know:
* Primitives, `String`, `NonZero*`, `Uuid`, and most third (`chrono`/`lettre`/`rust_decimal`) types freeze automatically, no attribute needed.
* Your own structs, enums, and unions need `#[derive(AutoFixture, Clone)]` plus `#[fixture(can_freeze)]` to opt in.
* `Rc<T>` and `Arc<T>` freeze to the exact same instance everywhere they're used. Everything else freezes to a clone of the same value.
* `Mutex`, `RwLock`, `Atomic*`, `Cell`, `RefCell`, and `Box<T>` can't be frozen at this time due to the way the langague itself works. There seems to be some unstable approaches to some of these, but that is an investigation for the future... If you try to freeze them, the next create will ignore you and create a new one anyway. 😔

## Feature Flags

| Feature | Description |
|---------|-------------|
| `derive` | Enables `#[derive(AutoFixture)]` for structs, enums, and unions. |
| `chrono` | Adds `AutoFixture` implementations for [`chrono`](https://docs.rs/chrono) types: `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `DateTime<Utc>`, `DateTime<FixedOffset>`, `DateTime<Local>`, `FixedOffset`, `TimeDelta`, `Weekday`, `Month`, `Days`, and `Months`. |
| `uuid-extra` | Extends the `Uuid` builder with version selection (`with_v1`, `with_v3`, `with_v5`, `with_v6`, `with_v7`, `with_v8`). Without this feature the builder only produces v4 (random) UUIDs. |
| `lettre` | Adds `AutoFixture` implementations for [`lettre`](https://docs.rs/lettre) types: `Address`, `Mailbox`, `Mailboxes`, `Envelope`, `Credentials`, and `Mechanism`. |
| `rust-decimal` | Adds support for the [`rust_decimal`](https://docs.rs/rust_decimal/latest/rust_decimal) types: `Decimal` and `RoundingStrategy`. |
| `tokio` | Adds `AutoFixture` implementations for `tokio::sync::Mutex<T>` and `tokio::sync::RwLock<T>`. |
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
 * ~~Struct derive with/without builders~~
   * ~~`with_<field>` setters to fix a field to a specific value~~
   * ~~`without_<field>` setters for types with a well-defined "empty" value (`Option<T>`, `String`, standard collections)~~
 * ~~Github Actions release pipeline~~
 * AutoMoq implementation
 * Fixture freezing
 * SQL fixture building

## License
MIT License
