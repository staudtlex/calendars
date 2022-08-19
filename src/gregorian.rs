//! Functions converting from and to Gregorian calendar dates

use crate::math::{floor_div, modulus, sum};

/// Gregorian month names
pub static GREGORIAN_MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Gregorian date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Gregorian {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

impl Gregorian {
    /// Create a new Gregorian date
    pub fn new(year: i64, month: i64, day: i64) -> Self {
        Self { year, month, day }
    }
}

/// Returns the last day (number of days) of a given Gregorian month.
pub fn last_day_of_gregorian_month(month: i64, year: i64) -> i64 {
    if month == 2 && modulus(year, 4) == 0 && !vec![100, 200, 300].contains(&modulus(year, 400)) {
        return 29;
    } else {
        let days: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        return days[(month - 1) as usize];
    }
}

/// Computes the absolute (fixed) date from a Gregorian date.
pub fn absolute_from_gregorian(d: Gregorian) -> i64 {
    let month = d.month;
    let year = d.year;
    let day = d.day;

    let f = |m: f64| last_day_of_gregorian_month(m as i64, year) as f64;
    let p = |m: f64| m < (month as f64);

    return day + sum(f, 1, p) as i64 + 365 * (year - 1) + floor_div(year - 1, 4)
        - floor_div(year - 1, 100)
        + floor_div(year - 1, 400);
}

/// Computes the Gregorian date corresponding to a given absolute date.
pub fn gregorian_from_absolute(absolute_date: i64) -> Gregorian {
    let d_0 = absolute_date - 1;
    let n_400 = floor_div(d_0, 146097);
    let d_1 = modulus(d_0, 146097);
    let n_100 = floor_div(d_1, 36524);
    let d_2 = modulus(d_1, 36524);
    let n_4 = floor_div(d_2, 1461);
    let d_3 = modulus(d_2, 1461);
    let n_1 = floor_div(d_3, 365);
    //let d_4 = d_3 % 365;
    let year = if n_100 == 4 || n_1 == 4 {
        400 * n_400 + 100 * n_100 + 4 * n_4 + n_1
    } else {
        400 * n_400 + 100 * n_100 + 4 * n_4 + n_1 + 1
    };
    let p = |m| {
        let d = Gregorian {
            year,
            month: m as i64,
            day: last_day_of_gregorian_month(m as i64, year),
        };
        return absolute_date > absolute_from_gregorian(d);
    };
    let month = sum(|_| 1.0, 1, p) as i64 + 1;
    let day = absolute_date
        - (absolute_from_gregorian(Gregorian {
            year,
            month,
            day: 1,
        }) - 1);
    return Gregorian { year, month, day };
}
