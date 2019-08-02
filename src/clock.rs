//! API for getting the date and time of day.  Enable with the `clock` feature.
//!
//! # Getting Started
//! ```rust,no_run
//! // Print current Local time and UTC time.
//! let clock = cala::Clock::new();
//! println!("Local: {}", clock);
//! println!("UTC:   {:?}", clock);
//!
//! // Print 'Hello, world #!' every 1/3 seconds
//! let mut a = 0;
//! loop {
//!     let now = cala::Clock::new();
//!     let b = now.since(&clock, cala::SECOND / 3);
//!     if a != b {
//!         a = b;
//!         println!("Hello, world {}!", a);
//!     }
//! }
//! ```

use std::ops::{Div, Mul};

/// An amount of time.
pub struct Duration {
    seconds: i32,
    denominator: u32,
}

impl Duration {
    /// Create a new fraction.
    pub const fn new(seconds: i32, denominator: u32) -> Duration {
        Duration {
            seconds,
            denominator,
        }
    }

    //    /// TODO Simplify the fraction.
    //    pub fn simplify() {
    //    }
}

impl Div<i32> for Duration {
    type Output = Duration;

    fn div(mut self, mut other: i32) -> Self::Output {
        if other.is_negative() {
            self.seconds = -self.seconds;
            other = -other;
        }
        Duration {
            seconds: self.seconds,
            denominator: self.denominator * (other as u32),
        }
    }
}

impl Mul<i32> for Duration {
    type Output = Duration;

    fn mul(mut self, mut other: i32) -> Self::Output {
        if other.is_negative() {
            self.seconds = -self.seconds;
            other = -other;
        }
        Duration {
            seconds: self.seconds * (other as i32),
            denominator: self.denominator,
        }
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}/{}", self.seconds, self.denominator)
    }
}

use chrono::{Datelike, TimeZone, Timelike};
use std::fmt::*;

/// 1 nanosecond.
pub const NANOSECOND: Duration = Duration::new(1, 1_000_000_000);
/// 1 microsecond.
pub const MICROSECOND: Duration = Duration::new(1, 1_000_000);
/// 1 millisecond.
pub const MILLISECOND: Duration = Duration::new(1, 1_000);
/// 1 second.
pub const SECOND: Duration = Duration::new(1, 1);
/// 1 minute.
pub const MINUTE: Duration = Duration::new(60, 1);
/// 1 hour.
pub const HOUR: Duration = Duration::new(60 * 60, 1);
/// 1 day.
pub const DAY: Duration = Duration::new(24 * 60 * 60, 1);

/// Month of the year.
#[repr(u8)]
pub enum Month {
    /// January
    Jan = 1u8,
    /// Febuary
    Feb = 2,
    /// March
    Mar = 3,
    /// April
    Apr = 4,
    /// May
    May = 5,
    /// June
    Jun = 6,
    /// July
    Jul = 7,
    /// August
    Aug = 8,
    /// September
    Sep = 9,
    /// October
    Oct = 10,
    /// November
    Nov = 11,
    /// December
    Dec = 12,
}

/// Which day of the week.
#[repr(u8)]
pub enum DayOfWeek {
    /// Sunday
    Sunday = 0u8,
    /// Monday
    Monday = 1,
    /// Tuesday
    Tuesday = 2,
    /// Wednesday
    Wednesday = 3,
    /// Thursday
    Thursday = 4,
    /// Friday
    Friday = 5,
    /// Saturday
    Saturday = 6,
}

/// A calendar date and time.  Stored as UTC.
/// ```
/// let clock = cala::Clock::new();
/// println!("{}", clock); // Print out in local time.
/// println!("{:?}", clock); // Print out in UTC.
/// ```
pub struct Clock(chrono::NaiveDateTime);

impl Clock {
    /// Get the current time.
    ///
    /// ```
    /// let clock = cala::Clock::new();
    /// ```
    pub fn new() -> Self {
        Clock(chrono::offset::Utc::now().naive_utc())
    }

    /// Define a utc time.
    pub fn utc(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        min: u8,
        sec: u8,
    ) -> Option<Self> {
        let date = chrono::offset::Utc
            .ymd(year, month as u32, day as u32)
            .and_hms(hour as u32, min as u32, sec as u32);

        Some(Clock(date.naive_utc()))
    }

    /// Define a local time.
    ///
    /// ```
    /// cala::Clock::new();
    /// ```
    pub fn local(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        min: u8,
        sec: u8,
    ) -> Option<Self> {
        let date = chrono::offset::Local
            .ymd(year, month as u32, day as u32)
            .and_hms(hour as u32, min as u32, sec as u32)
            .with_timezone(&chrono::Utc);

        Some(Clock(date.naive_utc()))
    }

    /// Get the year.
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Get the month.
    pub fn month(&self) -> Month {
        let month = self.0.month() as u8;
        unsafe { std::mem::transmute(month) }
    }

    /// Get the day of the month.
    pub fn day(&self) -> u8 {
        self.0.day() as u8
    }

    /// Get the day of the week.
    pub fn dayofweek(&self) -> DayOfWeek {
        let dayofweek = self.0.weekday().num_days_from_sunday() as u8;
        unsafe { std::mem::transmute(dayofweek) }
    }

    /// Get the hour (0-23).
    pub fn hour(&self) -> u8 {
        self.0.hour() as u8
    }

    /// Get the minute (0-59).
    pub fn minute(&self) -> u8 {
        self.0.minute() as u8
    }

    /// Get the second (0-59).
    pub fn second(&self) -> u8 {
        self.0.second() as u8
    }

    /// Get the nanosecond (0-1,999,999,999 b/c leap seconds).
    pub fn nanosecond(&self) -> u32 {
        self.0.nanosecond()
    }

    /// Get the amount of time since another clock in fractions of a second.
    ///
    /// ```
    /// let start = cala::Clock::new();
    /// let nanos_since_start = cala::Clock::new().since(&start, cala::NANOSECOND);
    /// assert!(nanos_since_start >= 0);
    /// ```
    pub fn since(&self, other: &Self, frac: Duration) -> i64 {
        let duration = self.0 - other.0;
        let seconds: i64 = duration.num_seconds();
        let nanos: i64 = (duration
            - chrono::Duration::seconds(duration.num_seconds()))
        .num_nanoseconds()
        .unwrap();

        // Multiply time by reciprocal fraction (numerator).
        let frac_den = frac.denominator as i128;
        let frac_num = frac.seconds as i128;
        let seconds = seconds as i128 * frac_num;
        let nanos = nanos as i128 * frac_num;

        // Denominator
        let seconds_remaining = seconds % frac_den; // what couldn't be divided
        let nanos = nanos + (seconds_remaining * 1_000_000_000);
        let nanos = (nanos / frac_den) as i64;
        let seconds = (seconds / frac_den) as i64;

        // Add together
        seconds + (nanos / 1_000_000_000)
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            chrono::DateTime::<chrono::Local>::from_utc(
                self.0,
                chrono::offset::Local.offset_from_utc_datetime(&self.0)
            )
            .naive_local()
        )
    }
}
