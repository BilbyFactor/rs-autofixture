//! # RS-AutoFixture
//!
//! A Rust equivalent of the popular .NET
//! [AutoFixture](https://github.com/AutoFixture/AutoFixture/) library for
//! quickly generating randomised test data fixtures.
//!
//! ## Quick Start
//!
//! ```rust
//! use rs_autofixture::fixture::Fixture;
//!
//! let mut fixture = Fixture::new();
//!
//! // Create a random primitive:
//! let value: u32 = fixture.create();
//!
//! // Create many at once:
//! let values: Vec<i64> = fixture.create_many(5).collect();
//! ```
//!
//! ## Builder Pattern
//!
//! Use builders to constrain generated values.
//! Builder calls chain naturally:
//!
//! ```rust
//! use rs_autofixture::fixture::Fixture;
//! use rs_autofixture::fixture::builder::FixtureBuilder;
//!
//! let mut fixture = Fixture::new();
//!
//! // Numeric range constraint:
//! let age: u8 = fixture
//!     .build::<u8>()
//!     .with_range(18..=65)
//!     .create();
//!
//! // String with specific generator and size:
//! let token: String = fixture
//!     .build::<String>()
//!     .with_alphanumeric_generator()
//!     .with_size(64)
//!     .create();
//!
//! // Option forced to Some:
//! let always_some: Option<u32> = fixture
//!     .build::<Option<u32>>()
//!     .with(42)
//!     .create();
//!
//! // Result forced to Err:
//! let always_err: Result<u32, String> = fixture
//!     .build::<Result<u32, String>>()
//!     .with_err(None)
//!     .create();
//!
//! // Collection with custom size:
//! let items: Vec<u32> = fixture
//!     .build::<Vec<u32>>()
//!     .with_size(10)
//!     .create();
//! ```
//!
//! ## Derive Macro
//!
//! With the `derive` feature enabled; you can automatically implement
//! [`AutoFixture`](fixture::auto_fixture::AutoFixture) for your own types:
//!
//! ```rust
//! use rs_autofixture::AutoFixture;
//! use rs_autofixture::fixture::Fixture;
//!
//! #[derive(AutoFixture)]
//! pub struct User {
//!     name: String,
//!     age: u8,
//!     active: bool,
//! }
//!
//! let mut fixture = Fixture::new();
//! let user: User = fixture.create();
//! ```
//!
//! Enums are also supported; a random variant is selected on each call
//! to `create()`; with any fields within that variant populated recursively:
//!
//! ```rust
//! use rs_autofixture::AutoFixture;
//! use rs_autofixture::fixture::Fixture;
//!
//! #[derive(AutoFixture)]
//! pub enum PaymentMethod {
//!     CreditCard { number: String, expiry: String },
//!     BankTransfer(String),
//!     Cash,
//! }
//!
//! let mut fixture = Fixture::new();
//! let method: PaymentMethod = fixture.create();
//! ```
//!
//! Unions are supported too; a random field is selected and populated:
//!
//! ```rust
//! use rs_autofixture::AutoFixture;
//! use rs_autofixture::fixture::Fixture;
//!
//! #[derive(AutoFixture, Clone, Copy)]
//! pub union RawValue {
//!     integer: i64,
//!     floating: f64,
//! }
//!
//! let mut fixture = Fixture::new();
//! let raw: RawValue = fixture.create();
//! ```
//!
//! ## Nested Types and Builder Composition
//!
//! Derived types compose naturally; nested structs, enums, and
//! collections are all populated recursively:
//!
//! ```rust
//! use rs_autofixture::AutoFixture;
//! use rs_autofixture::fixture::Fixture;
//! use rs_autofixture::fixture::builder::FixtureBuilder;
//!
//! #[derive(AutoFixture)]
//! pub struct Address {
//!     street: String,
//!     city: String,
//!     postcode: String,
//! }
//!
//! #[derive(AutoFixture)]
//! pub struct Customer {
//!     name: String,
//!     age: u8,
//!     address: Address,
//!     loyalty_points: Option<u32>,
//!     tags: Vec<String>,
//! }
//!
//! let mut fixture = Fixture::new();
//!
//! // All fields populated automatically, including nested Address,
//! // the Option<u32> (randomly Some or None), and the Vec<String>:
//! let customer: Customer = fixture.create();
//! ```
//!
//! For finer control over individual fields, you can use the builder pattern on
//! the field types directly and compose the struct yourself:
//!
//! ```rust
//! # use rs_autofixture::AutoFixture;
//! # use rs_autofixture::fixture::Fixture;
//! # use rs_autofixture::fixture::builder::FixtureBuilder;
//! #
//! # #[derive(AutoFixture)]
//! # pub struct Address {
//! #     street: String,
//! #     city: String,
//! #     postcode: String,
//! # }
//! #
//! # #[derive(AutoFixture)]
//! # pub struct Customer {
//! #     name: String,
//! #     age: u8,
//! #     address: Address,
//! #     loyalty_points: Option<u32>,
//! #     tags: Vec<String>,
//! # }
//! #
//! let mut fixture = Fixture::new();
//!
//! // Build a customer with constrained fields:
//! let customer = Customer {
//!     name: fixture
//!         .build::<String>()
//!         .with_alphabetic_generator()
//!         .with_size(12)
//!         .create(),
//!     age: fixture
//!         .build::<u8>()
//!         .with_range(18..=99)
//!         .create(),
//!     address: fixture.create(),
//!     loyalty_points: fixture
//!         .build::<Option<u32>>()
//!         .with(500)
//!         .create(),
//!     tags: fixture
//!         .build::<Vec<String>>()
//!         .with_size(2)
//!         .create(),
//! };
//! ```
//!
//! ## Feature Flags
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `derive` | Enables `#[derive(AutoFixture)]` for structs, enums, and unions. |
//! | `chrono` | Adds `AutoFixture` implementations for [`chrono`](https://docs.rs/chrono) types: `NaiveDate`, `NaiveTime`, `NaiveDateTime`, `DateTime<Utc>`, `DateTime<FixedOffset>`, `DateTime<Local>`, `FixedOffset`, `TimeDelta`, `Weekday`, `Month`, `Days`, and `Months`. |
//! | `uuid-extra` | Extends the `Uuid` builder with version selection (`with_v1`, `with_v3`, `with_v5`, `with_v6`, `with_v7`, `with_v8`). Without this feature the builder only produces v4 (random) UUIDs. |
//! | `nightly-float` | Enables `f16` and `f128` support (requires nightly Rust). |
//! | `double-tuples` | Adds `AutoFixture` for tuples up to 16 elements. |
//! | `tripple-tuples` | Adds `AutoFixture` for tuples up to 32 elements. |
//! | `way-too-many-tuples` | Adds `AutoFixture` for tuples up to 128 elements. |
//!
//! [LICENSE-MIT]: https://opensource.org/licenses/MIT

#![cfg_attr(feature = "nightly-float", feature(f16, f128))]

pub mod fixture;

pub use rand;

// Re-export #[derive(AutoFixture)].
#[cfg(feature = "derive")]
extern crate rs_autofixture_derive;

/// Derive macro available if rs-autofixture is built with `features = ["derive"]`.
#[cfg(feature = "derive")]
#[cfg_attr(docsrs, doc(cfg(feature = "derive")))]
pub use rs_autofixture_derive::AutoFixture;
