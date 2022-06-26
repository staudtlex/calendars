# calendars - Calendrical calculations in Rust

## About
_calendars_ is a translation into Rust of the Lisp code described and presented in

- [Dershowitz, Nachum, and Edward Reingold. 1990. "Calendrical Calculations", Software - Practice and Experience, 20 (9), 899-928.](https://www.cs.tau.ac.il/~nachum/papers/cc-paper.pdf)
- [Reingold, Edward, Nachum Dershowitz, and Stewart Clamen. 1993. "Calendrical Calculations, II: Three Historical Calendars", Software - Practice & Experience, 23 (4), 383-404.](https://www.cs.tau.ac.il/~nachum/papers/CalendricalCalculationsII.pdf)

and allows the computation of and conversion between dates from 11 calendars: Gregorian, ISO, Julian, Islamic, Hebrew, Mayan (long count, haab, tzolkin), French Revolutionary, and Old Hindu (solar, lunar).

The Lisp source code can be found at <https://www.cs.tau.ac.il/~nachum/calendar-book/papers/>.

## Limitations
The primary motivation for writing _calendars_ was to take first steps in Rust programming, hence there may be some amount of non-idiomatic code.

With regard to the calendar functions, note that:

- _calendars_ does _not_ implement the code discussed in: [Reingold, Edward, and Nachum Dershowitz. 2018. _Calendrical Calculations: The Ultimate Edition_. 4th edition. Cambridge: Cambridge University Press.](https://www.cambridge.org/de/academic/subjects/computer-science/computing-general-interest/calendrical-calculations-ultimate-edition-4th-edition?format=PB&isbn=9781107683167)
- the functions do not generally work for absolute dates smaller than 1 (except the Mayan calendars).
- the Islamic and French Revolutionary calendar functions do not work with dates prior to their respective epochs. If provided with such dates, the functions may return invalid results (this can be seen e.g. when running `cargo test`). 
- `daylight_saving_start` and `daylight_saving_end` in the `holidays`-module use the US rules for determining start and end of DST which are in place since 2007, whereas the corresponding Lisp-functions use the pre-2007 rules.
- for some dates, the Old Hindu solar and lunar calendar functions return results that are off by one day compared to those produced by the (more recent) Lisp-Code in [Reingold/Dershowitz (2018)](https://www.cambridge.org/de/academic/subjects/computer-science/computing-general-interest/calendrical-calculations-ultimate-edition-4th-edition?format=PB&isbn=9781107683167).
