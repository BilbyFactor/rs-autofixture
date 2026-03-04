use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

pub struct OptionBuilder<'b, T>
where
    T: AutoFixture + Clone,
{
    fixture: &'b mut Fixture,
    condition: Option<Option<T>>,
}

impl<'b, T> OptionBuilder<'b, T>
where
    T: AutoFixture + Clone,
{
    /// Forces `create()` to produce `Some(some)`.
    /// 
    /// # Example
    /// ```
    /// use rs_autofixture::fixture::{Fixture, builder::FixtureBuilder};
    /// 
    /// let mut fixture = Fixture::new();
    /// let mut result_fixture = fixture.build::<Option<bool>>();
    /// 
    /// assert_eq!(result_fixture.with(true).create(), Some(true));
    /// assert_eq!(result_fixture.with(false).create(), Some(false));
    /// ```
    /// 
    /// # Arguments
    /// * `some` - The value to force the result of `create()` to be.
    pub fn with(&mut self, some: T) -> &mut Self {
        self.condition = Some(Some(some));

        self
    }

    /// Forces `create()` to produce None.
    /// 
    /// # Example
    /// ```
    /// use rs_autofixture::fixture::{Fixture, builder::FixtureBuilder};
    /// 
    /// let mut fixture = Fixture::new();
    /// let mut result_fixture = fixture.build::<Option<()>>();
    /// 
    /// assert_eq!(result_fixture.without().create(), None);
    /// ```
    pub fn without(&mut self) -> &mut Self {
        self.condition = Some(None);

        self
    }
}

impl<'b, T> FixtureBuilder<'b> for OptionBuilder<'b, T>
where
    T: AutoFixture + Clone,
{
    type F = Option<T>;

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            condition: None,
        }
    }

    fn create(&mut self) -> Self::F {
        match &self.condition {
            Some(c) => c.clone(),
            None => Self::F::create(self.fixture),
        }
    }
}

impl<T> AutoFixture for Option<T>
where
    T: AutoFixture + Clone,
{
    type Builder<'b> = OptionBuilder<'b, T>;

    /// Unlike the C# version of AutoFixture, rs-autofixture will
    /// randomly pick `None` or `Some(T::create())`.
    /// 
    /// The logic is that if you are choosing to use a fixture for an Option
    /// type in unit testing, both outcomes; `Some` or `None` are valid.
    /// 
    /// Use the builder `with()` and `without()` methods if you instead always
    /// need `Some` or `None` respectively. 
    fn create(f: &mut crate::fixture::Fixture) -> Self {
        let some: bool = f.rng().random();

        if some {
            Some(T::create(f))
        }
        else {
            None
        }
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        OptionBuilder::<'b, T>::new(f)
    }
}
