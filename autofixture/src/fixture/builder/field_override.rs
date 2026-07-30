/// Tracks whether a derived struct builder's `with_<field>` or
/// `without_<field>` setter has been used to override a field, falling back
/// to `AutoFixture::create` when neither has been called.
#[derive(Default)]
pub enum FieldOverride<T> {
    #[default]
    NotSet,
    SetWith(T),
    SetWithout,
}
