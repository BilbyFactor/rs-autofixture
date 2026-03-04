use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

pub struct ResultBuilder<'b, T, E>
where
    T: AutoFixture + Clone,
    E: AutoFixture + Clone,
{
    fixture: &'b mut Fixture,
    condition: Option<Result<Option<T>, Option<E>>>,
}

impl<'b, T, E> ResultBuilder<'b, T, E>
where
    T: AutoFixture + Clone,
    E: AutoFixture + Clone,
{
    /// Forces `create()` to produce `Ok(ok)`.
    /// where `ok` can be `Some(ok_actual)`, or `None`
    /// (in this case, `None` will be the default `T::create(fixture)` behavior).
    /// 
    /// # Example
    /// ```
    /// use rs_autofixture::fixture::{Fixture, builder::FixtureBuilder};
    /// 
    /// let mut fixture = Fixture::new();
    /// let mut result_fixture = fixture.build::<Result<bool, bool>>();
    /// 
    /// assert!(result_fixture.with_ok(None).create().is_ok());
    /// assert_eq!(result_fixture.with_ok(Some(true)).create(), Ok(true));
    /// ```
    /// 
    /// # Arguments
    /// * `ok` - The value to force the `create()` method to consider.
    pub fn with_ok(&mut self, ok: Option<T>) -> &mut Self {
        self.condition = Some(Ok(ok));

        self
    }

    /// Forces `create()` to produce `Err(err)`.
    /// where `err` can be `Some(err_actual)`, or `None`
    /// (in this case, `None` will be the default `E::create(fixture)` behavior).
    /// 
    /// # Example
    /// ```
    /// use rs_autofixture::fixture::{Fixture, builder::FixtureBuilder};
    /// 
    /// let mut fixture = Fixture::new();
    /// let mut result_fixture = fixture.build::<Result<bool, bool>>();
    /// 
    /// assert!(result_fixture.with_err(None).create().is_err());
    /// assert_eq!(result_fixture.with_err(Some(false)).create(), Err(false));
    /// ```
    /// 
    /// # Arguments
    /// * `err` - The value to force the `create()` method to consider.
    pub fn with_err(&mut self, err: Option<E>) -> &mut Self {
        self.condition = Some(Err(err));

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
            condition: None,
        }
    }

    fn create(&mut self) -> Self::F {
        match &self.condition {
            Some(r) => match r {
                Ok(ok) => Ok(match ok {
                    Some(c) => c.clone(),
                    None => T::create(self.fixture),
                }),
                Err(err) => Err(match err {
                    Some(c) => c.clone(),
                    None => E::create(self.fixture),
                }),
            },
            None => Self::F::create(self.fixture),
        }
    }
}

impl<T, E> AutoFixture for Result<T, E>
where
    T: AutoFixture + Clone,
    E: AutoFixture + Clone,
{
    type Builder<'b> = ResultBuilder<'b, T, E>;

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

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        ResultBuilder::<T, E>::new(f)
    }
}
