use rand::{RngExt, distr::Alphanumeric};
use std::collections::HashSet;

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
};

pub struct StringBuilder<'b> {
    fixture: &'b mut Fixture,
    with: HashSet<char>,
    without: HashSet<char>,
    size: usize,
}

impl<'b> StringBuilder<'b> {
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

impl<'b> FixtureBuilder<'b> for StringBuilder<'b> {
    type F = String;

    fn new(f: &'b mut Fixture) -> Self {
        Self {
            fixture: f,
            with: HashSet::new(),
            without: HashSet::new(),
            size: 32,
        }
    }

    fn create(&mut self) -> Self::F {
        self.fixture
            .rng()
            .sample_iter(&Alphanumeric)
            .take(self.size)
            .map(char::from)
            .collect()
    }
}

impl AutoFixture for String {
    type Builder<'b> = StringBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        StringBuilder::new(f).create()
    }

    fn build<'b>(f: &'b mut Fixture) -> StringBuilder<'b> {
        StringBuilder::new(f)
    }
}
