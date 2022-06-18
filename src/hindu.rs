//! Functions to convert from and to Old Hindu calendar dates

use crate::math::{amod, modulus, sum};

/// Hindu solar month names
pub static HINDU_SOLAR_MONTH_NAMES: [&str; 12] = [
    "Mesha",
    "Vrshabha",
    "Mithuna",
    "Karka",
    "Simha",
    "Kanya",
    "Tula",
    "Vrischika",
    "Dhanus",
    "Makara",
    "Kumbha",
    "Mina",
];

/// Hindu lunar month names
pub static HINDU_LUNAR_MONTH_NAMES: [&str; 12] = [
    "Chaitra",
    "Vaisakha",
    "Jyaishtha",
    "Ashadha",
    "Sravana",
    "Bhadrapada",
    "Asvina",
    "Kartika",
    "Margasira",
    "Pausha",
    "Magha",
    "Phalguna",
];

static SOLAR_SIDEREAL_YEAR: f64 = 365.0 + (279457.0 / 1080000.);
static SOLAR_MONTH: f64 = SOLAR_SIDEREAL_YEAR / 12.0;
static LUNAR_SIDEREAL_MONTH: f64 = 27.0 + (4644439.0 / 14438334.0);
static LUNAR_SYNODIC_MONTH: f64 = 29.0 + (7087771.0 / 13358334.0);

/// Old Hindu Solar date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OldHinduSolar {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

impl OldHinduSolar {
    /// Create a new Old Hindu Solar date
    pub fn new(year: i64, month: i64, day: i64) -> Self {
        Self { year, month, day }
    }
}

/// Returns the position of the sun (in degrees) for a given moment (day and
/// fraction of a day).
fn solar_longitude(days: f64) -> f64 {
    return modulus(days / SOLAR_SIDEREAL_YEAR, 1.0) * 360.0;
}

/// Returns the zodiacal sign for a given moment (day and fraction of day).
fn zodiac(days: f64) -> i64 {
    return ((solar_longitude(days) / 30.0).floor() + 1.0) as i64;
}

/// Computes the Old Hindu solar date corresponding to a given absolute
/// (fixed) date.
pub fn old_hindu_solar_from_absolute(absolute_date: i64) -> OldHinduSolar {
    let h_date = absolute_date as f64 + 1132959.0 + (1.0 / 4.0);
    let year = (h_date / SOLAR_SIDEREAL_YEAR).floor() as i64;
    let month = zodiac(h_date);
    let day = (modulus(h_date, SOLAR_MONTH).floor() + 1.0) as i64;
    return OldHinduSolar { year, month, day };
}

/// Returns the absolute (fixed) date from a given Old Hindu solar date.
pub fn absolute_from_old_hindu_solar(d: OldHinduSolar) -> i64 {
    let year = d.year;
    let month = d.month;
    let day = d.day;
    return ((year as f64 * SOLAR_SIDEREAL_YEAR) + ((month - 1) as f64 * SOLAR_MONTH) + day as f64
        - (1.0 / 4.0)
        - 1132959.0)
        .floor() as i64;
}

/// Old Hindu Lunar date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct OldHinduLunar {
    pub year: i64,
    pub month: i64,
    pub leap_month: bool,
    pub day: i64,
}

impl OldHinduLunar {
    /// Create a new Old Hindu Solar date
    pub fn new(year: i64, month: i64, leap_month: bool, day: i64) -> Self {
        Self {
            year,
            month,
            leap_month,
            day,
        }
    }
}

/// Returns the sidereal longitude of the moon (in degrees) at a given moment
/// (date and fraction of a day).
fn lunar_longitude(days: f64) -> f64 {
    return modulus(days / LUNAR_SIDEREAL_MONTH, 1.0) * 360.0;
}

/// Computes the lunar phase of the moon for a given moment (date and fraction
/// of a day).
fn lunar_phase(days: f64) -> i64 {
    return (1.0 + (modulus(lunar_longitude(days) - solar_longitude(days), 360.0) / 12.0).floor())
        as i64;
}

/// Determines the time of the most recent new moon for a given moment (date
///  and fraction of day).
fn new_moon(days: f64) -> f64 {
    return days - modulus(days, LUNAR_SYNODIC_MONTH);
}

/// Computes the Old Hindu lunar date corresponding to a given absolute (fixed)
/// date.
pub fn old_hindu_lunar_from_absolute(absolute_date: i64) -> OldHinduLunar {
    let h_date = absolute_date + 1132959;
    let sunrise = h_date as f64 + (1.0 / 4.0);
    let last_new_moon = new_moon(sunrise);
    let next_new_moon = last_new_moon + LUNAR_SYNODIC_MONTH;
    let day = lunar_phase(sunrise);
    let month = amod(zodiac(last_new_moon) + 1, 12);
    let leap_month = zodiac(last_new_moon) == zodiac(next_new_moon);
    let next_month = next_new_moon + if leap_month { LUNAR_SYNODIC_MONTH } else { 0.0 };
    let year = (next_month / SOLAR_SIDEREAL_YEAR).floor() as i64;
    return OldHinduLunar {
        year,
        month,
        leap_month,
        day,
    };
}

/// Returns true if a given Hindu lunar date d1 precedes (i.e. is smaller
/// than) a given Hindu lunar date d2, and false otherwise.
fn old_hindu_lunar_precedes(d1: OldHinduLunar, d2: OldHinduLunar) -> bool {
    let year_1 = d1.year;
    let year_2 = d2.year;
    let month_1 = d1.month;
    let month_2 = d2.month;
    let leap_month_1 = d1.leap_month;
    let leap_month_2 = d2.leap_month;
    let day_1 = d1.day;
    let day_2 = d2.day;
    return year_1 < year_2
        || (year_1 == year_2
            && (month_1 < month_2
                || (month_1 == month_2
                    && ((leap_month_1 && !leap_month_2)
                        || ((leap_month_1 == leap_month_2) && (day_1 < day_2))))));
}

/// Returns the absolute (fixed) date corresponding to a given Old Hindu lunar
/// date.
pub fn absolute_from_old_hindu_lunar(d: OldHinduLunar) -> Option<i64> {
    let years = d.year;
    let months = d.month - 2;
    let approx = (years as f64 * SOLAR_SIDEREAL_YEAR).floor() as i64
        + (months as f64 * LUNAR_SYNODIC_MONTH).floor() as i64
        - 1132959;
    let try_value = approx
        + sum(
            |_| 1.0,
            approx,
            |i| old_hindu_lunar_precedes(old_hindu_lunar_from_absolute(i as i64), d),
        ) as i64;
    return if old_hindu_lunar_from_absolute(try_value) == d {
        Some(try_value)
    } else {
        None
    };
}
