use std::fmt;
use std::str::FromStr;

use chrono::Month;
use serde::{Deserialize, Deserializer, Serialize, de};

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Date {
    pub year: i32,
    pub month: Month,
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", &self.month.name()[..3], self.year)
    }
}

impl FromStr for Date {
    type Err = String;

    /// Parses a `Date` from a `"<month> <year>"` string, as produced by
    /// [`Date`]'s `Display`. The month accepts a full or three-letter name.
    ///
    /// # Examples
    ///
    /// ```
    /// use chrono::Month;
    /// use cv_website::item::Date;
    ///
    /// let date: Date = "Apr 2025".parse().unwrap();
    /// assert_eq!(date.year, 2025);
    /// assert_eq!(date.month, Month::April);
    ///
    /// // Full month names work too.
    /// assert!("January 1999".parse::<Date>().is_ok());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let month = parts
            .next()
            .ok_or_else(|| format!("missing month in date {s:?}"))?;
        let year = parts
            .next()
            .ok_or_else(|| format!("missing year in date {s:?}"))?;
        if parts.next().is_some() {
            return Err(format!("unexpected trailing text in date {s:?}"));
        }
        let month = month
            .parse::<Month>()
            .map_err(|_| format!("invalid month {month:?} in date {s:?}"))?;
        let year = year
            .parse::<i32>()
            .map_err(|_| format!("invalid year {year:?} in date {s:?}"))?;
        Ok(Date { year, month })
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
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

impl FromStr for CareerRange {
    type Err = String;

    /// Parses a `CareerRange` from its `Display` representation:
    ///
    /// - `"Jul 2026"` for `CareerRange::Single`
    /// - `"Apr 2025 - Jan 2027"` for `CareerRange::Range`
    /// - `"Jul 2026 - Ongoing"` for `CareerRange::Ongoing`
    ///
    /// # Examples
    ///
    /// ```
    /// use cv_website::item::CareerRange;
    ///
    /// assert!(matches!(
    ///     "Jul 2026".parse().unwrap(),
    ///     CareerRange::Single(_)
    /// ));
    /// assert!(matches!(
    ///     "Apr 2025 - Jan 2027".parse().unwrap(),
    ///     CareerRange::Range(_, _)
    /// ));
    /// assert!(matches!(
    ///     "Jul 2026 - Ongoing".parse().unwrap(),
    ///     CareerRange::Ongoing(_)
    /// ));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('-') {
            None => Ok(CareerRange::Single(s.trim().parse()?)),
            Some((start, end)) => {
                let start = start.trim().parse()?;
                let end = end.trim();
                if end.eq_ignore_ascii_case("Ongoing") {
                    Ok(CareerRange::Ongoing(start))
                } else {
                    Ok(CareerRange::Range(start, end.parse()?))
                }
            }
        }
    }
}

impl<'de> Deserialize<'de> for CareerRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Link {
    pub name: String,
    pub url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CareerItem {
    pub title: String,
    pub subtitle: String,
    pub description: String,
    pub range: CareerRange,
    pub links: Vec<Link>,
}
