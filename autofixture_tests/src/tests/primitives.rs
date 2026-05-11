use rs_autofixture::fixture::Fixture;

#[test]
fn bool_creates_successfully() {
    let mut f = Fixture::new();
    let _: bool = f.create();
}

#[test]
fn char_creates_successfully() {
    let mut f = Fixture::new();
    let _: char = f.create();
}

#[test]
fn unit_creates_successfully() {
    let mut f = Fixture::new();
    let _: () = f.create();
}

macro_rules! test_signed {
    ($($ty:ty),*) => {
        $(
            paste::paste! {
                #[test]
                fn [< $ty _creates_successfully >]() {
                    let mut f = Fixture::new();
                    let _: $ty = f.create();
                }

                #[test]
                fn [< $ty _values_vary >]() {
                    let mut f = Fixture::new();
                    let values: Vec<$ty> = (0..10).map(|_| f.create()).collect();
                    let all_same = values.windows(2).all(|w| w[0] == w[1]);
                    assert!(!all_same, concat!("expected ", stringify!($ty), " to vary"));
                }
            }
        )*
    };
}

test_signed!(i8, i16, i32, i64, i128, isize);

macro_rules! test_unsigned {
    ($($ty:ty),*) => {
        $(
            paste::paste! {
                #[test]
                fn [< $ty _creates_successfully >]() {
                    let mut f = Fixture::new();
                    let _: $ty = f.create();
                }

                #[test]
                fn [< $ty _values_vary >]() {
                    let mut f = Fixture::new();
                    let values: Vec<$ty> = (0..10).map(|_| f.create()).collect();
                    let all_same = values.windows(2).all(|w| w[0] == w[1]);
                    assert!(!all_same, concat!("expected ", stringify!($ty), " to vary"));
                }
            }
        )*
    };
}

test_unsigned!(u8, u16, u32, u64, u128, usize);

macro_rules! test_float {
    ($($ty:ty),*) => {
        $(
            paste::paste! {
                #[test]
                fn [< $ty _creates_successfully >]() {
                    let mut f = Fixture::new();
                    let _: $ty = f.create();
                }

                #[test]
                fn [< $ty _values_vary >]() {
                    let mut f = Fixture::new();
                    let values: Vec<$ty> = (0..10).map(|_| f.create()).collect();
                    let all_same = values.windows(2).all(|w| w[0] == w[1]);
                    assert!(!all_same, concat!("expected ", stringify!($ty), " to vary"));
                }
            }
        )*
    };
}

test_float!(f32, f64);
