use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::{
        FixtureBuilder,
        conditions::BuilderCondition,
    },
};

pub struct ResultBuilder<'b, T, E>
where
    T: AutoFixture,
    E: AutoFixture,
{
    fixture: &'b mut Fixture,
    temp_t: Option<T>,
    temp_e: Option<E>,
}

impl<'b, T, E> ResultBuilder<'b, T, E>
where
    T: AutoFixture,
    E: AutoFixture,
{
    pub fn with_ok(&mut self) -> &mut Self {
        self
    }

    pub fn with_err(&mut self) -> &mut Self {
        self
    }
}

impl<'b, T, E> FixtureBuilder<'b> for ResultBuilder<'b, T, E>
where
    T: AutoFixture + Clone,
    E: AutoFixture + Clone,
{
    type F = Result<T, E>;

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            temp_t: None,
            temp_e: None,
        }
    }

    fn create(&mut self) -> Self::F {
        //self.condition.apply(self.fixture)
        Ok(T::create(self.fixture))
    }
}

impl<T, E> AutoFixture for Result<T, E>
where
    T: AutoFixture + Clone,
    E: AutoFixture + Clone,
{
    /// rs-autofixture will randomly pick `Ok(T::create())` or `Err(E::create())`.
    /// 
    /// The logic is that if you are choosing to use a fixture for an Option
    /// type in unit testing, both outcomes; `Ok` or `Err` are valid.
    /// 
    /// Use the builder `with_ok()` and `with_err()` methods if you instead always
    /// need `Ok` or `Err` respectively. 
    fn create(f: &mut crate::fixture::Fixture) -> Self {
        let ok: bool = f.rng().random();

        if ok {
            Ok(T::create(f))
        }
        else {
            Err(E::create(f))
        }
    }

    fn build<'b>(f: &'b mut Fixture) -> impl FixtureBuilder<'b> {
        ResultBuilder::<T, E>::new(f)
    }
}
