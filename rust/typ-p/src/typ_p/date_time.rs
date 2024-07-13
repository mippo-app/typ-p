use derive_pb::Pb;
use serde::{Deserialize, Serialize};

use crate::pb::typ_p as pb_typ_p;

use crate::typ_p::value_of::ValueOf;

use pb_typ_p::DateTime as pb_DateTime;
use regex::Regex;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Pb, Hash, PartialOrd, Ord)]
#[pb(
    pb_name = "pb_DateTime",
    module_path = "super::super::pb",
    trait_path = "super::super::pb"
)]
pub struct DateTime {
    year: i32,
    month: i32,
    day: i32,

    hour: i32,
    minute: i32,
    second: i32,
}

use std::cmp::Ord;

impl From<DateTime> for ValueOf {
    fn from(f: DateTime) -> Self {
        ValueOf::DateTimeValue(f)
    }
}

impl DateTime {
    pub fn new(year: i32, month: i32, day: i32, hour: i32, minute: i32, second: i32) -> Self {
        return Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        };
    }

    pub fn from_string(item: String) -> Result<Self, Box<dyn std::error::Error>> {
        // \d{4}/\d{2}/\d{2} \d{2}:\d{2}
        let aaa = item.replace("/", "-").replace(" ", "T");
        let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2})")?;

        let caps = re.captures(aaa.as_str()).unwrap();

        let year: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let month: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let day: i32 = caps.get(3).unwrap().as_str().parse().unwrap();
        let hour: i32 = caps.get(4).unwrap().as_str().parse().unwrap();
        let minute: i32 = caps.get(5).unwrap().as_str().parse().unwrap();
        let second: i32 = caps.get(6).unwrap().as_str().parse().unwrap();

        let r = DateTime {
            year,
            month,
            day,
            hour,
            minute,
            second,
        };

        return Ok(r);
    }
}

impl std::fmt::Display for DateTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}-{:0>2}-{:0>2}T{:0>2}:{:0>2}:{:0>2}",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

/*
impl Ord for DateTime {
  fn cmp(&self, other: &Self) -> Ordering {
    let a = self.year * 100 * 100 * 100 * 100 * 10000
      + self.month * 100 * 100 * 100 * 100
      + self.day * 100 * 100 * 100
      + self.hour * 100 * 100
      + self.minute * 100
      + self.second;

    let b = other.year * 100 * 100 * 100 * 100 * 10000
      + other.month * 100 * 100 * 100 * 100
      + other.day * 100 * 100 * 100
      + other.hour * 100 * 100
      + other.minute * 100
      + other.second;

    return a.cmp(&b);
  }
}
*/
