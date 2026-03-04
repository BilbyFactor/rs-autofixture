use rand::RngExt;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::{
        FixtureBuilder,
        conditions::BuilderCondition,
        conditions::option::OptionCondition,
    },
};

pub struct OptionBuilder<'b, T>
where
    T: AutoFixture,
{
    fixture: &'b mut Fixture,
    condition: OptionCondition<T>,
}

impl<'b, T> OptionBuilder<'b, T>
where
    T: AutoFixture,
{
    pub fn with(&mut self) -> &mut Self {
        self
    }

    /// Force `create` to produce None.
    /// 
    /// # Example
    /// ```
    /// use rs_autofixture::fixture::Fixture;
    /// 
    /// let mut fixture = Fixture::new();
    /// 
    /// let result = fixture.build::<Option<()>>()
    ///     .without()
    ///     .create();
    /// ```
    pub fn without(&mut self) -> &mut Self {
        self.condition.without();

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
            condition: OptionCondition::<T>::default(),
        }
    }

    fn create(&mut self) -> Self::F {
        self.condition.apply(self.fixture)
    }
}

impl<T> AutoFixture for Option<T>
where
    T: AutoFixture + Clone,
{
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

    fn build<'b>(f: &'b mut Fixture) -> impl FixtureBuilder<'b> {
        OptionBuilder::<'b, T>::new(f)
    }
}
