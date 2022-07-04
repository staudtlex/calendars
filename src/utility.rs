//! Provides generic date struct and Calendar trait for easier date conversion

use crate::{
    french::{absolute_from_french, french_from_absolute, French, FRENCH_MONTH_NAMES},
    hindu::{
        absolute_from_old_hindu_lunar, absolute_from_old_hindu_solar,
        old_hindu_lunar_from_absolute, old_hindu_solar_from_absolute, OldHinduLunar, OldHinduSolar,
        HINDU_LUNAR_MONTH_NAMES, HINDU_SOLAR_MONTH_NAMES,
    },
};

use super::{
    gregorian::{
        absolute_from_gregorian, gregorian_from_absolute, Gregorian, GREGORIAN_MONTH_NAMES,
    },
    hebrew::{
        absolute_from_hebrew, hebrew_from_absolute, hebrew_leap_year, Hebrew, HEBREW_MONTH_NAMES,
    },
    islamic::{absolute_from_islamic, islamic_from_absolute, Islamic, ISLAMIC_MONTH_NAMES},
    iso::{absolute_from_iso, iso_from_absolute, Iso},
    julian::{absolute_from_julian, julian_from_absolute, Julian},
    mayan::{
        absolute_from_mayan_long_count, mayan_haab_from_absolute, mayan_long_count_from_absolute,
        mayan_tzolkin_from_absolute, MayanHaab, MayanLongCount, MayanTzolkin, MAYAN_MONTH_NAMES,
        MAYAN_TZOLKIN_NAMES,
    },
};
use core::panic;
use std::fmt;

// Calendar trait
pub trait Calendar {
    fn to_date(&self) -> Date;
    fn to_absolute(&self) -> i64;
    fn format(&self) -> String;
}

// Date
#[derive(Debug, Clone)]
pub struct Date {
    pub calendar: String,
    pub components: Vec<i64>,
    pub component_names: Vec<String>,
    pub month_names: Vec<String>,
}

/// Implement fmt::Display trait for [`Date`]
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?}", &self.calendar, &self.components)
    }
}

impl Date {
    /// Create new generic date
    pub fn new(
        calendar: &'static str,
        components: Vec<i64>,
        component_names: Vec<String>,
        month_names: Vec<String>,
    ) -> Self {
        Self {
            calendar: calendar.to_string(),
            components,
            component_names,
            month_names,
        }
    }

    /// Convert [`Date`] to boxed Date-type (e.g. boxed Gregorian date)
    pub fn to_calendar_date(&self) -> Box<dyn Calendar> {
        let date = self.clone();
        match self.calendar.as_str() {
            "gregorian" => Box::new(gregorian_from_date(date)),
            "iso" => Box::new(iso_from_date(date)),
            "julian" => Box::new(julian_from_date(date)),
            "islamic" => Box::new(islamic_from_date(date)),
            "hebrew" => Box::new(hebrew_from_date(date)),
            "mayanLongCount" => Box::new(mayan_long_count_from_date(date)),
            "mayanHaab" => Box::new(mayan_haab_from_date(date)),
            "mayanTzolkin" => Box::new(mayan_tzolkin_from_date(date)),
            "french" => Box::new(french_from_date(date)),
            "oldHinduSolar" => Box::new(old_hindu_solar_from_date(date)),
            "oldHinduLunar" => Box::new(old_hindu_lunar_from_date(date)),
            _ => Box::new(gregorian_from_date(date)),
        }
    }

    /// Convert [`Date`] to absolute (fixed) date
    pub fn to_absolute(&self) -> i64 {
        self.to_calendar_date().to_absolute()
    }

    /// Convert a given Date into a Date with the calendar representation
    /// specified in `calendar`.
    ///
    /// Currently supports the following calendars:
    /// * `"gregorian"`
    /// * `"iso"`
    /// * `"julian"`
    /// * `"islamic"`
    /// * `"hebrew"`
    /// * "`mayanLongCount`"
    /// * "`mayanHaab`"
    /// * "`mayanTzolkin`"
    /// * "`french`"
    /// * "`oldHinduSolar`"
    /// * "`oldHinduLunar`"
    pub fn convert_to(&self, calendar: &str) -> Date {
        let date = self.to_absolute();
        match calendar {
            "gregorian" => gregorian_from_absolute(date).to_date(),
            "iso" => iso_from_absolute(date).to_date(),
            "julian" => julian_from_absolute(date).to_date(),
            "islamic" => islamic_from_absolute(date).to_date(),
            "hebrew" => hebrew_from_absolute(date).to_date(),
            "mayanLongCount" => mayan_long_count_from_absolute(date).to_date(),
            "mayanHaab" => mayan_haab_from_absolute(date).to_date(),
            "mayanTzolkin" => mayan_tzolkin_from_absolute(date).to_date(),
            "french" => french_from_absolute(date).to_date(),
            "oldHinduSolar" => old_hindu_solar_from_absolute(date).to_date(),
            "oldHinduLunar" => old_hindu_lunar_from_absolute(date).to_date(),
            _ => gregorian_from_absolute(date).to_date(),
        }
    }

