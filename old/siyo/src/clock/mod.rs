//! # Getting Started
//! ```
//! use siyo::clock::*;
//!
//! // Print current Local time and UTC time.
//! let clock = Clock::new();
//! println!("Local: {}", clock);
//! println!("UTC:   {:?}", clock);
//!
//! // Print 'Hello, world #!' every 1/3 seconds
//! /* let mut a = 0;
//! loop {
//!     let now = Clock::new();
//!     let b = now.since(&clock, SECOND / 3);
//!     if a != b {
//!         a = b;
//!         println!("Hello, world {}!", a);
//!     }
//! } */
//! ```

use crate::math::Fr64;
use chrono::{Datelike, TimeZone, Timelike};
use std::fmt::*;

/// Fraction value for nanoseconds.
pub const NANOSECOND: Fr64 = Fr64(1, 1_000_000_000);
/// Fraction value for microseconds.
pub const MICROSECOND: Fr64 = Fr64(1, 1_000_000);
/// Fraction value for milliseconds.
pub const MILLISECOND: Fr64 = Fr64(1, 1_000);
/// Fraction value for seconds.
pub const SECOND: Fr64 = Fr64(1, 1);
/// Fraction value for minutes.
pub const MINUTE: Fr64 = Fr64(60, 1);
/// Fraction value for minutes.
pub const HOUR: Fr64 = Fr64(60 * 60, 1);
/// Fraction value for days.
pub const DAY: Fr64 = Fr64(24 * 60 * 60, 1);

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
/// use siyo::clock::*;
/// let clock = Clock::new();
/// println!("{}", clock); // Print out in local time.
/// println!("{:?}", clock); // Print out in UTC.
/// ```
pub struct Clock(chrono::NaiveDateTime);

impl Clock {
    /// Get the current time.
    ///
    /// ```
    /// use siyo::clock::*;
    /// let clock = Clock::new();
    /// ```
    pub fn new() -> Self {
        Clock(chrono::offset::Utc::now().naive_utc())
    }

    /// Define a utc time.
    pub fn utc(year: i32, month: u8, day: u8, hour: u8, min: u8, sec: u8) -> Option<Self> {
        let date = chrono::offset::Utc
            .ymd(year, month as u32, day as u32)
            .and_hms(hour as u32, min as u32, sec as u32);

        Some(Clock(date.naive_utc()))
    }

    /// Define a local time.
    ///
    /// ```
    /// use siyo::clock::*;
    /// Clock::new();
    /// ```
    pub fn local(year: i32, month: u8, day: u8, hour: u8, min: u8, sec: u8) -> Option<Self> {
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
    /// use siyo::clock::*;
    /// let start = Clock::new();
    /// let nanos_since_start = Clock::new().since(&start, NANOSECOND);
    /// assert!(nanos_since_start >= 0);
    /// ```
    pub fn since(&self, other: &Self, frac: Fr64) -> i64 {
        let duration = self.0 - other.0;
        let seconds: i64 = duration.num_seconds();
        let nanos: i64 = (duration - chrono::Duration::seconds(duration.num_seconds()))
            .num_nanoseconds()
            .unwrap();

        // Multiply time by reciprocal fraction (numerator).
        let frac_den = frac.0 as i128;
        let frac_num = frac.1 as i128;
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

/*#[cfg(test)]
mod tests {
        #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}*/
