use crate::typ_p::operator::Operator;
use crate::typ_p::value_of::ValueOf;

use crate::typ_p::{Date, DateRange, DateTime};

use regex::Regex;

use super::Uuid;

pub trait ValueOfCompare {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool>;
}

impl ValueOfCompare for ValueOf {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        return match self {
            ValueOf::BoolValue(a) => a.compare_value_of(operator, b),
            ValueOf::StringValue(a) => a.compare_value_of(operator, b),
            ValueOf::BytesValue(a) => a.compare_value_of(operator, b),
            ValueOf::UuidValue(a) => a.compare_value_of(operator, b),
            ValueOf::IntValue(a) => a.compare_value_of(operator, b),
            ValueOf::DoubleValue(a) => a.compare_value_of(operator, b),
            ValueOf::DateTimeValue(a) => a.compare_value_of(operator, b),
            ValueOf::DateValue(a) => a.compare_value_of(operator, b),
            ValueOf::DateRangeValue(a) => a.compare_value_of(operator, b),
            ValueOf::ArrayValue(a) => a.compare_value_of(operator, b),
            _ => None,
        };
    }
}

impl ValueOfCompare for bool {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::BoolValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX
                | Operator::GT
                | Operator::LT
                | Operator::GTE
                | Operator::LTE
                | Operator::IN
                | Operator::NotIn => None,
            };
        }
        return None;
    }
}

impl ValueOfCompare for String {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::StringValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX => {
                    let re = Regex::new(self).unwrap();
                    Some(re.is_match(b))
                }
                Operator::GT
                | Operator::LT
                | Operator::GTE
                | Operator::LTE
                | Operator::IN
                | Operator::NotIn => None,
            };
        }
        return None;
    }
}

impl ValueOfCompare for Vec<u8> {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::BytesValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX
                | Operator::GT
                | Operator::LT
                | Operator::GTE
                | Operator::LTE
                | Operator::IN
                | Operator::NotIn => None,
            };
        }
        return None;
    }
}

impl ValueOfCompare for Uuid {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::UuidValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX
                | Operator::GT
                | Operator::LT
                | Operator::GTE
                | Operator::LTE
                | Operator::IN
                | Operator::NotIn => None,
            };
        }
        return None;
    }
}

impl ValueOfCompare for i64 {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::IntValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX => None,
                Operator::GT => Some(self < b),
                Operator::GTE => Some(self <= b),
                Operator::LT => Some(self > b),
                Operator::LTE => Some(self >= b),
                Operator::IN | Operator::NotIn => None,
            };
        }
        return None;
    }
}

impl ValueOfCompare for f64 {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::DoubleValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX => None,
                Operator::GT => Some(self < b),
                Operator::GTE => Some(self <= b),
                Operator::LT => Some(self > b),
                Operator::LTE => Some(self >= b),
                Operator::IN | Operator::NotIn => None,
            };
        }
        return None;
    }
}

impl ValueOfCompare for DateTime {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::DateTimeValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX => None,
                Operator::GT => Some(self < b),
                Operator::GTE => Some(self <= b),
                Operator::LT => Some(self > b),
                Operator::LTE => Some(self >= b),
                Operator::IN | Operator::NotIn => None,
            };
        }
        return None;
    }
}

impl ValueOfCompare for Date {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::DateValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX => None,
                Operator::GT => Some(self < b),
                Operator::GTE => Some(self <= b),
                Operator::LT => Some(self > b),
                Operator::LTE => Some(self >= b),
                Operator::IN | Operator::NotIn => None,
            };
        }
        return None;
    }
}

impl ValueOfCompare for DateRange {
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::DateRangeValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX => None,
                Operator::GT => Some(self < b),
                Operator::GTE => Some(self <= b),
                Operator::LT => Some(self > b),
                Operator::LTE => Some(self >= b),
                Operator::IN | Operator::NotIn => None,
            };
        }
        return None;
    }
}