    /// Creates a date string from a [`Date`]
    pub fn format(&self) -> String {
        self.to_calendar_date().format()
    }
}

/// Convert Date into Gregorian date.
fn gregorian_from_date(d: Date) -> Gregorian {
    return Gregorian {
        year: d.components[0],
        month: d.components[1],
        day: d.components[2],
    };
}

/// Convert Date into ISO week date.
fn iso_from_date(d: Date) -> Iso {
    return Iso {
        year: d.components[0],
        week: d.components[1],
        day: d.components[2],
    };
}

/// Convert Date into Julian date.
fn julian_from_date(d: Date) -> Julian {
    return Julian {
        year: d.components[0],
        month: d.components[1],
        day: d.components[2],
    };
}

/// Convert Date into Islamic date.
fn islamic_from_date(d: Date) -> Islamic {
    return Islamic {
        year: d.components[0],
        month: d.components[1],
        day: d.components[2],
    };
}

/// Convert Date into Hebrew date.
fn hebrew_from_date(d: Date) -> Hebrew {
    return Hebrew {
        year: d.components[0],
        month: d.components[1],
        day: d.components[2],
    };
}

/// Convert Date into Mayan Long Count.
fn mayan_long_count_from_date(d: Date) -> MayanLongCount {
    return MayanLongCount {
        baktun: d.components[0],
        katun: d.components[1],
        tun: d.components[2],
        uinal: d.components[3],
        kin: d.components[4],
    };
}

/// Convert Date into Mayan Haab date.
fn mayan_haab_from_date(d: Date) -> MayanHaab {
    return MayanHaab {
        day: d.components[0],
        month: d.components[1],
    };
}

/// Convert Date into Mayan Tzolkin date.
fn mayan_tzolkin_from_date(d: Date) -> MayanTzolkin {
    return MayanTzolkin {
        number: d.components[0],
        name: d.components[1],
    };
}

/// Convert Date into French Revolutionary date.
fn french_from_date(d: Date) -> French {
    return French {
        year: d.components[0],
        month: d.components[1],
        day: d.components[2],
    };
}

/// Convert Date into Old Hindu Solar date.
fn old_hindu_solar_from_date(d: Date) -> OldHinduSolar {
    return OldHinduSolar {
        year: d.components[0],
        month: d.components[1],
        day: d.components[2],
    };
}

/// Convert Date into Old Hindu Lunar date.
fn old_hindu_lunar_from_date(d: Date) -> OldHinduLunar {
    return OldHinduLunar {
        year: d.components[0],
        month: d.components[1],
        leap_month: d.components[2] == 1,
        day: d.components[3],
    };
}

