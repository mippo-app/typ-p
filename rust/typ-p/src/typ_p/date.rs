use derive_pb::Pb;
use serde::{Deserialize, Serialize};

use crate::pb::typ_p::{self as pb_typ_p};
use pb_typ_p::Date as pb_Date;

use crate::typ_p::value_of::ValueOf;
use crate::typ_p::DateDelta;

use chrono::Datelike;

use std::{
    convert::{TryFrom, TryInto},
    error::Error,
    ops,
};

use super::Value;

impl From<chrono::NaiveDate> for Date {
    fn from(value: chrono::NaiveDate) -> Self {
        return Self::new(value.year() as u16, value.month() as u8, value.day() as u8);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Pb, Hash, PartialOrd, Ord)]
#[pb(pb_name = "pb_Date")]
pub struct Date {
    value: u32,
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (y, m, d) = self.get_value();
        write!(f, "{}-{:0>2}-{:0>2}", y, m, d)
    }
}

impl From<Date> for ValueOf {
    fn from(a: Date) -> Self {
        return ValueOf::DateValue(a);
    }
}

impl From<Date> for Value {
    fn from(a: Date) -> Self {
        return Value::new(ValueOf::DateValue(a));
    }
}

impl TryFrom<Value> for Date {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let a = value.value_of.unwrap().as_date().unwrap();

        return Ok(a);
    }
}

impl TryFrom<&Value> for Date {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let a = &value.value_of.as_ref().unwrap().as_date().unwrap();

        return Ok(a.clone());
    }
}

impl TryFrom<Value> for chrono::NaiveDate {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let a: Date = value.value_of.unwrap().as_date().unwrap();

        return Ok(a.get_chrono_date());
    }
}

impl TryFrom<&Value> for chrono::NaiveDate {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let a = &value.value_of.as_ref().unwrap().as_date().unwrap();

        return Ok(a.get_chrono_date());
    }
}

impl TryFrom<&Value> for chrono::NaiveDateTime {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        let a = &value.value_of.as_ref().unwrap().as_date().unwrap();

        return Ok(a.get_chrono_date().into());
    }
}

/*
impl From<pb_Date> for Date {
  fn from(item: pb_Date) -> Self {
    return Date {
      year: item.year,
      month: item.month,
      day: item.day,
    };
  }
}

impl From<Date> for pb_Date {
  fn from(item: Date) -> Self {
    return Self {
      year: item.year,
      month: item.month,
      day: item.day,
    };
  }
}
*/

impl ops::Add<DateDelta> for Date {
    type Output = Date;

    fn add(self, _rhs: DateDelta) -> Date {
        let chrono_date = self.get_chrono_date();

        let a_date = chrono_date + _rhs.get_chrono_duration();

        return Date::new(
            u16::try_from(a_date.year()).unwrap(),
            u8::try_from(a_date.month()).unwrap(),
            u8::try_from(a_date.day()).unwrap(),
        );
    }
}
// add, compare

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        // 16 + 8 + 8
        let y_bit = u32::from(year) << 16;
        let m_bit = u32::from(month) << 8;
        let d_bit = u32::from(day);

        let a_bit = y_bit | m_bit | d_bit;

        Self { value: a_bit }
    }

    pub fn new_from_string(v: String) -> Result<Self, Box<dyn Error>> {
        let v: Vec<&str> = v.split('-').collect();

        let y = v.get(0).unwrap().parse()?;
        let m = v.get(1).unwrap().parse()?;
        let d = v.get(2).unwrap().parse()?;

        return Ok(Self::new(y, m, d));
    }

    pub fn get_value(&self) -> (u16, u8, u8) {
        let y_mask = u32::from(!0_u16) << 16;
        let m_mask = u32::from(!0_u8) << 8;
        let d_mask = u32::from(!0_u8);

        let y = (self.value & y_mask) >> 16;
        let m = (self.value & m_mask) >> 8;
        let d = self.value & d_mask;

        let y = u16::try_from(y).unwrap();
        let m = u8::try_from(m).unwrap();
        let d = u8::try_from(d).unwrap();

        return (y, m, d);
    }

    pub fn today() -> Self {
        let date = chrono::Utc::now().date_naive();

        return Self::new(
            date.year().try_into().unwrap(),
            date.month().try_into().unwrap(),
            date.day().try_into().unwrap(),
        );
    }

    pub fn get_weekday(&self) -> u32 {
        return self.get_chrono_date().iso_week().week();
    }

    pub fn get_chrono_date(&self) -> chrono::NaiveDate {
        let (y, m, d) = self.get_value();
        let date = chrono::NaiveDate::from_ymd_opt(
            y.try_into().unwrap(),
            m.try_into().unwrap(),
            d.try_into().unwrap(),
        )
        .unwrap();

        return date;
    }

    pub fn get_string(&self) -> String {
        return self.get_chrono_date().format("%Y-%m-%d").to_string();
    }

    pub fn gen_from_isocalendar(year: i32, weeknumber: u32, dw: i32) -> Self {
        let mut weekday = chrono::Weekday::Mon;
        match dw {
            1 => {
                weekday = chrono::Weekday::Mon;
            }
            2 => {
                weekday = chrono::Weekday::Tue;
            }
            3 => {
                weekday = chrono::Weekday::Wed;
            }
            4 => {
                weekday = chrono::Weekday::Thu;
            }
            5 => {
                weekday = chrono::Weekday::Fri;
            }
            6 => {
                weekday = chrono::Weekday::Sat;
            }
            7 => {
                weekday = chrono::Weekday::Sun;
            }
            _ => {}
        }
        let d = chrono::NaiveDate::from_isoywd_opt(year, weeknumber, weekday).unwrap();

        return Self::new(
            d.year().try_into().unwrap(),
            d.month().try_into().unwrap(),
            d.day().try_into().unwrap(),
        );
    }

    // pub fn get_date(&self) {}

    /*pub fn add_date_delta(&self) -> Self {
      let a = self.get_chrono_date();


    }*/
}
/*
impl Eq for Date {}

impl PartialEq for Date {
  fn eq(&self, other: &Self) -> bool {
    if self.year == other.year && self.month == other.month && self.day == other.day {
      return true;
    } else {
      return false;
    }
  }
}
*/

/*
impl Ord for Date {
  fn cmp(&self, other: &Self) -> Ordering {
    let a = self.get_chrono_date();
    let b = other.get_chrono_date();

    return a.cmp(&b);
  }
}

impl PartialOrd for Date {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
*/

// cargo test date_tests::it_works -- --exact
// cargo test date_tests::

#[cfg(test)]
mod date_tests {
    use crate::typ_p::Date;

    #[test]
    fn it_works() {
        let (y, m, d) = (2022, 01, 01);

        let date = Date::new(y, m, d);

        let (n_y, n_m, n_d) = date.get_value();

        assert_eq!(y, n_y);
        assert_eq!(m, n_m);
        assert_eq!(d, n_d);
    }
}
