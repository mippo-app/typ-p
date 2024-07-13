use derive_pb::Pb;
use serde::{Deserialize, Serialize};

use crate::pb::typ_p::number::ValueOf as pb_ValueOf;
use crate::pb::typ_p::Number as pb_Number;

use std::fmt::{self, Display};

use crate::typ_p::value_of::ValueOf as a_ValueOf;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Pb, PartialOrd)]
#[pb(pb_name = "pb_Number")]
pub struct Number {
    pub value_of: Option<ValueOf>,
}

impl Number {
    /*
    pub fn as_double(&self) -> Option<f64> {
        if let ValueOf::DoubleValue(a) = self.clone().value_of? {
            return Some(a.clone());
        } else {
            return None;
        }
    }
    */

    pub fn as_int32(&self) -> Option<i32> {
        if let ValueOf::Int32Value(a) = self.clone().value_of? {
            return Some(a.clone());
        } else {
            return None;
        }
    }

    pub fn as_float(&self) -> Option<f32> {
        if let ValueOf::FloatValue(a) = self.clone().value_of? {
            return Some(a.clone());
        } else {
            return None;
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        // `x`と`y`のみが明示的になるようにカスタマイズ
        if let Some(value) = &self.value_of {
            write!(f, "{}", value)?
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Pb, PartialOrd)]
#[pb(pb_name = "pb_ValueOf")]
pub enum ValueOf {
    Int32Value(i32),
    Int64Value(i64),
    FloatValue(f32),
    DoubleValue(f64),
}

impl Display for ValueOf {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            ValueOf::DoubleValue(v) => write!(f, "{}", v),
            ValueOf::FloatValue(v) => write!(f, "{}", v),
            ValueOf::Int32Value(v) => write!(f, "{}", v),
            ValueOf::Int64Value(v) => write!(f, "{}", v),
        }
    }
}

impl From<Number> for a_ValueOf {
    fn from(f: Number) -> Self {
        a_ValueOf::NumberValue(f)
    }
}

impl From<i32> for Number {
    fn from(f: i32) -> Self {
        return Number {
            value_of: Some(ValueOf::Int32Value(f.into())),
        };
    }
}

impl From<f32> for Number {
    fn from(f: f32) -> Self {
        return Number {
            value_of: Some(ValueOf::FloatValue(f.into())),
        };
    }
}
/*
impl From<f64> for Number {
    fn from(f: f64) -> Self {
        return Number {
            value_of: Some(ValueOf::FloatValue(f.try_into().unwrap())),
        };
    }
}
 */
