//! Create tests
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Deserialize, Clone)]
struct ReferenceDates {
    //note: String,
    rd: Vec<i64>,
    gregorian: Vec<Vec<i64>>,
    julian: Vec<[i64; 3]>,
    iso: Vec<[i64; 3]>,
    islamic: Vec<[i64; 3]>,
    hebrew: Vec<[i64; 3]>,
    mayanlongcount: Vec<[i64; 5]>,
    mayanhaab: Vec<[i64; 2]>,
    mayantzolkin: Vec<[i64; 2]>,
    french: Vec<[i64; 3]>,
    oldhindusolar: Vec<[i64; 3]>,
    oldhindulunar: Vec<[i64; 4]>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct ReferenceHolidays {
    //note: String,
    year: Vec<i64>,
    independence_day: Vec<i64>,
    labor_day: Vec<i64>,
    memorial_day: Vec<i64>,
    daylight_savings_start: Vec<i64>,
    daylight_savings_end: Vec<i64>,
    christmas: Vec<i64>,
    advent: Vec<i64>,
    epiphany: Vec<i64>,
    eastern_orthodox_christmas: Vec<Vec<i64>>,
    nicaean_rule_easter: Vec<i64>,
    easter: Vec<i64>,
    pentecost: Vec<i64>,
    mulad_al_nabi: Vec<Vec<i64>>,
    yom_kippur: Vec<i64>,
    passover: Vec<i64>,
    purim: Vec<i64>,
    ta_anit_esther: Vec<i64>,
    tisha_b_av: Vec<i64>,
}

macro_rules! test_from_absolute {
    ($file:ident, $calendar:literal, $rd:expr, $components:expr) => {
        let calendar = match $calendar {
            "gregorian" =>  ["gregorian", "calendars::gregorian::*"],
            "iso" => ["iso", "calendars::iso::*"],
            "julian" => ["julian", "calendars::julian::*"],
            "islamic" => ["islamic", "calendars::islamic::*"],
            "hebrew" => ["hebrew", "calendars::hebrew::*"],
            "mayanLongCount" => ["mayan_long_count", "calendars::mayan::*"],
            "mayanHaab" => ["mayan_haab", "calendars::mayan::*"],
            "mayanTzolkin" => ["mayan_tzolkin", "calendars::mayan::*"],
            "french" =>  ["french", "calendars::french::*"],
            "oldHinduSolar" =>  ["old_hindu_solar", "calendars::hindu::*"],
            "oldHinduLunar" =>  ["old_hindu_lunar", "calendars::hindu::*"],
            _ => ["", ""],
        };
        let mod_name = format!("test_{}_from_absolute", calendar[0]);
        let s = format!(
            "#[cfg(test)]\nmod {} {{\nextern crate calendars;\nuse {};\nuse calendars::helper::*;",
            mod_name, calendar[1]
        );
        let function = format!("{}_from_absolute", calendar[0]);
        let date_from_slice = format!("{}_from_slice", calendar[0]);
        writeln!($file, "{}", s).unwrap();
        for (absolute_date, date_components) in ($rd).iter().zip(($components).iter()) {
            writeln!(
                $file,
                "#[test]\nfn {name}() {{\nassert_eq!({function}({rd}), {ref_date});\n}}",
                name = if *absolute_date < 0 {
                    "neg_".to_owned() + &absolute_date.abs().to_string()
                } else {
                    "pos_".to_owned() + &absolute_date.to_string()
                },
                function = function,
                rd = absolute_date,
                ref_date = format!("{}({:?})", date_from_slice, date_components),
            )
            .unwrap();
        }
        writeln!($file, "}}").unwrap();
    };
}

macro_rules! test_from_calendar {
    ($file:ident, $calendar:literal, $rd:expr, $components:expr) => {
        let calendar = match $calendar {
            "gregorian" =>  ["gregorian", "calendars::gregorian::*"],
            "iso" => ["iso", "calendars::iso::*"],
            "julian" => ["julian", "calendars::julian::*"],
            "islamic" => ["islamic", "calendars::islamic::*"],
            "hebrew" => ["hebrew", "calendars::hebrew::*"],
            "mayanLongCount" => ["mayan_long_count", "calendars::mayan::*"],
            "mayanHaab" => ["mayan_haab", "calendars::mayan::*"],
            "mayanTzolkin" => ["mayan_tzolkin", "calendars::mayan::*"],
            "french" =>  ["french", "calendars::french::*"],
            "oldHinduSolar" =>  ["old_hindu_solar", "calendars::hindu::*"],
            "oldHinduLunar" =>  ["old_hindu_lunar", "calendars::hindu::*"],
            _ => ["", ""],
        };
        let mod_name = format!("test_absolute_from_{}", calendar[0]);
        let s = format!(
            "#[cfg(test)]\nmod {} {{\nextern crate calendars;\nuse {};\nuse calendars::helper::*;",
            mod_name, calendar[1]
        );
        let function = format!("absolute_from_{}", calendar[0]);
        let date_from_slice = format!("{}_from_slice", calendar[0]);
        writeln!($file, "{}", s).unwrap();
        for (absolute_date, date_components) in ($rd).iter().zip(($components).iter()) {
            writeln!(
                $file,
                "#[test]\nfn {name}() {{\nassert_eq!({function}({date}), {rd});\n}}",
                name = if *absolute_date < 0 {
                    "neg_".to_owned() + &absolute_date.abs().to_string()
                } else {
                    "pos_".to_owned() + &absolute_date.to_string()
                },
                function = function,
                rd = if calendar[0] == "old_hindu_lunar" {
                    format!("Some({})", absolute_date)
                } else {
                    format!("{}", absolute_date)
                },
                date = format!("{}({:?})", date_from_slice, date_components),
            )
            .unwrap();
        }
        writeln!($file, "}}").unwrap();
    };
}

macro_rules! test_holidays {
    ($file:ident, $holiday:literal, $years:expr, $dates:expr) => {
        let holiday = $holiday;
        let mod_name = format!("test_{}", holiday);
        let s = format!(
            "#[cfg(test)]\nmod {} {{\nextern crate calendars;\nuse calendars::holidays::*;",
            mod_name
        );
        writeln!($file, "{}", s).unwrap();
        for (year, date) in ($years).iter().zip(($dates).iter()) {
            writeln!(
                $file,
                "#[test]\nfn {name}() {{\nassert_eq!({function}({year}), {ref_date});\n}}",
                name = format!("y{}", year),
                function = holiday,
                ref_date = format!("({:?})", date),
            )
            .unwrap();
        }
        writeln!($file, "}}").unwrap();
    };
}

fn main() {
    // check if tests-directory exists
    if !Path::new("tests").exists() {
        let r = fs::create_dir("./tests");
        r.expect("an error occurred while creating the tests directory");
    }

    // define file into which test code will be written
    let destination = std::path::Path::new("tests").join("test.rs");
    let mut file = std::fs::File::create(&destination).unwrap();

    // load reference dates
    let dates_json =
        fs::read_to_string("./test-dates/reference_dates_test.json").expect("Unable to read file.");
    let dates: ReferenceDates =
        serde_json::from_str(&dates_json).expect("JSON does not have correct format.");

    // load reference holidays
    let holidays_json = fs::read_to_string("./test-dates/reference_holidays_test.json")
        .expect("Unable to read file.");
    let holidays: ReferenceHolidays =
        serde_json::from_str(&holidays_json).expect("JSON does not have correct format.");

    test_holidays!(
        file,
        "independence_day",
        holidays.year,
        holidays.independence_day
    );
    test_holidays!(file, "labor_day", holidays.year, holidays.labor_day);
    test_holidays!(file, "memorial_day", holidays.year, holidays.memorial_day);
    test_holidays!(
        file,
        "daylight_savings_start",
        holidays.year,
        holidays.daylight_savings_start
    );
    test_holidays!(
        file,
        "daylight_savings_end",
        holidays.year,
        holidays.daylight_savings_end
    );
    test_holidays!(file, "christmas", holidays.year, holidays.christmas);
    test_holidays!(file, "advent", holidays.year, holidays.advent);
    test_holidays!(file, "epiphany", holidays.year, holidays.epiphany);
    test_holidays!(
        file,
        "eastern_orthodox_christmas",
        holidays.year,
        holidays.eastern_orthodox_christmas
    );
    test_holidays!(
        file,
        "nicaean_rule_easter",
        holidays.year,
        holidays.nicaean_rule_easter
    );
    test_holidays!(file, "easter", holidays.year, holidays.easter);
    test_holidays!(file, "pentecost", holidays.year, holidays.pentecost);
    test_holidays!(file, "mulad_al_nabi", holidays.year, holidays.mulad_al_nabi);
    test_holidays!(file, "yom_kippur", holidays.year, holidays.yom_kippur);
    test_holidays!(file, "passover", holidays.year, holidays.passover);
    test_holidays!(file, "purim", holidays.year, holidays.purim);
    test_holidays!(
        file,
        "ta_anit_esther",
        holidays.year,
        holidays.ta_anit_esther
    );
    test_holidays!(file, "tisha_b_av", holidays.year, holidays.tisha_b_av);

    // Gregorian calendar
    test_from_absolute!(file, "gregorian", dates.rd, dates.gregorian);
    test_from_calendar!(file, "gregorian", dates.rd, dates.gregorian);

    // Iso calendar
    test_from_absolute!(file, "iso", dates.rd, dates.iso);
    test_from_calendar!(file, "iso", dates.rd, dates.iso);

    // Julian calendar
    test_from_absolute!(file, "julian", dates.rd, dates.julian);
    test_from_calendar!(file, "julian", dates.rd, dates.julian);

    // Islamic calendar
    test_from_absolute!(file, "islamic", dates.rd, dates.islamic);
    test_from_calendar!(file, "islamic", dates.rd, dates.islamic);

    // Hebrew calendar
    test_from_absolute!(file, "hebrew", dates.rd, dates.hebrew);
    test_from_calendar!(file, "hebrew", dates.rd, dates.hebrew);

    // Mayan Long Count
    test_from_absolute!(file, "mayanLongCount", dates.rd, dates.mayanlongcount);
    test_from_calendar!(file, "mayanLongCount", dates.rd, dates.mayanlongcount);

    // Mayan Haab calendar
    test_from_absolute!(file, "mayanHaab", dates.rd, dates.mayanhaab);

    // Mayan Tzolkin calendar
    test_from_absolute!(file, "mayanTzolkin", dates.rd, dates.mayantzolkin);

    // French Revolutionary calendar
    test_from_absolute!(file, "french", dates.rd, dates.french);
    test_from_calendar!(file, "french", dates.rd, dates.french);

    // Old Hindu Solar calendar
    test_from_absolute!(file, "oldHinduSolar", dates.rd, dates.oldhindusolar);
    test_from_calendar!(file, "oldHinduSolar", dates.rd, dates.oldhindusolar);

    // Old Hindu Lunar calendar
    test_from_absolute!(file, "oldHinduLunar", dates.rd, dates.oldhindulunar);
    test_from_calendar!(file, "oldHinduLunar", dates.rd, dates.oldhindulunar);
}