impl Calendar for Gregorian {
    fn to_date(&self) -> Date {
        let month_names = GREGORIAN_MONTH_NAMES
            .iter()
            .map(|s| s.to_string())
            .collect();
        let component_names = ["year", "month", "day"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Date::new(
            "gregorian",
            [self.year, self.month, self.day].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        return absolute_from_gregorian(self.clone());
    }

    fn format(&self) -> String {
        return self.day.to_string()
            + " "
            + GREGORIAN_MONTH_NAMES[(&self.month - 1) as usize]
            + " "
            + &self.year.to_string();
    }
}

impl Calendar for Iso {
    fn to_date(&self) -> Date {
        let component_names = ["year", "week", "day"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Date::new(
            "iso",
            [self.year, self.week, self.day].to_vec(),
            component_names,
            [].to_vec(),
        );
    }

    fn to_absolute(&self) -> i64 {
        return absolute_from_iso(self.clone());
    }

    fn format(&self) -> String {
        return self.year.to_string() + "-W" + &self.week.to_string() + "-" + &self.day.to_string();
    }
}

impl Calendar for Julian {
    fn to_date(&self) -> Date {
        let month_names = GREGORIAN_MONTH_NAMES
            .iter()
            .map(|s| s.to_string())
            .collect();
        let component_names = ["year", "month", "day"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Date::new(
            "julian",
            [self.year, self.month, self.day].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        return absolute_from_julian(self.clone());
    }

    fn format(&self) -> String {
        return self.day.to_string()
            + " "
            + GREGORIAN_MONTH_NAMES[(&self.month - 1) as usize]
            + " "
            + &self.year.to_string();
    }
}

impl Calendar for Islamic {
    fn to_date(&self) -> Date {
        let month_names = ISLAMIC_MONTH_NAMES.iter().map(|s| s.to_string()).collect();
        let component_names = ["year", "month", "day"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Date::new(
            "islamic",
            [self.year, self.month, self.day].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        return absolute_from_islamic(self.clone());
    }

    fn format(&self) -> String {
        return self.day.to_string()
            + " "
            + ISLAMIC_MONTH_NAMES[(&self.month - 1) as usize]
            + " "
            + &self.year.to_string();
    }
}

impl Calendar for Hebrew {
    fn to_date(&self) -> Date {
        let month_names = HEBREW_MONTH_NAMES.iter().map(|s| s.to_string()).collect();
        let component_names = ["year", "month", "day"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Date::new(
            "hebrew",
            [self.year, self.month, self.day].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        return absolute_from_hebrew(self.clone());
    }

    fn format(&self) -> String {
        let month_name = match true {
            _ if hebrew_leap_year(self.year) && self.month == 12 => "Adar I",
            _ if hebrew_leap_year(self.year) && self.month == 13 => "Adar II",
            _ => HEBREW_MONTH_NAMES[(self.month - 1) as usize],
        };
        return self.day.to_string() + " " + month_name + " " + &self.year.to_string();
    }
}

impl Calendar for MayanLongCount {
    fn to_date(&self) -> Date {
        let month_names: Vec<String> = vec![];
        let component_names = ["baktun", "katun", "tun", "uinal", "kin"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Date::new(
            "mayanLongCount",
            [self.baktun, self.katun, self.tun, self.uinal, self.kin].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        return absolute_from_mayan_long_count(self.clone());
    }

    fn format(&self) -> String {
        return self.baktun.to_string()
            + "."
            + &self.katun.to_string()
            + "."
            + &self.tun.to_string()
            + "."
            + &self.uinal.to_string()
            + "."
            + &self.kin.to_string();
    }
}

impl Calendar for MayanHaab {
    fn to_date(&self) -> Date {
        let month_names: Vec<String> = vec![];
        let component_names = ["day", "month"].iter().map(|s| s.to_string()).collect();
        return Date::new(
            "mayanHaab",
            [self.day, self.month].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        panic!()
    }

    fn format(&self) -> String {
        return self.day.to_string() + " " + MAYAN_MONTH_NAMES[(&self.month - 1) as usize];
    }
}

impl Calendar for MayanTzolkin {
    fn to_date(&self) -> Date {
        let month_names: Vec<String> = vec![];
        let component_names = ["number", "name"].iter().map(|s| s.to_string()).collect();
        return Date::new(
            "mayanTzolkin",
            [self.number, self.name].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        panic!()
    }

    fn format(&self) -> String {
        return self.number.to_string() + " " + MAYAN_TZOLKIN_NAMES[(&self.name - 1) as usize];
    }
}

impl Calendar for French {
    fn to_date(&self) -> Date {
        let month_names = FRENCH_MONTH_NAMES.iter().map(|s| s.to_string()).collect();
        let component_names = ["year", "month", "day"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Date::new(
            "french",
            [self.year, self.month, self.day].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        return absolute_from_french(self.clone());
    }

    fn format(&self) -> String {
        return self.day.to_string()
            + " "
            + FRENCH_MONTH_NAMES[(&self.month - 1) as usize]
            + " "
            + &self.year.to_string();
    }
}

impl Calendar for OldHinduSolar {
    fn to_date(&self) -> Date {
        let month_names = HINDU_SOLAR_MONTH_NAMES
            .iter()
            .map(|s| s.to_string())
            .collect();
        let component_names = ["year", "month", "day"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Date::new(
            "oldHinduSolar",
            [self.year, self.month, self.day].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        return absolute_from_old_hindu_solar(self.clone());
    }

    fn format(&self) -> String {
        return self.day.to_string()
            + " "
            + HINDU_SOLAR_MONTH_NAMES[(&self.month - 1) as usize]
            + " "
            + &self.year.to_string();
    }
}

impl Calendar for OldHinduLunar {
    fn to_date(&self) -> Date {
        let month_names = HINDU_LUNAR_MONTH_NAMES
            .iter()
            .map(|s| s.to_string())
            .collect();
        let component_names = ["year", "month", "leapMonth", "day"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        return Date::new(
            "gregorian",
            [self.year, self.month, self.day].to_vec(),
            component_names,
            month_names,
        );
    }

    fn to_absolute(&self) -> i64 {
        return absolute_from_old_hindu_lunar(self.clone()).unwrap();
    }

    fn format(&self) -> String {
        return self.day.to_string()
            + " "
            + HINDU_LUNAR_MONTH_NAMES[(&self.month - 1) as usize]
            + " "
            + &self.year.to_string();
    }
}
