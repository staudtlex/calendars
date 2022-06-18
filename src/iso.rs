//! Functions converting from and to ISO week calendar dates

use crate::math::modulus;

use super::gregorian::{absolute_from_gregorian, gregorian_from_absolute, Gregorian};

/// Iso week date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Iso {
    pub year: i64,
    pub week: i64,
    pub day: i64,
}

impl Iso {
    /// Create new ISO week date
    pub fn new(year: i64, week: i64, day: i64) -> Self {
        Self { year, week, day }
    }
}

/// Computes the absolute date of a given week day in the seven-day interval
/// ending on date.
pub fn kday_on_or_before(absolute_date: i64, k: i64) -> i64 {
    return absolute_date - modulus(absolute_date - k, 7);
}

/// Computes the absolute (fixed) date from an ISO date.
pub fn absolute_from_iso(d: Iso) -> i64 {
    let year = d.year;
    let week = d.week;
    let day = d.day;
    return kday_on_or_before(
        absolute_from_gregorian(Gregorian {
            year,
            month: 1,
            day: 4,
        }),
        1,
    ) + 7 * (week - 1)
        + (day - 1);
}

/// Computes the IsoDate corresponding to a given absolute (fixed) date.
pub fn iso_from_absolute(absolute_date: i64) -> Iso {
    let approx = gregorian_from_absolute(absolute_date - 3).year;
    let year = if absolute_date
        >= absolute_from_iso(Iso {
            year: approx + 1,
            week: 1,
            day: 1,
        }) {
        approx + 1
    } else {
        approx
    };
    let week = ((absolute_date
        - absolute_from_iso(Iso {
            year,
            week: 1,
            day: 1,
        }))
        / 7)
        + 1;
    let day = if modulus(absolute_date, 7) == 0 {
        7
    } else {
        modulus(absolute_date, 7)
    };
    return Iso { year, week, day };
}
