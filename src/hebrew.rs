//! Functions converting from and to Hebrew calendar dates

use crate::math::{floor_div, modulus};

use super::math::sum;

/// Hebrew month names
pub static HEBREW_MONTH_NAMES: [&str; 14] = [
    "Nisan", "Iyyar", "Sivan", "Tammuz", "Av", "Elul", "Tishri", "Heshvan", "Kislev", "Teveth",
    "Shevat", "Adar ", "Adar I", "Adar II",
];

/// Hebrew date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Hebrew {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

impl Hebrew {
    /// Create new Hebrew date
    pub fn new(year: i64, month: i64, day: i64) -> Self {
        Self { year, month, day }
    }
}

/// Returns true if year is a Hebrew leap year.
pub fn hebrew_leap_year(year: i64) -> bool {
    return modulus((year * 7) + 1, 19) < 7;
}

/// Returns the last month of a given Hebrew year.
pub fn last_month_of_hebrew_year(year: i64) -> i64 {
    if hebrew_leap_year(year) {
        return 13;
    } else {
        return 12;
    }
}

/// Returns the day (number of days) of a given Hebrew month.
fn last_day_of_hebrew_month(month: i64, year: i64) -> i64 {
    if [2, 4, 6, 10, 13].contains(&month)
        || (month == 12 && !hebrew_leap_year(year))
        || (month == 8 && !long_heshvan(year))
        || (month == 9 && short_kislev(year))
    {
        return 29;
    } else {
        return 30;
    }
}

/// Computes the number of days elapsed from the Sunday prior to the start
/// of the Hebrew calendar to the mean conjunction of Tishri of a given Hebrew
/// year.
fn hebrew_calendar_elapsed_days(year: i64) -> i64 {
    let months_elapsed = 235 * floor_div(year - 1, 19)
        + 12 * modulus(year - 1, 19)
        + floor_div(modulus(year - 1, 19) * 7 + 1, 19);
    let parts_elapsed = 204 + (793 * modulus(months_elapsed, 1080));
    let hours_elapsed = 5
        + (12 * months_elapsed)
        + (793 * floor_div(months_elapsed, 1080))
        + floor_div(parts_elapsed, 1080);
    let day = 1 + (29 * months_elapsed) + floor_div(hours_elapsed, 24);
    let parts = (1080 * modulus(hours_elapsed, 24)) + modulus(parts_elapsed, 1080);
    let alternative_day = if (parts >= 19440)
        || (modulus(day, 7) == 2 && parts >= 9924 && !hebrew_leap_year(year))
        || (modulus(day, 7) == 1 && parts >= 16789 && hebrew_leap_year(year - 1))
    {
        day + 1
    } else {
        day
    };
    if [0, 3, 5].contains(&modulus(alternative_day, 7)) {
        return alternative_day + 1;
    } else {
        return alternative_day;
    }
}

/// Computes the number of days in a given Hebrew year.
fn days_in_hebrew_year(year: i64) -> i64 {
    return hebrew_calendar_elapsed_days(year + 1) - hebrew_calendar_elapsed_days(year);
}

/// Returns true if Heshvan is long in a given Hebrew year.
pub fn long_heshvan(year: i64) -> bool {
    return modulus(days_in_hebrew_year(year), 10) == 5;
}

/// Returns true if Kislev is short in a given Hebrew year.
pub fn short_kislev(year: i64) -> bool {
    return modulus(days_in_hebrew_year(year), 10) == 3;
}

/// Computes the absolute (fixed) date from a given Hebrew date.
pub fn absolute_from_hebrew(d: Hebrew) -> i64 {
    let year = d.year;
    let month = d.month;
    let day = d.day;
    return day
        + if month < 7 {
            sum(
                |m| last_day_of_hebrew_month(m as i64, year) as f64,
                7,
                |m| m as i64 <= last_month_of_hebrew_year(year),
            ) as i64
                + sum(
                    |m| last_day_of_hebrew_month(m as i64, year) as f64,
                    1,
                    |m| (m as i64) < month,
                ) as i64
        } else {
            sum(
                |m| last_day_of_hebrew_month(m as i64, year) as f64,
                7,
                |m| (m as i64) < month,
            ) as i64
        }
        + hebrew_calendar_elapsed_days(year)
        - 1373429;
}

/// Computes the Hebrew date corresponding to a given absolute (fixed) date
pub fn hebrew_from_absolute(absolute_date: i64) -> Hebrew {
    let approx = modulus(absolute_date + 1373429, 366);
    let year = approx
        + sum(
            |_| 1.0,
            approx,
            |y| {
                absolute_date
                    >= absolute_from_hebrew(Hebrew {
                        year: (y as i64) + 1,
                        month: 7,
                        day: 1,
                    })
            },
        ) as i64;
    let start = if absolute_date
        < absolute_from_hebrew(Hebrew {
            year,
            month: 1,
            day: 1,
        }) {
        7
    } else {
        1
    };
    let month = start
        + sum(
            |_| 1.0,
            start,
            |m| {
                absolute_date
                    > absolute_from_hebrew(Hebrew {
                        year,
                        month: m as i64,
                        day: last_day_of_hebrew_month(m as i64, year),
                    })
            },
        ) as i64;
    let day = absolute_date
        - (absolute_from_hebrew(Hebrew {
            year,
            month,
            day: 1,
        }) - 1);
    return Hebrew { year, month, day };
}
