use std::sync::{Mutex, RwLock};

use std::sync::atomic::{
    AtomicBool, AtomicI8, AtomicI16, AtomicI32, AtomicI64, AtomicIsize, AtomicU8, AtomicU16,
    AtomicU32, AtomicU64, AtomicUsize, Ordering,
};

// `AtomicBool` is excluded from `test_atomic!` below: with only two possible
// values, a "values vary across 10 samples" check would be flaky (~0.2%
// chance of a false failure), so it only gets a basic creation check, same
// as `bool` in primitives.rs.
#[test]
fn atomic_bool_creates_successfully() {
    let mut f = Fixture::new();
    let a: AtomicBool = f.create();
    let _value = a.load(Ordering::Relaxed);
}

#[test]
fn atomic_bool_builder_creates_successfully() {
    let mut f = Fixture::new();
    let mut builder = f.build::<AtomicBool>();
    let _value = builder.create();
}

use rs_autofixture::fixture::Fixture;
use rs_autofixture::fixture::builder::FixtureBuilder;

#[test]
fn mutex_creates_successfully() {
    let mut f = Fixture::new();
    let m: Mutex<u32> = f.create();
    let _value = *m.lock().unwrap();
}

#[test]
fn mutex_builder_creates_successfully() {
    let mut f = Fixture::new();
    let mut builder = f.build::<Mutex<u32>>();
    let _value = builder.create();
}

#[test]
fn rwlock_creates_successfully() {
    let mut f = Fixture::new();
    let l: RwLock<u32> = f.create();
    let _value = *l.read().unwrap();
}

#[test]
fn rwlock_builder_creates_successfully() {
    let mut f = Fixture::new();
    let mut builder = f.build::<RwLock<u32>>();
    let _value = builder.create();
}

macro_rules! test_atomic {
    ($($atomic:ty, $inner:ty, $name:ident);* $(;)?) => {
        $(
            paste::paste! {
                #[test]
                fn [< $name _creates_successfully >]() {
                    let mut f = Fixture::new();
                    let a: $atomic = f.create();
                    let _value: $inner = a.load(Ordering::Relaxed);
                }

                #[test]
                fn [< $name _values_vary >]() {
                    let mut f = Fixture::new();

                    let values: Vec<$inner> = (0..10)
                        .map(|_| {
                            let a: $atomic = f.create();
                            a.load(Ordering::Relaxed)
                        })
                        .collect();

                    let all_same = values
                        .windows(2)
                        .all(|w| w[0] == w[1]);

                    assert!(!all_same, concat!("expected ", stringify!($atomic), " to vary"));
                }

                #[test]
                fn [< $name _builder_creates_successfully >]() {
                    let mut f = Fixture::new();
                    let mut builder = f.build::<$atomic>();
                    let _value = builder.create();
                }
            }
        )*
    };
}

test_atomic!(
    AtomicI8, i8, atomic_i8;
    AtomicI16, i16, atomic_i16;
    AtomicI32, i32, atomic_i32;
    AtomicI64, i64, atomic_i64;
    AtomicIsize, isize, atomic_isize;
    AtomicU8, u8, atomic_u8;
    AtomicU16, u16, atomic_u16;
    AtomicU32, u32, atomic_u32;
    AtomicU64, u64, atomic_u64;
    AtomicUsize, usize, atomic_usize;
);
