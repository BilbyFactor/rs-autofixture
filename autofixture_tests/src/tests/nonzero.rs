use std::num::{
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize, NonZeroU8,
    NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
};

use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

macro_rules! test_nonzero {
    ($($nz:ty, $name:ident);* $(;)?) => {
        $(
            paste::paste! {
                #[test]
                fn [< $name _creates_successfully >]() {
                    let mut f = Fixture::new();
                    let value: $nz = f.create();
                    assert_ne!(value.get(), 0);
                }

                #[test]
                fn [< $name _values_vary >]() {
                    let mut f = Fixture::new();

                    let values: Vec<_> = (0..10)
                        .map(|_| {
                            let value: $nz = f.create();
                            value.get()
                        })
                        .collect();

                    let all_same = values
                        .windows(2)
                        .all(|w| w[0] == w[1]);

                    assert!(!all_same, concat!("expected ", stringify!($nz), " to vary"));
                }

                #[test]
                fn [< $name _builder_creates_successfully >]() {
                    let mut f = Fixture::new();
                    let mut builder = f.build::<$nz>();
                    let _value = builder.create();
                }
            }
        )*
    };
}

test_nonzero!(
    NonZeroI8, nonzero_i8;
    NonZeroI16, nonzero_i16;
    NonZeroI32, nonzero_i32;
    NonZeroI64, nonzero_i64;
    NonZeroI128, nonzero_i128;
    NonZeroIsize, nonzero_isize;
    NonZeroU8, nonzero_u8;
    NonZeroU16, nonzero_u16;
    NonZeroU32, nonzero_u32;
    NonZeroU64, nonzero_u64;
    NonZeroU128, nonzero_u128;
    NonZeroUsize, nonzero_usize;
);

#[test]
fn freeze_nonzero_repeats_on_subsequent_creates() {
    let mut f = Fixture::new();

    let frozen: NonZeroU32 = f.freeze();
    let created: NonZeroU32 = f.create();

    assert_eq!(frozen, created);
}
