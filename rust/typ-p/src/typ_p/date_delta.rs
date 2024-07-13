use crate::pb::typ_p as pb_typ_p;
use chronoutil::RelativeDuration;
use pb_typ_p::DateDelta as pb_DateDelta;

use derive_pb::Pb;

#[derive(Debug, Clone, PartialEq, Eq, Pb, Hash, PartialOrd, Ord)]
#[pb(pb_name = "pb_DateDelta")]
pub struct DateDelta {
    pub year: i32,
    pub month: i32,
    pub week: i32,
    pub day: i32,
}

impl DateDelta {
    pub fn new(year: i32, month: i32, week: i32, day: i32) -> Self {
        Self {
            year: year,
            month: month,
            week: week,
            day: day,
        }
    }
    pub fn get_year(&self) -> i32 {
        return self.year;
    }

    pub fn get_month(&self) -> i32 {
        return self.month;
    }

    pub fn get_week(&self) -> i32 {
        return self.week;
    }

    pub fn get_day(&self) -> i32 {
        return self.day;
    }

    pub fn get_chrono_duration(&self) -> RelativeDuration {
        let one_year = RelativeDuration::days(self.year.into());
        let one_month = RelativeDuration::months(self.month);
        let one_week = RelativeDuration::weeks(self.week.into());
        let one_day = RelativeDuration::days(self.day.into());

        let one_duration = one_year + one_month + one_week + one_day;

        return one_duration;
    }
}
