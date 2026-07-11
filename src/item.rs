use std::fmt;

use chrono::Month;

/// A year-month pair.
///
/// # Examples
///
/// Displayed as the three-letter month abbreviation followed by the year:
///
/// ```
/// use chrono::Month;
/// use cv_website::item::Date;
///
/// let date = Date { year: 2026, month: Month::July };
/// assert_eq!(date.to_string(), "Jul 2026");
///
/// let date = Date { year: 1999, month: Month::December };
/// assert_eq!(date.to_string(), "Dec 1999");
/// ```
#[derive(Clone, Copy, Debug)]
pub struct Date {
    pub year: i32,
    pub month: Month,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.month.name()[..3], self.year)
    }
}

/// A career range.
///
/// # Examples
///
/// Displayed as:
/// - Same as `Date` for `CareerRange::Single`
/// - "start - end" for `CareerRange::Range`
/// - "start - Ongoing" for `CareerRange::Ongoing`
///
/// ```
/// use chrono::Month;
/// use cv_website::item::{CareerRange, Date};
///
/// let career_range = CareerRange::Single(Date { year:  2026, month: Month::July });
/// assert_eq!(career_range.to_string(), "Jul 2026");
///
/// let start = Date { year: 2025, month: Month::April };
/// let end = Date { year: 2027, month: Month::January };
/// let career_range = CareerRange::Range(start, end);
/// assert_eq!(career_range.to_string(), "Apr 2025 - Jan 2027");
///
/// let career_range = CareerRange::Ongoing(Date { year:  2026, month: Month::July });
/// assert_eq!(career_range.to_string(), "Jul 2026 - Ongoing");
/// ```
#[derive(Clone, Copy, Debug)]
pub enum CareerRange {
    Single(Date),
    Range(Date, Date),
    Ongoing(Date),
}

impl fmt::Display for CareerRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CareerRange::Single(date) => write!(f, "{date}"),
            CareerRange::Range(start, end) => write!(f, "{start} - {end}"),
            CareerRange::Ongoing(start) => write!(f, "{start} - Ongoing"),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Link {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug)]
pub struct CareerItem {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub range: CareerRange,
    pub links: Vec<Link>,
}
