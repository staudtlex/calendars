//! Functions converting from and to Islamic calendar dates

use crate::math::{floor_div, modulus};

use super::math::sum;

/// Islamic month names
pub static ISLAMIC_MONTH_NAMES: [&str; 12] = [
    "Muharram",
    "Safar",
    "Rabi I",
    "Rabi II",
    "Jumada I",
    "Jumada II",
    "Rajab",
    "Sha' Ban",
    "Ramadan",
    "Shawwal",
    "Dhu al-Qada",
    "Dhu al-Hijjah",
];

/// Islamic date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Islamic {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

impl Islamic {
    /// Create new Islamic date
    pub fn new(year: i64, month: i64, day: i64) -> Self {
        Self { year, month, day }
    }
}

/// Returns true if a given Islamic year is leap, and false otherwise.
pub fn islamic_leap_year(year: i64) -> bool {
    return modulus(14 + (11 * year), 30) < 11;
}

/// Determines the last day of an Islamic month.
fn last_day_of_islamic_month(month: i64, year: i64) -> i64 {
    if modulus(month, 2) != 0 || (month == 12 && islamic_leap_year(year)) {
        return 30;
    } else {
        return 29;
    }
}

/// Computes the absolute date corresponding to a given Islamic date.
pub fn absolute_from_islamic(d: Islamic) -> i64 {
    let month = d.month;
    let year = d.year;
    let day = d.day;
    return day
        + (29 * (month - 1))
        + floor_div(month, 2)
        + (year - 1) * 354
        + floor_div(3 + (11 * year), 30)
        + 227014;
}

/// Computes the Islamic date corresponding to a given absolute date.
pub fn islamic_from_absolute(absolute_date: i64) -> Islamic {
    if absolute_date < 227014 {
        return Islamic {
            year: 0,
            month: 0,
            day: 0,
        };
    }
    let approx = floor_div(absolute_date - 227014, 355);
    let year = approx
        + sum(
            |_| return 1.0,
            approx,
            |y| {
                return absolute_date
                    >= absolute_from_islamic(Islamic {
                        year: y as i64 + 1,
                        month: 1,
                        day: 1,
                    });
            },
        ) as i64;
    let month = 1 + sum(
        |_| 1.0,
        1,
        |m| {
            return absolute_date
                > absolute_from_islamic(Islamic {
                    year,
                    month: m as i64,
                    day: last_day_of_islamic_month(m as i64, year),
                });
        },
    ) as i64;
    let day = absolute_date
        - (absolute_from_islamic(Islamic {
            year,
            month,
            day: 1,
        }) - 1);
    return Islamic { year, month, day };
}
