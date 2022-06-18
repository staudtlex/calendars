//! Functions converting from and to Mayan dates

use crate::math::{amod, floor_div, modulus};

/// Mayan Long Count
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MayanLongCount {
    pub baktun: i64,
    pub katun: i64,
    pub tun: i64,
    pub uinal: i64,
    pub kin: i64,
}

impl MayanLongCount {
    /// Create a new Mayan Long Count
    pub fn new(baktun: i64, katun: i64, tun: i64, uinal: i64, kin: i64) -> Self {
        Self {
            baktun,
            katun,
            tun,
            uinal,
            kin,
        }
    }
}

/// Number of days of the Mayan calendar epoch before absolute day 0,
/// according to the Goodman-Martinez-Thompson correlation (see Reingold/
/// Dershowitz 2018).
///
/// NB: In Reingold et al. 1993, the authors propose `1137140` (Goodman-
/// Martinez-Thompson correlation) and `1232041` (Spinden);
static MAYAN_DAYS_BEFORE_ABSOLUTE_ZERO: i64 = 1137142; //

/// Returns the absolute (fixed) date of a given Mayan long count.
pub fn absolute_from_mayan_long_count(d: MayanLongCount) -> i64 {
    return d.baktun * 144000 + d.katun * 7200 + d.tun * 360 + d.uinal * 20 + d.kin
        - MAYAN_DAYS_BEFORE_ABSOLUTE_ZERO;
}

/// Computes the Mayan long count corresponding to the given absolute date.
pub fn mayan_long_count_from_absolute(absolute_date: i64) -> MayanLongCount {
    let long_count = absolute_date + MAYAN_DAYS_BEFORE_ABSOLUTE_ZERO;
    let baktun = floor_div(long_count, 144000);
    let day_of_baktun = modulus(long_count, 144000);
    let katun = floor_div(day_of_baktun, 7200);
    let day_of_katun = modulus(day_of_baktun, 7200);
    let tun = floor_div(day_of_katun, 360);
    let day_of_tun = modulus(day_of_katun, 360);
    let uinal = floor_div(day_of_tun, 20);
    let kin = modulus(day_of_tun, 20);
    return MayanLongCount {
        baktun,
        katun,
        tun,
        uinal,
        kin,
    };
}

/// Mayan Haab date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MayanHaab {
    pub day: i64,
    pub month: i64,
}

/// Mayan month names
pub static MAYAN_MONTH_NAMES: [&str; 19] = [
    "Pop", "Uo", "Zip", "Zotz", "Tzec", "Xul", "Yaxkin", "Mol", "Chen", "Yax", "Zac", "Ceh", "Mac",
    "Kankin", "Muan", "Pax", "Kayab", "Cumku", "Uayeb",
];

impl MayanHaab {
    /// Create a new Mayan Long Count
    pub fn new(day: i64, month: i64) -> Self {
        Self { day, month }
    }
}

/// Denotes the haab date at long count 0.0.0.0.0.
static MAYAN_HAAB_AT_EPOCH: MayanHaab = MayanHaab { day: 8, month: 18 };

/// Returns the Mayan haab date corresponding to a given absolute (fixed)
/// date.
pub fn mayan_haab_from_absolute(absolute_date: i64) -> MayanHaab {
    let long_count = absolute_date + MAYAN_DAYS_BEFORE_ABSOLUTE_ZERO;
    let day_of_haab = modulus(
        long_count + MAYAN_HAAB_AT_EPOCH.day + 20 * (MAYAN_HAAB_AT_EPOCH.month - 1),
        365,
    );
    let day = modulus(day_of_haab, 20);
    let month = floor_div(day_of_haab, 20) + 1;
    return MayanHaab { day, month };
}

/// Computes the number of days between two Haab dates.
pub fn mayan_haab_difference(d1: MayanHaab, d2: MayanHaab) -> i64 {
    return modulus(20 * (d2.month - d1.month) + (d2.day - d1.day), 365);
}

/// Returns the absolute (fixed) date of a Mayan Haab date on or before a
/// given absolute date.
pub fn mayan_haab_on_or_before(d: MayanHaab, absolute_date: i64) -> i64 {
    return absolute_date
        - modulus(
            absolute_date - mayan_haab_difference(mayan_haab_from_absolute(0), d),
            365,
        );
}

/// Mayan Tzolkin date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct MayanTzolkin {
    pub number: i64,
    pub name: i64,
}

/// Mayan Tzolkin names
pub static MAYAN_TZOLKIN_NAMES: [&str; 20] = [
    "Imix", "Ik", "Akbal", "Kan", "Chiccan", "Cimi", "Manik", "Lamat", "Muluc", "Oc", "Chuen",
    "Eb", "Ben", "Ix", "Men", "Cib", "Caban", "Etznab", "Cauac", "Ahau",
];

impl MayanTzolkin {
    /// Create a new Mayan Long Count
    pub fn new(number: i64, name: i64) -> Self {
        Self { number, name }
    }
}

// Denotes the Tzolkin date at long count 0.0.0.0.0.
static MAYAN_TZOLKIN_AT_EPOCH: MayanTzolkin = MayanTzolkin {
    number: 4,
    name: 20,
};

/// Returns a Mayan Tzolkin date corresponding to a given absolute (fixed)
/// date.
pub fn mayan_tzolkin_from_absolute(absolute_date: i64) -> MayanTzolkin {
    let long_count = absolute_date + MAYAN_DAYS_BEFORE_ABSOLUTE_ZERO;
    let number = amod(long_count + MAYAN_TZOLKIN_AT_EPOCH.number, 13);
    let name = amod(long_count + MAYAN_TZOLKIN_AT_EPOCH.name, 20);
    return MayanTzolkin { number, name };
}

/// Returns the number of days between two given Mayan Tzolkin dates.
pub fn mayan_tzolkin_difference(d1: MayanTzolkin, d2: MayanTzolkin) -> i64 {
    let number_difference = d2.number - d2.number;
    let name_difference = d2.name - d1.name;
    return modulus(
        number_difference + (13 * modulus(3 * (number_difference - name_difference), 20)),
        260,
    );
}

/// Returns the absolute (fixed) date of a Mayan Tzolkin date on or before a
/// given absolute date.
pub fn mayan_tzolkin_on_or_before(d: MayanTzolkin, absolute_date: i64) -> i64 {
    return absolute_date
        - modulus(
            absolute_date - mayan_tzolkin_difference(mayan_tzolkin_from_absolute(0), d),
            260,
        );
}

/// Returns an Option with the absolute date of the latest date on or before a
/// given Haab date and a given Tzolkin date. Result contains `None` if such a
/// combination is impossible.
pub fn mayan_haab_tzolkin_on_or_before(
    dh: MayanHaab,
    dt: MayanTzolkin,
    absolute_date: i64,
) -> Option<i64> {
    let haab_difference = mayan_haab_difference(mayan_haab_from_absolute(0), dh);
    let tzolkin_difference = mayan_tzolkin_difference(mayan_tzolkin_from_absolute(0), dt);
    let difference = tzolkin_difference - haab_difference;
    return if modulus(difference, 5) == 0 {
        Some(
            absolute_date
                - modulus(
                    absolute_date - (haab_difference + (365 * difference)),
                    18980,
                ),
        )
    } else {
        None
    };
}
