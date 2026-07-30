use crate::fixture::auto_fixture::AutoFixture;

/// Types with a well-defined "empty" value, used by generated
/// `without_<field>` builder setters (e.g. `None` for `Option<T>`, or an
/// empty `Vec`/`String`/etc).
pub trait EmptyFixture: AutoFixture {
    fn empty() -> Self;
}
