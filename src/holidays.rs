//! Provides functions to compute holiday dates

use crate::math::{floor_div, modulus};

use super::{
    gregorian::{absolute_from_gregorian, last_day_of_gregorian_month, Gregorian},
    hebrew::{
        absolute_from_hebrew, hebrew_leap_year, last_month_of_hebrew_year, long_heshvan,
        short_kislev, Hebrew,
    },
    islamic::{absolute_from_islamic, islamic_from_absolute, Islamic},
    iso::kday_on_or_before,
    julian::{absolute_from_julian, julian_from_absolute, Julian},
};

/// Computes the absolute (fixed) date of the nth kth day in a given month in
/// a given Gregorian year.
fn nth_kday(n: i64, k: i64, month: i64, year: i64) -> i64 {
    return if n > 0 {
        kday_on_or_before(
            absolute_from_gregorian(Gregorian {
                year,
                month,
                day: 7,
            }),
            k,
        ) + (7 * (n - 1))
    } else {
        kday_on_or_before(
            absolute_from_gregorian(Gregorian {
                year,
                month,
                day: last_day_of_gregorian_month(month, year),
            }),
            k,
        ) + (7 * (n + 1))
    };
}

// US holidays

/// Returns the absolute (fixed) date of the US Independence Day.
pub fn independence_day(year: i64) -> i64 {
    return absolute_from_gregorian(Gregorian {
        year,
        month: 7,
        day: 4,
    });
}

/// Returns the absolute (fixed) date of US Labor Day in a given Gregorian
/// year.
pub fn labor_day(year: i64) -> i64 {
    return nth_kday(1, 1, 9, year);
}

/// Returns the absolute (fixed) date of US Memorial Day in a given Gregorian
/// year.
pub fn memorial_day(year: i64) -> i64 {
    return nth_kday(-1, 1, 5, year);
}

/// Returns the absolute (fixed) date of the start of US daylight savings time.
pub fn daylight_savings_start(year: i64) -> i64 {
    return nth_kday(1, 0, 4, year); // before 2007
                                    // return nth_kday(2, 0, 3, year); // since 2007
}

/// Returns the absolute (fixed) date of the end of US daylight savings time.
pub fn daylight_savings_end(year: i64) -> i64 {
    return nth_kday(-1, 0, 10, year); // before 2007
                                      // return nth_kday(1, 0, 11, year); // since 2007
}

// Christian holidays

/// Returns the absolute (fixed) date of Gregorian Christmas in a given
/// Gregorian year.
pub fn christmas(year: i64) -> i64 {
    return absolute_from_gregorian(Gregorian {
        year,
        month: 12,
        day: 25,
    });
}

/// Returns the absolute (fixed) date of Advent in a given Gregorian year.
pub fn advent(year: i64) -> i64 {
    return kday_on_or_before(
        absolute_from_gregorian(Gregorian {
            year,
            month: 12,
            day: 3,
        }),
        0,
    );
}

/// Returns the absolute (fixed) date of Epiphany in a given Gregorian year.
pub fn epiphany(year: i64) -> i64 {
    return 12 + christmas(year);
}

/// Returns the absolute (fixed) date of Eastern Orthodox Christmas in a given
///  Gregorian year.
pub fn eastern_orthodox_christmas(year: i64) -> Vec<i64> {
    let jan_1 = absolute_from_gregorian(Gregorian {
        year,
        month: 1,
        day: 1,
    });
    let dec_31 = absolute_from_gregorian(Gregorian {
        year,
        month: 12,
        day: 31,
    });
    let y = julian_from_absolute(jan_1).year;
    let c1 = absolute_from_julian(Julian {
        year: y,
        month: 12,
        day: 25,
    });
    let c2 = absolute_from_julian(Julian {
        year: y + 1,
        month: 12,
        day: 25,
    });
    let mut res = vec![];
    if jan_1 <= c1 && c1 <= dec_31 {
        res.push(c1);
    }
    if jan_1 <= c2 && c2 <= dec_31 {
        res.push(c2);
    }
    return res;
}

// Computes the absolute (fixed) date of Easter in a given Julian year.
pub fn nicaean_rule_easter(year: i64) -> i64 {
    let shifted_epact = modulus(14 + (11 * modulus(year, 19)), 30);
    let paschal_moon = absolute_from_julian(Julian {
        year,
        month: 4,
        day: 19,
    }) - shifted_epact;
    return kday_on_or_before(paschal_moon + 7, 0);
}

/// Computes the absolute (fixed) date of Easter in a given Gregorian year.
pub fn easter(year: i64) -> i64 {
    let century = floor_div(year, 100) + 1;
    let shifted_epact = modulus(
        14 + (11 * modulus(year, 19)) - floor_div(3 * century, 4)
            + floor_div(5 + (8 * century), 25)
            + (30 * century),
        30,
    );
    let adjusted_epact = if shifted_epact == 0 || (shifted_epact == 1 && 10 < modulus(year, 19)) {
        shifted_epact + 1
    } else {
        shifted_epact
    };
    let paschal_moon = absolute_from_gregorian(Gregorian {
        year,
        month: 4,
        day: 19,
    }) - adjusted_epact;
    return kday_on_or_before(paschal_moon + 7, 0);
}

