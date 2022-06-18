//! Provide helper functions create calendar dates from slices

use crate::{
    french::French,
    gregorian::Gregorian,
    hebrew::Hebrew,
    hindu::{OldHinduLunar, OldHinduSolar},
    islamic::Islamic,
    iso::Iso,
    julian::Julian,
    mayan::{MayanHaab, MayanLongCount, MayanTzolkin},
};

pub fn gregorian_from_slice(s: [i64; 3]) -> Gregorian {
    return Gregorian {
        year: s[0],
        month: s[1],
        day: s[2],
    };
}

pub fn iso_from_slice(s: [i64; 3]) -> Iso {
    return Iso {
        year: s[0],
        week: s[1],
        day: s[2],
    };
}

pub fn julian_from_slice(s: [i64; 3]) -> Julian {
    return Julian {
        year: s[0],
        month: s[1],
        day: s[2],
    };
}

pub fn islamic_from_slice(s: [i64; 3]) -> Islamic {
    return Islamic {
        year: s[0],
        month: s[1],
        day: s[2],
    };
}

pub fn hebrew_from_slice(s: [i64; 3]) -> Hebrew {
    return Hebrew {
        year: s[0],
        month: s[1],
        day: s[2],
    };
}

pub fn mayan_long_count_from_slice(s: [i64; 5]) -> MayanLongCount {
    return MayanLongCount {
        baktun: s[0],
        katun: s[1],
        tun: s[2],
        uinal: s[3],
        kin: s[4],
    };
}

pub fn mayan_haab_from_slice(s: [i64; 2]) -> MayanHaab {
    return MayanHaab {
        day: s[1], // switched order as in reference data
        month: s[0],
    };
}

pub fn mayan_tzolkin_from_slice(s: [i64; 2]) -> MayanTzolkin {
    return MayanTzolkin {
        number: s[0],
        name: s[1],
    };
}

pub fn french_from_slice(s: [i64; 3]) -> French {
    return French {
        year: s[0],
        month: s[1],
        day: s[2],
    };
}

pub fn old_hindu_solar_from_slice(s: [i64; 3]) -> OldHinduSolar {
    return OldHinduSolar {
        year: s[0],
        month: s[1],
        day: s[2],
    };
}

pub fn old_hindu_lunar_from_slice(s: [i64; 4]) -> OldHinduLunar {
    return OldHinduLunar {
        year: s[0],
        month: s[1],
        leap_month: s[2] == 1, // as boolean "true" is stored as 1 in json source
        day: s[3],
    };
}
