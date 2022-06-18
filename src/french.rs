//! Functions converting from and to French Revolutionary calendar dates

use crate::math::{floor_div, modulus, sum};

/// French Revolutionary month names
pub static FRENCH_MONTH_NAMES: [&str; 13] = [
    "Vendémiare",
    "Brumaire",
    "Frimaire",
    "Nivôse",
    "Pluviôse",
    "Ventôse",
    "Germinal",
    "Floréal",
    "Prairial",
    "Messidor",
    "Thermidor",
    "Fructidor",
    "Sansculottides",
];

/// French monthless days added at the end of a given year
pub static SANSCULOTTIDES: [&str; 6] = [
    "Jour de la vertu",
    "Jour du génie",
    "Jour du travail",
    "Jour de l'opinion",
    "Jour des récompenses",
    "Jour de la révolution",
];

/// French date
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct French {
    pub year: i64,
    pub month: i64,
    pub day: i64,
}

impl French {
    /// Create a new French Revolutionary date
    pub fn new(year: i64, month: i64, day: i64) -> Self {
        Self { year, month, day }
    }
}

/// Returns the last day of a given French Revolutionary month in a given
/// French Revolutionary year
fn french_last_day_of_month(month: i64, year: i64) -> i64 {
    return if month < 13 {
        30
    } else {
        if french_leap_year(year) {
            6
        } else {
            5
        }
    };
}

/// Returns true if a given year is a leap year, and false otherwise
fn french_leap_year(f_year: i64) -> bool {
    return vec![3, 7, 11].contains(&f_year)
        || vec![15, 20].contains(&f_year)
        || (f_year > 20
            && modulus(f_year, 4) == 0
            && !vec![100, 200, 300].contains(&modulus(f_year, 400))
            && modulus(f_year, 4000) != 0);
}

/// Returns the absolute (fixed) date from a given French Revolutionary date.
pub fn absolute_from_french(d: French) -> i64 {
    let year = d.year;
    let month = d.month;
    let day = d.day;
    return 654414
        + (365 * (year - 1))
        + if year < 20 {
            floor_div(year, 4)
        } else {
            floor_div(year - 1, 4) - floor_div(year - 1, 100) + floor_div(year - 1, 400)
                - floor_div(year - 1, 4000)
        }
        + (30 * (month - 1))
        + day;
}

/// Returns the French Revolutionary date corresponding to a given absolute
/// (fixed) date.
pub fn french_from_absolute(absolute_date: i64) -> French {
    if absolute_date < 654415 {
        return French {
            year: 0,
            month: 0,
            day: 0,
        };
    }
    let approx = floor_div(absolute_date - 654414, 366);
    let year = approx
        + sum(
            |_| 1.0,
            approx,
            |y| {
                absolute_date
                    >= absolute_from_french(French {
                        year: (y as i64) + 1,
                        month: 1,
                        day: 1,
                    })
            },
        ) as i64;
    let month = 1 + sum(
        |_| 1.0,
        1,
        |m| {
            absolute_date
                > absolute_from_french(French {
                    year,
                    month: m as i64,
                    day: french_last_day_of_month(m as i64, year),
                })
        },
    ) as i64;
    let day = absolute_date
        - (absolute_from_french(French {
            year,
            month,
            day: 1,
        }) - 1);
    return French { year, month, day };
}
