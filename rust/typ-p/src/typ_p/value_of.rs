use crate::typ_p::Value;
use std::cmp::Ordering;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::typ_p::Date;
use crate::typ_p::DateRange;
use crate::typ_p::DateTime;
use crate::typ_p::Number;
use crate::typ_p::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ValueOf {
    Null,
    BoolValue(bool),
    StringValue(String),
    BytesValue(Vec<u8>),
    UuidValue(Uuid),
    IntValue(i64),
    DoubleValue(f64),
    NumberValue(Number),
    DateTimeValue(DateTime),
    DateValue(Date),
    DateRangeValue(DateRange),
    ArrayValue(Vec<ValueOf>),
    Object(HashMap<String, ValueOf>),
}

impl From<ValueOf> for Value {
    fn from(a: ValueOf) -> Self {
        return Value::new(a);
    }
}

impl PartialOrd for ValueOf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (ValueOf::BoolValue(a), ValueOf::BoolValue(b)) => a.partial_cmp(b),
            (ValueOf::StringValue(a), ValueOf::StringValue(b)) => a.partial_cmp(b),
            (ValueOf::BytesValue(a), ValueOf::BytesValue(b)) => a.partial_cmp(b),
            (ValueOf::UuidValue(a), ValueOf::UuidValue(b)) => a.partial_cmp(b),
            (ValueOf::IntValue(a), ValueOf::IntValue(b)) => a.partial_cmp(b),
            (ValueOf::DoubleValue(a), ValueOf::DoubleValue(b)) => a.partial_cmp(b),
            (ValueOf::NumberValue(a), ValueOf::NumberValue(b)) => a.partial_cmp(b),
            (ValueOf::DateTimeValue(a), ValueOf::DateTimeValue(b)) => a.partial_cmp(b),
            (ValueOf::DateValue(a), ValueOf::DateValue(b)) => a.partial_cmp(b),
            (ValueOf::DateRangeValue(a), ValueOf::DateRangeValue(b)) => a.partial_cmp(b),
            (ValueOf::ArrayValue(a), ValueOf::ArrayValue(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

impl std::fmt::Display for ValueOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueOf::Null => write!(f, ""),
            ValueOf::BoolValue(a) => write!(f, "{}", a),
            ValueOf::StringValue(a) => write!(f, "{}", a),
            ValueOf::BytesValue(a) => write!(f, "{:?}", a),
            ValueOf::UuidValue(a) => write!(f, "{}", a),
            ValueOf::IntValue(a) => write!(f, "{}", a),
            ValueOf::DoubleValue(a) => write!(f, "{}", a),
            ValueOf::NumberValue(a) => write!(f, "{}", a),
            ValueOf::DateTimeValue(a) => write!(f, "{}", a),
            ValueOf::DateValue(a) => write!(f, "{}", a),
            ValueOf::DateRangeValue(a) => write!(f, "{}", a),
            // ValueOf::ArrayValue(a) => write!(f, "{}", a),
            // ValueOf::Object(a) => write!(f, "{}", a),
            _ => write!(f, ""),
        }
    }
}

impl ValueOf {
    pub fn as_bool(&self) -> Option<bool> {
        if let ValueOf::BoolValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_string(&self) -> Option<String> {
        if let ValueOf::StringValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_bytes(&self) -> Option<Vec<u8>> {
        if let ValueOf::BytesValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_uuid(&self) -> Option<Uuid> {
        if let ValueOf::UuidValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if let ValueOf::IntValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        if let ValueOf::DoubleValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_number(&self) -> Option<Number> {
        if let ValueOf::NumberValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_date_time(&self) -> Option<DateTime> {
        if let ValueOf::DateTimeValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_date(&self) -> Option<Date> {
        if let ValueOf::DateValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_date_range(&self) -> Option<DateRange> {
        if let ValueOf::DateRangeValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_array(&self) -> Option<Vec<ValueOf>> {
        if let ValueOf::ArrayValue(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_object(&self) -> Option<HashMap<String, ValueOf>> {
        if let ValueOf::Object(a) = self {
            return Some(a.clone());
        } else {
            return None;
        }
    }
}
