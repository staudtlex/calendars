// Copyright (C) 2022  Alexander Staudt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Package calendars implements functions to compute and convert dates
// from various calendars. These are the
// Gregorian, ISO, Julian, Islamic, Hebrew, Mayan (long count, haab, tzolkin),
// French Revolutionary, and Old Hindu (solar, lunar) calendars.
//
// The calendrical algorithms are a translation of the Lisp code discussed in:
//
// Dershowitz, Nachum, and Edward Reingold. 1990. "Calendrical Calculations",
// Software - Practice and Experience, 20 (9), 899-928.
// https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.17.4274
//
// Reingold, Edward, Nachum Dershowitz, and Stewart Clamen. 1993. "Calendrical
// Calculations, II: Three Historical Calendars", Software - Practice &
// Experience, 23 (4), 383-404.
// https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.13.9215

//! Calendrical calculations
//!
//! `calendars` provides functions to compute and convert dates from
//! 11 calendars.

pub mod french;
pub mod gregorian;
pub mod hebrew;
pub mod helper;
pub mod hindu;
pub mod holidays;
pub mod islamic;
pub mod iso;
pub mod julian;
pub mod math;
pub mod mayan;
pub mod utility;
