use std::collections::HashSet;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use rand::{RngExt, distr::Alphanumeric};

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

pub struct StringBuilder<'b, S> {
    fixture: &'b mut Fixture,
    with: HashSet<char>,
    without: HashSet<char>,
    size: usize,
    _phantom: PhantomData<S>,
}

impl<'b, S> StringBuilder<'b, S> {
    /// Sets the desired string size for on create...
    pub fn with_size(&mut self, size: usize) -> &mut Self {
        self.size = size;

        self
    }

    /// Adds characters that are allowed to appear in generated strings.
    ///
    /// This call is additive across multiple uses.
    /// If a call to without is also made, it will exclude any items added here.
    pub fn with<I>(&mut self, chars: I) -> &mut Self
    where
        I: IntoIterator<Item = char>,
    {
        self.with.extend(chars);

        unimplemented!("Not yet implemented :(")
    }

    /// Adds characters that are disallowed in generated strings.
    ///
    /// This call is additive across multiple uses.
    /// Characters listed here are removed after `with(...)` is applied.
    pub fn without<I>(&mut self, chars: I) -> &mut Self
    where
        I: IntoIterator<Item = char>,
    {
        self.without.extend(chars);

        unimplemented!("Not yet implemented :(")
    }
}

impl<'b, S> FixtureBuilder<'b> for StringBuilder<'b, S>
where
    S: AutoFixture + From<String>,
{
    type F = S;

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            with: HashSet::new(),
            without: HashSet::new(),
            size: 32,
            _phantom: PhantomData,
        }
    }

    fn create(&mut self) -> Self::F {
        self.fixture
            .rng()
            .sample_iter(&Alphanumeric)
            .take(self.size)
            .map(char::from)
            .collect::<String>()
            .into()
    }
}

impl AutoFixture for String {
    type Builder<'b> = StringBuilder<'b, String>;

    fn create(f: &mut Fixture) -> Self {
        StringBuilder::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> StringBuilder<'b, String> {
        StringBuilder::new(f)
    }
}

macro_rules! impl_autofixture_string_like {
    ($($ty:ty), *) => {
        $(
            impl AutoFixture for $ty {
                type Builder<'b> = StringBuilder<'b, $ty>;

                fn create(f: &mut Fixture) -> Self {
                    StringBuilder::new(f).create()
                }

                fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
                    StringBuilder::new(f)
                }
            }
        )*
    };
}

impl_autofixture_string_like!(Box<str>, Arc<str>, Rc<str>);
