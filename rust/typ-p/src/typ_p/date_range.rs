use std::fmt::Display;

use derive_pb::Pb;
use serde::{Deserialize, Serialize};

use crate::pb::typ_p::{self as pb_typ_p};

use pb_typ_p::DateRange as pb_DateRange;

use crate::typ_p::{Date, DateDelta};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Pb, Hash, PartialOrd, Ord)]
#[pb(pb_name = "pb_DateRange")]
pub struct DateRange {
    date_from: Option<Date>,
    date_to: Option<Date>,
}

impl Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(date_from) = &self.date_from {
            write!(f, "{}-", date_from)?;
        }

        if let Some(date_to) = &self.date_to {
            write!(f, "{}", date_to)?;
        }

        return Ok(());
    }
}

impl DateRange {
    pub fn new(date_from: Option<Date>, date_to: Option<Date>) -> Self {
        return Self {
            date_from: date_from,
            date_to: date_to,
        };
    }

    /*
    pub fn gen_date_range_weekly(
        a: DateRange,
        from_expand: bool,
        to_expand: bool,
    ) -> Vec<Date> {
        //

        let start_date = a.date_from.unwrap().clone();
        let end_date = a.date_to.unwrap().clone();

        while start_date < end_date {


            start_date +=
        }

    } */
}

impl Iterator for DateRange {
    type Item = Date;

    fn next(&mut self) -> Option<Date> {
        // let new_date = self.date_from.clone().unwrap() + DateDelta::new(0, 0, 0, 1);
        let a_date = self.date_from.clone().unwrap();

        if a_date < self.date_to.clone().unwrap() {
            self.date_from = Some(a_date.clone() + DateDelta::new(0, 0, 0, 1));

            return Some(a_date);
        } else {
            return None;
        }
    }
}

// cargo test date_range_tests::it_works -- --exact

// cargo test date_range_tests::  -- --nocapture

#[cfg(test)]
mod date_range_tests {
    use super::{Date, DateRange};

    #[test]
    fn it_works() {
        let date_from = Date::new(2019, 1, 1);
        let date_to = Date::new(2021, 1, 1);

        let date_range = DateRange::new(Some(date_from), Some(date_to));

        for a_date in date_range.into_iter() {
            println!("{:}", a_date);
        }

        assert_eq!(true, true);
    }
}

/*
impl From<DateRange> for pb_DateRange {
  fn from(item: DateRange) -> Self {
    return Self {
      date_from: option_into(item.date_from),
      date_to: option_into(item.date_to),
    };
  }
}
*/
pub fn option_into<T, V>(v: Option<T>) -> Option<V>
where
    T: From<V>,
    V: From<T>,
{
    if let Some(v) = v {
        let a = V::from(v);

        return Some(a);
    } else {
        return None;
    }
}

impl DateRange {
    pub fn combine() {}

    pub fn get_date_from(&self) {}

    pub fn get_date_to(&self) {}

    pub fn get_date_delta(&self) {}

    pub fn shift(&self) {}

    pub fn isin(&self) {}
}
