use chrono::{
    DateTime, Days, FixedOffset, Local, Month, Months, NaiveDate, NaiveDateTime, NaiveTime,
    TimeDelta, Utc, Weekday,
};

use rs_autofixture::fixture::Fixture;

macro_rules! test_chrono_creates {
    ($($name:ident: $ty:ty),*) => {
        $(
            #[test]
            fn $name() {
                let mut f = Fixture::new();
                let _: $ty = f.create();
            }
        )*
    };
}

test_chrono_creates!(
    naive_date_creates_successfully: NaiveDate,
    naive_time_creates_successfully: NaiveTime,
    naive_date_time_creates_successfully: NaiveDateTime,
    date_time_utc_creates_successfully: DateTime<Utc>,
    date_time_fixed_offset_creates_successfully: DateTime<FixedOffset>,
    date_time_local_creates_successfully: DateTime<Local>,
    time_delta_creates_successfully: TimeDelta,
    weekday_creates_successfully: Weekday,
    month_creates_successfully: Month,
    days_creates_successfully: Days,
    months_creates_successfully: Months,
    fixed_offset_creates_successfully: FixedOffset
);

macro_rules! test_chrono_varies {
    ($($name:ident: $ty:ty),*) => {
        $(
            #[test]
            fn $name() {
                let mut f = Fixture::new();

                let values: Vec<$ty> = f
                    .create_many(10)
                    .collect();

                let all_same = values.windows(2).all(|w| w[0] == w[1]);
                assert!(!all_same, concat!("expected ", stringify!($ty), " values to vary"));
            }
        )*
    };
}

test_chrono_varies!(
    naive_date_values_vary: NaiveDate,
    naive_time_values_vary: NaiveTime,
    naive_date_time_values_vary: NaiveDateTime,
    date_time_utc_values_vary: DateTime<Utc>,
    date_time_fixed_offset_values_vary: DateTime<FixedOffset>,
    date_time_local_values_vary: DateTime<Local>,
    time_delta_values_vary: TimeDelta,
    fixed_offset_values_vary: FixedOffset
);

#[test]
fn create_many_naive_dates() {
    let mut f = Fixture::new();

    let dates: Vec<NaiveDate> = f
        .create_many(5)
        .collect();

    assert_eq!(dates.len(), 5);
}

#[test]
fn create_many_date_time_utc() {
    let mut f = Fixture::new();

    let dates: Vec<DateTime<Utc>> = f
        .create_many(5)
        .collect();

    assert_eq!(dates.len(), 5);
}

#[test]
fn freeze_naive_date_repeats_on_subsequent_creates() {
    let mut f = Fixture::new();

    let frozen: NaiveDate = f.freeze();
    let created: NaiveDate = f.create();

    assert_eq!(frozen, created);
}