impl ValueOfCompare for Vec<ValueOf> {
    // bは外からくるデータ。selfのいずれかのものがbに合致すればOK
    fn compare_value_of(&self, operator: Operator, b: &ValueOf) -> Option<bool> {
        if let ValueOf::ArrayValue(b) = b {
            return match operator {
                Operator::Null => None,
                Operator::EQUAL => Some(self == b),
                Operator::NotEqual => Some(self != b),
                Operator::REGEX => None,
                Operator::GT => None,
                Operator::LT => None,
                Operator::GTE => None,
                Operator::LTE => None,
                Operator::IN => {
                    let mut into_iter = false;
                    for aa in self.iter() {
                        into_iter = true;

                        let mut has_aa = false;
                        for bb in b.iter() {
                            if let Some(r) = aa.compare_value_of(Operator::EQUAL, bb) {
                                if r == true {
                                    has_aa = true
                                }
                            }
                        }

                        if has_aa == false {
                            return Some(false);
                        }
                    }
                    if into_iter == true {
                        return Some(true);
                    } else {
                        return Some(false);
                    }
                } // TODO:aa,
                Operator::NotIn => {
                    let mut into_iter = false;
                    let mut has_aa = false;
                    for aa in self.iter() {
                        into_iter = true;

                        for bb in b.iter() {
                            if let Some(r) = aa.compare_value_of(Operator::EQUAL, bb) {
                                if r == true {
                                    has_aa = true
                                }
                            }
                        }
                    }
                    if into_iter == true {
                        return Some(!has_aa);
                    } else {
                        return Some(true);
                    }
                } // TODO:aa
            };
        }

        return match operator {
            Operator::Null => None,
            Operator::EQUAL => None,
            Operator::NotEqual => None,
            Operator::REGEX => None,
            Operator::GT => None,
            Operator::LT => None,
            Operator::GTE => None,
            Operator::LTE => None,
            Operator::IN => {
                for aa in self.iter() {
                    if aa.compare_value_of(Operator::EQUAL, b) == Some(true) {
                        return Some(true);
                    }
                }

                return Some(false);
            } // TODO:aa,
            Operator::NotIn => {
                let mut has = false;
                for aa in self.iter() {
                    if aa.compare_value_of(Operator::EQUAL, b) == Some(true) {
                        has = true
                    }
                }

                return Some(!has);
            } // TODO:aa
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bool_compare_value_of() {
        let value_of = false;

        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BoolValue(false)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BoolValue(true)),
            Some(false)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::StringValue(String::from(""))),
            None
        );
        assert_eq!(
            value_of.compare_value_of(Operator::LT, &ValueOf::StringValue(String::from(""))),
            None
        );
    }

    #[test]
    fn string_compare_value_of() {
        let value_of = String::from("AAA");

        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::StringValue(String::from("AAA"))),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::StringValue(String::from("BBB"))),
            Some(false)
        );

        let value_of = String::from(r"\d");
        assert_eq!(
            value_of.compare_value_of(Operator::REGEX, &ValueOf::StringValue(String::from("000"))),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::REGEX, &ValueOf::StringValue(String::from("aaa"))),
            Some(false)
        );

        // None
        assert_eq!(
            value_of.compare_value_of(Operator::GT, &ValueOf::StringValue(String::from("aaa"))),
            None
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BoolValue(true)),
            None
        );
    }

    #[test]
    fn byte_compare_value_of() {
        let value_of: Vec<u8> = br#"AAA"#.to_vec();
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BytesValue(br#"AAA"#.to_vec())),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BytesValue(br#"BBB"#.to_vec())),
            Some(false)
        );

        // None
        assert_eq!(
            value_of.compare_value_of(Operator::GT, &ValueOf::BytesValue(br#"BBB"#.to_vec())),
            None
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BoolValue(true)),
            None
        );
    }

    /*
       #[test]
       fn uuid_compare_value_of() {
           let value_of = uuid::Uuid::new_v4();
           assert_eq!(
               value_of.compare_value_of(Operator::EQUAL, &ValueOf::UuidValue(value_of)),
               Some(true)
           );
           assert_eq!(
               value_of.compare_value_of(Operator::EQUAL, &ValueOf::UuidValue(uuid::Uuid::new_v4())),
               Some(false)
           );

           // None
           assert_eq!(
               value_of.compare_value_of(Operator::GT, &ValueOf::UuidValue(uuid::Uuid::new_v4())),
               None
           );
           assert_eq!(
               value_of.compare_value_of(Operator::EQUAL, &ValueOf::BoolValue(true)),
               None
           );
       }
    */
    #[test]
    fn int_compare_value_of() {
        let value_of = 10;

        // EQUAL
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::IntValue(10)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::IntValue(9)),
            Some(false)
        );

        // GT
        assert_eq!(
            value_of.compare_value_of(Operator::GT, &ValueOf::IntValue(11)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::GT, &ValueOf::IntValue(10)),
            Some(false)
        );
        // GTE
        assert_eq!(
            value_of.compare_value_of(Operator::GTE, &ValueOf::IntValue(11)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::GTE, &ValueOf::IntValue(10)),
            Some(true)
        );

        // LT
        assert_eq!(
            value_of.compare_value_of(Operator::LT, &ValueOf::IntValue(9)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::LT, &ValueOf::IntValue(10)),
            Some(false)
        );
        // LTE
        assert_eq!(
            value_of.compare_value_of(Operator::LTE, &ValueOf::IntValue(9)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::LTE, &ValueOf::IntValue(10)),
            Some(true)
        );

        // None
        assert_eq!(
            value_of.compare_value_of(Operator::IN, &ValueOf::IntValue(10)),
            None
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BoolValue(true)),
            None
        );
    }

    #[test]
    fn float_compare_value_of() {
        let value_of = 10.0;

        // EQUAL
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::DoubleValue(10.0)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::DoubleValue(9.0)),
            Some(false)
        );

        // GT
        assert_eq!(
            value_of.compare_value_of(Operator::GT, &ValueOf::DoubleValue(11.0)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::GT, &ValueOf::DoubleValue(10.0)),
            Some(false)
        );
        // GTE
        assert_eq!(
            value_of.compare_value_of(Operator::GTE, &ValueOf::DoubleValue(11.0)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::GTE, &ValueOf::DoubleValue(10.0)),
            Some(true)
        );

        // LT
        assert_eq!(
            value_of.compare_value_of(Operator::LT, &ValueOf::DoubleValue(9.0)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::LT, &ValueOf::DoubleValue(10.0)),
            Some(false)
        );
        // LTE
        assert_eq!(
            value_of.compare_value_of(Operator::LTE, &ValueOf::DoubleValue(9.0)),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::LTE, &ValueOf::DoubleValue(10.0)),
            Some(true)
        );

        // None
        assert_eq!(
            value_of.compare_value_of(Operator::IN, &ValueOf::DoubleValue(10.0)),
            None
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BoolValue(true)),
            None
        );
    }

    #[test]
    fn datetime_compare_value_of() {
        let value_of = DateTime::new(2000, 1, 1, 0, 0, 0);

        // EQUAL
        assert_eq!(
            value_of.compare_value_of(
                Operator::EQUAL,
                &ValueOf::DateTimeValue(DateTime::new(2000, 1, 1, 0, 0, 0))
            ),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(
                Operator::EQUAL,
                &ValueOf::DateTimeValue(DateTime::new(2000, 1, 1, 1, 0, 0))
            ),
            Some(false)
        );

        // GT
        assert_eq!(
            value_of.compare_value_of(
                Operator::GT,
                &ValueOf::DateTimeValue(DateTime::new(2000, 1, 1, 0, 0, 1))
            ),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(
                Operator::GT,
                &ValueOf::DateTimeValue(DateTime::new(2000, 1, 1, 0, 0, 0))
            ),
            Some(false)
        );
        // GTE
        assert_eq!(
            value_of.compare_value_of(
                Operator::GTE,
                &ValueOf::DateTimeValue(DateTime::new(2000, 1, 1, 0, 0, 1))
            ),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(
                Operator::GTE,
                &ValueOf::DateTimeValue(DateTime::new(2000, 1, 1, 0, 0, 0))
            ),
            Some(true)
        );

        // LT
        assert_eq!(
            value_of.compare_value_of(
                Operator::LT,
                &ValueOf::DateTimeValue(DateTime::new(1999, 12, 31, 23, 59, 59))
            ),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(
                Operator::LT,
                &ValueOf::DateTimeValue(DateTime::new(2000, 1, 1, 0, 0, 0))
            ),
            Some(false)
        );
        // LTE
        assert_eq!(
            value_of.compare_value_of(
                Operator::LTE,
                &ValueOf::DateTimeValue(DateTime::new(1999, 12, 31, 23, 59, 59))
            ),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(
                Operator::LTE,
                &ValueOf::DateTimeValue(DateTime::new(2000, 1, 1, 0, 0, 0))
            ),
            Some(true)
        );

        // None
        assert_eq!(
            value_of.compare_value_of(
                Operator::IN,
                &ValueOf::DateTimeValue(DateTime::new(2000, 1, 1, 0, 0, 0))
            ),
            None
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BoolValue(true)),
            None
        );
    }

    #[test]
    fn date_compare_value_of() {
        let value_of = Date::new(2000, 1, 1);

        // EQUAL
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::DateValue(Date::new(2000, 1, 1))),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::DateValue(Date::new(2000, 1, 2))),
            Some(false)
        );

        // GT
        assert_eq!(
            value_of.compare_value_of(Operator::GT, &ValueOf::DateValue(Date::new(2000, 1, 2))),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::GT, &ValueOf::DateValue(Date::new(2000, 1, 1))),
            Some(false)
        );
        // GTE
        assert_eq!(
            value_of.compare_value_of(Operator::GTE, &ValueOf::DateValue(Date::new(2000, 1, 2))),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::GTE, &ValueOf::DateValue(Date::new(2000, 1, 1))),
            Some(true)
        );

        // LT
        assert_eq!(
            value_of.compare_value_of(Operator::LT, &ValueOf::DateValue(Date::new(1999, 12, 31))),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::LT, &ValueOf::DateValue(Date::new(2000, 1, 1))),
            Some(false)
        );
        // LTE
        assert_eq!(
            value_of.compare_value_of(Operator::LTE, &ValueOf::DateValue(Date::new(1999, 12, 31))),
            Some(true)
        );
        assert_eq!(
            value_of.compare_value_of(Operator::LTE, &ValueOf::DateValue(Date::new(2000, 1, 1))),
            Some(true)
        );

        // None
        assert_eq!(
            value_of.compare_value_of(Operator::IN, &ValueOf::DateValue(Date::new(2000, 1, 1))),
            None
        );
        assert_eq!(
            value_of.compare_value_of(Operator::EQUAL, &ValueOf::BoolValue(true)),
            None
        );
    }

    /*
    #[test]
    fn array_uuid_compare_value_of() {
        let value_of = vec![
            ValueOf::UuidValue(uuid::Uuid::new_v4()),
            ValueOf::UuidValue(uuid::Uuid::new_v4()),
            ValueOf::UuidValue(uuid::Uuid::new_v4()),
        ];

        // Array Uuid
        assert_eq!(
            vec![value_of[1].clone()]
                .clone()
                .compare_value_of(Operator::IN, &ValueOf::ArrayValue(value_of.clone())),
            Some(true)
        );

        assert_eq!(
            vec![ValueOf::UuidValue(uuid::Uuid::new_v4())]
                .compare_value_of(Operator::IN, &ValueOf::ArrayValue(value_of.clone())),
            Some(false)
        );
    }
     */
}
