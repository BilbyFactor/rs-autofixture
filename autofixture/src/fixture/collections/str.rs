use std::collections::HashSet;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::Arc;

use rand::{
    RngExt,
    distr::{Alphabetic, Alphanumeric},
};
use uuid::Uuid;

use crate::fixture::{Fixture, FixtureExt, auto_fixture::AutoFixture, builder::FixtureBuilder};

enum StringGeneration {
    Alphbetic,
    Alphanumeric,
    Uuid4,
    Domain,
    Url,
}

const URL_TLDS: &[&str] = &["com", "org", "net", "io", "dev"];
const URL_SCHEMES: &[&str] = &["http", "https"];

pub struct StringBuilder<'b, S> {
    fixture: &'b mut Fixture,
    string_generation: StringGeneration,
    with: HashSet<char>,
    without: HashSet<char>,
    size: usize,
    _phantom: PhantomData<S>,
}

impl<'b, S> StringBuilder<'b, S> {
    /// Sets the desired string size for on create.
    /// Defaults is `16`. Will be ignored by the default UUID v4
    /// generator.
    pub fn with_size(&mut self, size: usize) -> &mut Self {
        self.size = size;

        self
    }

    /// Sets the String generator to use UUID v4 generation.
    ///
    /// Will ignore other `with()`, `without()` and `with_size()`
    /// builder calls if set.
    ///
    /// Default generator is UUID v4.
    pub fn with_uuid_v4_generator(&mut self) -> &mut Self {
        self.string_generation = StringGeneration::Uuid4;

        self
    }

    /// Sets the String generator to use `rand::dist::Alphabetic` generation.
    ///
    /// `with()` and `without()` builder calls are additive to the default set.
    ///
    /// Default generator is UUID v4.
    pub fn with_alphabetic_generator(&mut self) -> &mut Self {
        self.string_generation = StringGeneration::Alphbetic;

        self
    }

    /// Sets the String generator to use `rand::dist::Alphanumeric` generation.
    ///
    /// `with()` and `without()` builder calls are additive to the default set.
    ///
    /// Default generator is UUID v4.
    pub fn with_alphanumeric_generator(&mut self) -> &mut Self {
        self.string_generation = StringGeneration::Alphanumeric;

        self
    }

    /// Sets the String generator to produce a random domain name
    /// (e.g. `"xkqfmt.dev"`).
    ///
    /// Ignores `with_size()`.
    pub fn with_domain_generator(&mut self) -> &mut Self {
        self.string_generation = StringGeneration::Domain;

        self
    }

    /// Sets the String generator to produce a fully qualified URL
    /// (e.g. `"https://xkqfmt.dev/abcde"`).
    ///
    /// Ignores `with_size()`.
    pub fn with_url_generator(&mut self) -> &mut Self {
        self.string_generation = StringGeneration::Url;

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
        self.with
            .extend(chars);

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
        self.without
            .extend(chars);

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
            string_generation: StringGeneration::Uuid4,
            with: HashSet::new(),
            without: HashSet::new(),
            size: 16,
            _phantom: PhantomData,
        }
    }

    fn create(&mut self) -> Self::F {
        match self.string_generation {
            StringGeneration::Uuid4 => Uuid::new_v4()
                .to_string()
                .into(),
            StringGeneration::Alphanumeric => self
                .fixture
                .rng()
                .sample_iter(&Alphanumeric)
                .take(self.size)
                .map(char::from)
                .collect::<String>()
                .into(),
            StringGeneration::Alphbetic => self
                .fixture
                .rng()
                .sample_iter(&Alphabetic)
                .take(self.size)
                .map(char::from)
                .collect::<String>()
                .into(),
            StringGeneration::Domain => {
                let tld_idx = self
                    .fixture
                    .rng()
                    .random_range(0..URL_TLDS.len());

                let label_len = self
                    .fixture
                    .rng()
                    .random_range(4..10);

                let label: String = self
                    .fixture
                    .rng()
                    .sample_iter(&Alphabetic)
                    .take(label_len)
                    .map(|c| char::from(c).to_ascii_lowercase())
                    .collect();

                format!("{}.{}", label, URL_TLDS[tld_idx]).into()
            }
            StringGeneration::Url => {
                let scheme_idx = self
                    .fixture
                    .rng()
                    .random_range(0..URL_SCHEMES.len());

                let tld_idx = self
                    .fixture
                    .rng()
                    .random_range(0..URL_TLDS.len());

                let label_len = self
                    .fixture
                    .rng()
                    .random_range(4..10);

                let path_len = self
                    .fixture
                    .rng()
                    .random_range(3..8);

                let label: String = self
                    .fixture
                    .rng()
                    .sample_iter(&Alphabetic)
                    .take(label_len)
                    .map(|c| char::from(c).to_ascii_lowercase())
                    .collect();

                let path: String = self
                    .fixture
                    .rng()
                    .sample_iter(&Alphanumeric)
                    .take(path_len)
                    .map(char::from)
                    .collect();

                format!(
                    "{}://{}.{}/{}",
                    URL_SCHEMES[scheme_idx], label, URL_TLDS[tld_idx], path,
                )
                .into()
            }
        }
    }
}

impl crate::fixture::builder::EmptyFixture for String {
    fn empty() -> Self {
        String::new()
    }
}

impl AutoFixture for String {
    type Builder<'b> = StringBuilder<'b, String>;

    fn create(f: &mut Fixture) -> Self {
        if let Some(frozen) = f.frozen::<Self>() {
            return frozen;
        }

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
                    if let Some(frozen) = f.frozen::<Self>() {
                        return frozen;
                    }

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
