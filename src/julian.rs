//! Functions converting from and to Julian calendar dates

use crate::math::{floor_div, modulus};

use super::math::sum;

/// Julian date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Julian {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

impl Julian {
    /// Create new Julian date
    pub fn new(year: i64, month: i64, day: i64) -> Self {
        Self { year, month, day }
    }
}

/// Returns the last day (number of days) of a given Julian month.
fn last_day_of_julian_month(month: i64, year: i64) -> i64 {
    if month == 2 && modulus(year, 4) == 0 {
        return 29;
    } else {
        let days: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        return days[(month - 1) as usize];
    }
}

/// Computes the absolute (fixed) date corresponding to a given Julian date.
pub fn absolute_from_julian(d: Julian) -> i64 {
    let year = d.year;
    let month = d.month;
    let day = d.day;
    let f = |m: f64| return last_day_of_julian_month(m as i64, year) as f64;
    let p = |m: f64| return (m as i64) < month;
    return day + (sum(f, 1, p) as i64) + 365 * (year - 1) + floor_div(year - 1, 4) - 2;
}

/// Computes the Julian date corresponding to a given absolute date.
pub fn julian_from_absolute(absolute_date: i64) -> Julian {
    let approx = floor_div(absolute_date + 2, 366);
    let year = approx
        + sum(
            |_| return 1.0,
            approx,
            |y| {
                return absolute_date >= absolute_from_julian(Julian::new(y as i64 + 1, 1, 1));
            },
        ) as i64;
    let month = 1 + sum(
        |_| return 1.0,
        1,
        |m| {
            return absolute_date
                > absolute_from_julian(Julian::new(
                    year,
                    m as i64,
                    last_day_of_julian_month(m as i64, year),
                ));
        },
    ) as i64;
    let day = absolute_date - (absolute_from_julian(Julian::new(year, month, 1)) - 1);
    return Julian { year, month, day };
}
