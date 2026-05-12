//! AutoFixture implementations for [`chrono`](https://docs.rs/chrono) date/time types.
//!
//! Provides `AutoFixture` for: `NaiveDate`, `NaiveTime`, `NaiveDateTime`,
//! `DateTime<Utc>`, `DateTime<FixedOffset>`, `DateTime<Local>`, `FixedOffset`,
//! `TimeDelta`, `Weekday`, `Month`, `Days`, and `Months`.
//!
//! Requires the `chrono` feature to be enabled.

use chrono::{
    DateTime,
    Days,
    FixedOffset,
    Local,
    Month,
    Months,
    NaiveDate,
    NaiveDateTime,
    NaiveTime,
    TimeDelta,
    TimeZone,
    Utc,
    Weekday,
};

use rand::RngExt;

use crate::fixture::{
    AutoFixture,
    Fixture,
    FixtureExt,
    builder::FixtureBuilder,
};

macro_rules! impl_chrono_autofixture_builder {
    ($($ty:ty => $builder:ident), *) => {
        $(
            pub struct $builder<'b> {
                fixture: &'b mut Fixture,
            }

            impl<'b> FixtureBuilder<'b> for $builder<'b> {
                type F = $ty;

                fn new(f: &'b mut Fixture) -> Self {
                    Self { fixture: f }
                }

                fn create(&mut self) -> Self::F {
                    <$ty as AutoFixture>::create(self.fixture)
                }
            }
        )*
    };
}

impl_chrono_autofixture_builder!(
    NaiveDate => NaiveDateBuilder,
    NaiveTime => NaiveTimeBuilder,
    NaiveDateTime => NaiveDateTimeBuilder,
    DateTime<Utc> => DateTimeUtcBuilder,
    DateTime<FixedOffset> => DateTimeFixedOffsetBuilder,
    DateTime<Local> => DateTimeLocalBuilder,
    TimeDelta => TimeDeltaBuilder,
    Weekday => WeekdayBuilder,
    Month => MonthBuilder,
    Days => DaysBuilder,
    Months => MonthsBuilder,
    FixedOffset => FixedOffsetBuilder
);

const EXPECT_VALID_RANGE_MSG: &str = "the data range here should always be valid.";

const NANOS_PER_SEC: u32 = 1_000_000_000;
const SECS_PER_MIN: u32 = 60;
const MINS_PER_HOUR: u32 = 60;
const HOURS_PER_DAY: u32 = 24;
const DAYS_PER_YEAR: u32 = 365;
const SECS_PER_HOUR: i32 = (SECS_PER_MIN * MINS_PER_HOUR) as i32;
const SECS_PER_DAY: i64 = SECS_PER_HOUR as i64 * HOURS_PER_DAY as i64;

// UTC offset bounds: UTC-12:00 to UTC+14:00, in seconds.
const MIN_UTC_OFFSET_SECS: i32 = -12 * SECS_PER_HOUR;
const MAX_UTC_OFFSET_SECS: i32 = 14 * SECS_PER_HOUR;

impl AutoFixture for NaiveDate {
    type Builder<'b> = NaiveDateBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let year = f.rng().random_range(1970..=2100);
        let ordinal = f.rng().random_range(1..=DAYS_PER_YEAR);

        NaiveDate::from_yo_opt(year, ordinal)
            .unwrap_or(NaiveDate::from_yo_opt(year, 1)
                .expect(EXPECT_VALID_RANGE_MSG)
            )
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        NaiveDateBuilder::new(f)
    }
}

impl AutoFixture for NaiveTime {
    type Builder<'b> = NaiveTimeBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let hour = f.rng().random_range(0..HOURS_PER_DAY);
        let min = f.rng().random_range(0..MINS_PER_HOUR);
        let sec = f.rng().random_range(0..SECS_PER_MIN);
        let nano = f.rng().random_range(0..NANOS_PER_SEC);

        NaiveTime::from_hms_nano_opt(hour, min, sec, nano)
            .expect(EXPECT_VALID_RANGE_MSG)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        NaiveTimeBuilder::new(f)
    }
}

impl AutoFixture for NaiveDateTime {
    type Builder<'b> = NaiveDateTimeBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let date = NaiveDate::create(f);
        let time = NaiveTime::create(f);
        NaiveDateTime::new(date, time)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        NaiveDateTimeBuilder::new(f)
    }
}

impl AutoFixture for DateTime<Utc> {
    type Builder<'b> = DateTimeUtcBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let naive = NaiveDateTime::create(f);
        Utc.from_utc_datetime(&naive)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        DateTimeUtcBuilder::new(f)
    }
}

impl AutoFixture for DateTime<FixedOffset> {
    type Builder<'b> = DateTimeFixedOffsetBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let naive = NaiveDateTime::create(f);
        let offset_secs = f.rng().random_range(MIN_UTC_OFFSET_SECS..=MAX_UTC_OFFSET_SECS);

        let offset = FixedOffset::east_opt(offset_secs)
            .expect(EXPECT_VALID_RANGE_MSG);

        offset.from_utc_datetime(&naive)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        DateTimeFixedOffsetBuilder::new(f)
    }
}

impl AutoFixture for DateTime<Local> {
    type Builder<'b> = DateTimeLocalBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let utc = DateTime::<Utc>::create(f);
        utc.with_timezone(&Local)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        DateTimeLocalBuilder::new(f)
    }
}

impl AutoFixture for TimeDelta {
    type Builder<'b> = TimeDeltaBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let secs = f
            .rng()
            .random_range(-SECS_PER_DAY * DAYS_PER_YEAR as i64..=SECS_PER_DAY * DAYS_PER_YEAR as i64);

        let nanos = f.rng().random_range(0..NANOS_PER_SEC);
        TimeDelta::new(secs, nanos).expect(EXPECT_VALID_RANGE_MSG)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        TimeDeltaBuilder::new(f)
    }
}

impl AutoFixture for Weekday {
    type Builder<'b> = WeekdayBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        Weekday::try_from(f.rng().random_range(0..7u8))
            .expect(EXPECT_VALID_RANGE_MSG)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        WeekdayBuilder::new(f)
    }
}

impl AutoFixture for Month {
    type Builder<'b> = MonthBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        Month::try_from(f.rng().random_range(1..=12u8))
            .expect(EXPECT_VALID_RANGE_MSG)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        MonthBuilder::new(f)
    }
}

impl AutoFixture for Days {
    type Builder<'b> = DaysBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        Days::new(f.rng().random_range(0..=DAYS_PER_YEAR as u64 * 100))
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        DaysBuilder::new(f)
    }
}

impl AutoFixture for Months {
    type Builder<'b> = MonthsBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        Months::new(f.rng().random_range(0..=12 * 100))
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        MonthsBuilder::new(f)
    }
}

impl AutoFixture for FixedOffset {
    type Builder<'b> = FixedOffsetBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let offset_secs = f.rng().random_range(MIN_UTC_OFFSET_SECS..=MAX_UTC_OFFSET_SECS);
        FixedOffset::east_opt(offset_secs).expect(EXPECT_VALID_RANGE_MSG)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        FixedOffsetBuilder::new(f)
    }
}
