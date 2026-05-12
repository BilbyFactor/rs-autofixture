//! AutoFixture implementations for `chrono` date/time types.

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

macro_rules! impl_chrono_autofixture {
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

impl_chrono_autofixture!(
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
    Months => MonthsBuilder
);

const EXPECT_VALID_RANGE_MSG: &str = "the data range here should always be valid.";

impl AutoFixture for NaiveDate {
    type Builder<'b> = NaiveDateBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let year = f.rng().random_range(1970..=2100);
        let ordinal = f.rng().random_range(1..=365);

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
        let hour = f.rng().random_range(0..24);
        let min = f.rng().random_range(0..60);
        let sec = f.rng().random_range(0..60);
        let nano = f.rng().random_range(0..1000000000);

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
        let offset_secs = f.rng().random_range(-43200..=50400);

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
        let secs = f.rng().random_range(-86400 * 365..=86400 * 365);
        let nanos = f.rng().random_range(0..1000000000);
        TimeDelta::new(secs, nanos).expect(EXPECT_VALID_RANGE_MSG)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        TimeDeltaBuilder::new(f)
    }
}

impl AutoFixture for Weekday {
    type Builder<'b> = WeekdayBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        match f.rng().random_range(0..7) {
            0 => Weekday::Mon,
            1 => Weekday::Tue,
            2 => Weekday::Wed,
            3 => Weekday::Thu,
            4 => Weekday::Fri,
            5 => Weekday::Sat,
            _ => Weekday::Sun,
        }
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        WeekdayBuilder::new(f)
    }
}

impl AutoFixture for Month {
    type Builder<'b> = MonthBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        match f.rng().random_range(0..12) {
            0 => Month::January,
            1 => Month::February,
            2 => Month::March,
            3 => Month::April,
            4 => Month::May,
            5 => Month::June,
            6 => Month::July,
            7 => Month::August,
            8 => Month::September,
            9 => Month::October,
            10 => Month::November,
            _ => Month::December,
        }
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        MonthBuilder::new(f)
    }
}

impl AutoFixture for Days {
    type Builder<'b> = DaysBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let num = f.rng().random_range(0..=365 * 100);
        Days::new(num)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        DaysBuilder::new(f)
    }
}

impl AutoFixture for Months {
    type Builder<'b> = MonthsBuilder<'b>;

    fn create(f: &mut Fixture) -> Self {
        let num = f.rng().random_range(0..=12 * 100);
        Months::new(num)
    }

    fn build<'b>(f: &'b mut Fixture) -> Self::Builder<'b> {
        MonthsBuilder::new(f)
    }
}