/// Returns the absolute (fixed) date of Pentecost in a given Gregorian year.
pub fn pentecost(year: i64) -> i64 {
    return 49 + easter(year);
}

// Islamic holidays

/// Returns a slice of absolute dates of a given Islamic date (month, day)
/// that occur in a given Gregorian year (g_year).
pub fn islamic_date_in_gregorian_year(month: i64, day: i64, g_year: i64) -> Vec<i64> {
    let jan_1 = absolute_from_gregorian(Gregorian {
        year: g_year,
        month: 1,
        day: 1,
    });
    let dec_31 = absolute_from_gregorian(Gregorian {
        year: g_year,
        month: 12,
        day: 31,
    });
    let y = islamic_from_absolute(jan_1).year;
    let date_1 = absolute_from_islamic(Islamic {
        year: y,
        month,
        day,
    });
    let date_2 = absolute_from_islamic(Islamic {
        year: y + 1,
        month,
        day,
    });
    let date_3 = absolute_from_islamic(Islamic {
        year: y + 2,
        month,
        day,
    });
    let mut res = vec![];
    if jan_1 <= date_1 && date_1 <= dec_31 {
        res.push(date_1)
    }
    if jan_1 <= date_2 && date_2 <= dec_31 {
        res.push(date_2)
    }
    if jan_1 <= date_3 && date_3 <= dec_31 {
        res.push(date_3)
    }
    return res;
}

/// Computes a vector of absolute (fixed) dates of Mulad al Nabi that occur in
/// a given Gregorian year.
pub fn mulad_al_nabi(g_year: i64) -> Vec<i64> {
    return islamic_date_in_gregorian_year(3, 12, g_year);
}

// Jewish holidays

/// Returns the absolute (fixed) date of Yom Kippur in a given Gregorian year.
pub fn yom_kippur(g_year: i64) -> i64 {
    return absolute_from_hebrew(Hebrew {
        year: g_year + 3761,
        month: 7,
        day: 10,
    });
}

/// Returns the abolute (fixed) date of Passover in a given Gregorian year.
pub fn passover(g_year: i64) -> i64 {
    return absolute_from_hebrew(Hebrew {
        year: g_year + 3760,
        month: 1,
        day: 15,
    });
}

/// Returns the absolute (fixed) date of Purim in a given Gregorian year.
pub fn purim(g_year: i64) -> i64 {
    return absolute_from_hebrew(Hebrew {
        year: g_year + 3760,
        month: last_month_of_hebrew_year(g_year + 3760),
        day: 14,
    });
}

/// Returns the absolute (fixed) date of TaAnitEsther in a given Gregorian
/// year.
pub fn ta_anit_esther(g_year: i64) -> i64 {
    let purim_date = purim(g_year);
    return if modulus(purim_date, 7) == 0 {
        purim_date - 3
    } else {
        purim_date - 1
    };
}

// Returns the absolute (fixed) date of Tisha B'Av in a given Gregorian year.
pub fn tisha_b_av(g_year: i64) -> i64 {
    let ninth_of_av = absolute_from_hebrew(Hebrew {
        year: g_year + 3760,
        month: 5,
        day: 9,
    });
    return if modulus(ninth_of_av, 7) == 6 {
        ninth_of_av + 1
    } else {
        ninth_of_av
    };
}

/// Determines the absolute (fixed) date of the anniversary of a given Hebrew
/// birth date in a given Hebrew year.
pub fn hebrew_birthday(birthdate: Hebrew, h_year: i64) -> i64 {
    let birth_year = birthdate.year;
    let birth_month = birthdate.month;
    let birth_day = birthdate.day;
    return if birth_month == last_month_of_hebrew_year(birth_year) {
        absolute_from_hebrew(Hebrew {
            year: h_year,
            month: last_month_of_hebrew_year(h_year),
            day: birth_day,
        })
    } else {
        absolute_from_hebrew(Hebrew {
            year: h_year,
            month: birth_month,
            day: birth_day,
        })
    };
}

/// Determines the absolute (fixed) date of the anniversary of a given Hebrew
/// death-date in a given Hebrew year.
pub fn yahrzeit(death_date: Hebrew, year: i64) -> i64 {
    let death_year = death_date.year;
    let death_month = death_date.month;
    let death_day = death_date.day;
    match () {
        _ if death_month == 8 && death_day == 30 && !long_heshvan(death_year + 1) => {
            absolute_from_hebrew(Hebrew {
                year,
                month: 9,
                day: 1,
            })
        }
        _ if death_month == 9 && death_day == 30 && short_kislev(death_year + 1) => {
            absolute_from_hebrew(Hebrew {
                year,
                month: 10,
                day: 1,
            })
        }
        _ if death_month == 13 => absolute_from_hebrew(Hebrew {
            year,
            month: last_month_of_hebrew_year(year),
            day: death_day,
        }),
        _ if death_day == 30 && death_month == 12 && !hebrew_leap_year(death_year) => {
            absolute_from_hebrew(Hebrew {
                year,
                month: 11,
                day: 30,
            })
        }
        _ => absolute_from_hebrew(Hebrew {
            year,
            month: death_month,
            day: death_day,
        }),
    }
}
