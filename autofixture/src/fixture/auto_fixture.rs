use crate::fixture::{Fixture, builder::FixtureBuilder};

pub trait AutoFixture {
    type Builder<'b>: FixtureBuilder<'b, F = Self>;

    /// Creates a new randomly populated implementation of Self.
    /// 
    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn create(f: &mut Fixture) -> Self;

    /// Creates the FixtureBuilder implementation for Self.
    /// 
    /// # Arguments
    /// * `f` - the base `Fixture` struct.
    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b>;
}

macro_rules! impl_autofixture_random{
    ($($prim:ty => $b:ident), *) => {
        $(
            impl crate::fixture::auto_fixture::AutoFixture for $prim {
                type Builder<'b> = $b<'b>;

                #[inline]
                fn create(f: &mut crate::fixture::Fixture) -> Self {
                    use rand::RngExt;
                    
                    use crate::fixture::FixtureExt;

                    f.rng().random()
                }

                #[inline]
                fn build<'b>(f: &'b mut crate::fixture::Fixture)
                    -> Self::Builder<'b>
                {
                    use crate::fixture::builder::FixtureBuilder;

                    <$b>::new(f)
                }
            }
        )*
    };
}

macro_rules! impl_autofixture_random_dyn {
    ($($prim:ty => $b:ident), *) => {
        $(
            impl crate::fixture::auto_fixture::AutoFixture for $prim {
                type Builder<'b> = $b<'b>;

                #[inline]
                fn create(f: &mut crate::fixture::Fixture) -> Self {
                    use rand::RngExt;

                    use crate::fixture::FixtureExt;

                    <$prim>::from_ne_bytes(f.rng().random())
                }

                #[inline]
                fn build<'b>(f: &'b mut crate::fixture::Fixture)
                    -> Self::Builder<'b>
                {
                    use crate::fixture::builder::FixtureBuilder;
                    
                    <$b>::new(f)
                }
            }
        )*
    };
}

pub(crate) use impl_autofixture_random;
pub(crate) use impl_autofixture_random_dyn;
