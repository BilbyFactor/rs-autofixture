//! AutoFixture implementations for standard library collection types.
//!
//! All collection types default to a size of `3` on `create()`.
//! Use the builder's `with_size()` to override:
//!
//! ```rust
//! use rs_autofixture::fixture::Fixture;
//! use rs_autofixture::fixture::builder::FixtureBuilder;
//!
//! let mut fixture = Fixture::new();
//!
//! let items: Vec<u32> = fixture
//!     .build::<Vec<u32>>()
//!     .with_size(10)
//!     .create();
//!
//! assert_eq!(items.len(), 10);
//! ```
//!
//! Supported types: `Vec`, `VecDeque`, `LinkedList`, `HashSet`, `BTreeSet`,
//! `BinaryHeap`, `HashMap`, `BTreeMap`, `String`, `Box<str>`, `Arc<str>`,
//! `Rc<str>`, and tuples up to arity 8 (extendable via feature flags).

pub mod iter;
pub mod str;
pub mod tuple;
