use std::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
};

use crate::fixture::{
    Fixture,
    auto_fixture::AutoFixture,
    builder::FixtureBuilder,
    wrapper::{FromInner, WrapperBuilder},
};

macro_rules! impl_autofixture_nonzero {
    ($($w:ident => $t:ty), *) => {
        $(
            impl FromInner<$t> for $w {
                // Setting the low bit guarantees a nonzero value without
                // ever needing to retry, at the cost of a slight bias
                // towards odd numbers.
                fn from_inner(value: $t) -> Self {
                    $w::new(value | 1).expect("value | 1 is never zero")
                }
            }

            impl AutoFixture for $w {
                type Builder<'b> = WrapperBuilder<'b, $w, $t>;

                fn create(f: &mut Fixture) -> Self {
                    if let Some(frozen) = f.frozen::<Self>() {
                        return frozen;
                    }

                    WrapperBuilder::<$w, $t>::new(f).create()
                }

                fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
                    WrapperBuilder::new(f)
                }
            }
        )*
    };
}

impl_autofixture_nonzero!(
    NonZeroU8 => u8,
    NonZeroU16 => u16,
    NonZeroU32 => u32,
    NonZeroU64 => u64,
    NonZeroU128 => u128,
    NonZeroUsize => usize,
    NonZeroI8 => i8,
    NonZeroI16 => i16,
    NonZeroI32 => i32,
    NonZeroI64 => i64,
    NonZeroI128 => i128,
    NonZeroIsize => isize
);
