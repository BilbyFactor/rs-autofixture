pub mod auto_fixture;
pub mod builder;
pub mod collections;
pub mod primitives;

use std::any::{Any, TypeId};
use std::collections::HashMap;

use rand::rngs::ThreadRng;

use crate::fixture::{
    auto_fixture::AutoFixture,
};

/// Methods intended for extending the internal functionality of `Fixture`.
pub trait FixtureExt {
    /// Return the `Fixture` implementations `ThreadRng` engine.
    fn rng(&mut self) -> &mut ThreadRng;

    /// Register a `&str` internally that will live for the lifetime of
    /// this instance.
    /// Will return the `&str` instance registered.
    /// 
    /// # Arguments
    /// * `to_register`: The `String` to register as a `&str`.
    fn register_str_ref(&mut self, to_register: String) -> &str;

    /// Register a `T` slice internally that will live for the lifetime of
    /// this instance.
    /// Will return the `&[T]` instance registered.
    /// 
    /// # Arguments
    /// * `to_register`: An iterator of type `T` to register as a `&[T]`.
    fn register_slice_ref<T: 'static, I>(&mut self, to_register: I) -> &[T]
    where
        I: IntoIterator<Item = T>;
}

/// The main `Fixture` structure that maintains any state and the RNG engine.
pub struct Fixture {
    rng: ThreadRng,
    ref_pool: HashMap<TypeId, Box<dyn Any>>,
}

impl Fixture {
    /// Creates a new `Fixture` with a default RNG engine.
    pub fn new() -> Self {
        Self {
            rng: ThreadRng::default(),
            ref_pool: HashMap::new(),
        }
    }

    /// Creates a new `FixtureBuilder` for type `F`, where `F` is a
    /// type that implements `AutoFixture`. This includes all Rust primitive
    /// types, and can be derived on a struct or enum with `#[derive(AutoFixture)]`.
    pub fn build<'b, F: AutoFixture>(&'b mut self) -> F::Builder<'b> {
        F::build(self)
    }

    /// Creates a new randomly populated implementation of the given type `F`
    /// where `F` implements `AutoFixture`. This includes all Rust primitive
    /// types, and can be derived on a struct or enum with `#[derive(AutoFixture)]`
    pub fn create<F: AutoFixture>(&mut self) -> F {
        F::create(self)
    }

    /// Creates a new iterator with `n` items pre-populated of the given type `F`
    /// where `F` implements `AutoFixture`. This includes all Rust primitive
    /// types, and can be derived on a struct or enum with `#[derive(AutoFixture)]`
    /// 
    /// # Arguments
    /// * `n`: The number of items to pre-populate.
    pub fn create_many<F: AutoFixture>(&mut self, n: usize)
        -> impl Iterator<Item = F>
    {
        (0..n).map(|_| F::create(self))
    }
}

impl FixtureExt for Fixture {
    fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }
    
    fn register_str_ref(&mut self, to_register: String) -> &str {
        let str_pool = self.ref_pool
            .entry(TypeId::of::<Vec<Box<str>>>())
            .or_insert_with(|| Box::new(Vec::<Box<str>>::new()))
            .downcast_mut::<Vec<Box<str>>>()
            .expect("expecting `TypeId` to match or be inserted as `Vec<Box<str>>`...");

        str_pool.push(to_register.into_boxed_str());

        str_pool
            .last()
            .expect("expecting one &str to have just been registered...")
    }
    
    fn register_slice_ref<T: 'static, I>(&mut self, to_register: I) -> &[T]
    where
        I: IntoIterator<Item = T>,
    {
        let slice_pool = self.ref_pool
            .entry(TypeId::of::<Vec<Box<[T]>>>())
            .or_insert_with(|| Box::new(Vec::<Box<[T]>>::new()))
            .downcast_mut::<Vec<Box<[T]>>>()
            .expect("expecting `TypeId` to match or be inserted as `Vec<Box<T>>`...");

        slice_pool.push(to_register
            .into_iter()
            .collect::<Vec<T>>()
            .into_boxed_slice());

        slice_pool
            .last()
            .expect("expecting one `&[T]` to have just been registered...")
    }
}

impl Default for Fixture {
    fn default() -> Self {
        Self::new()
    }
}
