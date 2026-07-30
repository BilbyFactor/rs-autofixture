/// Tracks whether a derived struct builder's `with_<field>` or
/// `without_<field>` setter has been used to override a field, falling back
/// to `AutoFixture::create` when neither has been called.
pub enum FieldOverride<T> {
    NotSet,
    SetWith(T),
    SetWithout,
}

impl<T> Default for FieldOverride<T> {
    fn default() -> Self {
        FieldOverride::NotSet
    }
}
