// use derive_pb::Pb;

use crate::pb::typ_p as pb_typ_p;

use pb_typ_p::ArrayValue as pb_ArrayValue;
use pb_typ_p::Value as pb_Value;

use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::typ_p::value_of::ValueOf;

/*
#[derive(Debug, Clone)]
#[pb(
  pb_name = "pb_Value",
  module_path = "super::super::pb",
  trait_path = "super::super::pb"
)]
pub struct Value {
  // #[pb(skip = "true")]
  value_type: i16,
  // #[pb(oneof = "value_of")]
  bool_value: bool,
  // #[pb(oneof = "value_of")]
  string_value: String,
  // #[pb(oneof = "value_of")]
  bytes_value: Vec<u8>,
  // #[pb(oneof = "value_of")]
  uuid_value: Vec<u8>,
  // #[pb(oneof = "value_of")]
  int_value: i64,
  // #[pb(oneof = "value_of")]
  double_value: f64,
  #[pb(
    oneof = "value_of",
    from_pb_func_name = "conv_timestamp_pb_to_datetime",
    to_pb_func_name = "conv_datetime_to_timestamp_pb"
  )]
  timestamp_value: Option<DateTime<Utc>>,
  // #[pb(oneof = "value_of")]
  date_value: Option<Date>,
  // #[pb(oneof = "value_of")]
  date_range_value: Option<DateRange>,
}*/

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
pub struct Value {
    pub value_of: Option<ValueOf>,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(v) = &self.value_of {
            write!(f, "{}", v)?;
        }

        Ok(())
    }
}

/*
pub fn aaa(v: &Any) -> Result<Value, String> {
  if v.is::<bool>() {
    let v = v.downcast_ref::<bool>().unwrap();
    return Ok(Value::new(value::ValueOf::BoolValue(*v)));
  } else if v.is::<String>() {
    let v = v.downcast_ref::<String>().unwrap();
    return Ok(Value::new(value::ValueOf::StringValue(v.clone())));
  } else if v.is::<Vec<u8>>() {
    let v = v.downcast_ref::<Vec<u8>>().unwrap();
    return Ok(Value::new(value::ValueOf::BytesValue(v.clone())));
  } else if v.is::<i64>() {
    let v = v.downcast_ref::<i64>().unwrap();
    return Ok(Value::new(value::ValueOf::IntValue(v.clone())));
  } else if v.is::<f64>() {
    let v = v.downcast_ref::<f64>().unwrap();
    return Ok(Value::new(value::ValueOf::DoubleValue(v.clone())));
  } else if v.is::<uuid::Uuid>() {
    let v = v.downcast_ref::<uuid::Uuid>().unwrap();
    return Ok(Value::new(value::ValueOf::BytesValue(
      v.as_bytes().to_vec(),
    )));
  }

  return Err(String::from("not found"));
}
*/

impl Value {
    pub fn new(value: ValueOf) -> Self {
        return Self {
            value_of: Some(value),
        };
    }

    pub fn as_array_value(&self) -> Option<Vec<Value>> {
        if let Some(ValueOf::ArrayValue(a)) = self.value_of.clone() {
            let mut r = vec![];
            for b in a.into_iter() {
                r.push(b.into());
            }

            return Some(r);
        } else {
            return None;
        }
    }
}

impl<T> From<Option<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Option<T>) -> Self {
        if let Some(value) = value {
            return value.into();
        } else {
            Self { value_of: None }
        }
    }
}

/*
impl<T> From<Value> for Option<T> {
    fn from(value: Value) -> Self {
        if let Some(value) = value.value_of {
            return value.into();
        } else {
            return None;
        }
    }
}
 */

impl From<pb_Value> for Value {
    fn from(s: pb_Value) -> Value {
        let value_of = match s.value_of {
            Some(pb_typ_p::value::ValueOf::BoolValue(a)) => Some(ValueOf::BoolValue(a)),
            Some(pb_typ_p::value::ValueOf::StringValue(a)) => Some(ValueOf::StringValue(a)),
            Some(pb_typ_p::value::ValueOf::BytesValue(a)) => Some(ValueOf::BytesValue(a)),
            Some(pb_typ_p::value::ValueOf::UuidValue(a)) => Some(ValueOf::UuidValue(a.into())),
            Some(pb_typ_p::value::ValueOf::NumberValue(a)) => Some(ValueOf::NumberValue(a.into())),
            // Some(pb_basic::value::ValueOf::DateTimeValue(a)) => {
            //     Some(value::ValueOf::DateTimeValue(a.into()))
            // }
            Some(pb_typ_p::value::ValueOf::DateValue(a)) => Some(ValueOf::DateValue(a.into())),
            // Some(pb_basic::value::ValueOf::DateRangeValue(a)) => {
            //     Some(value::ValueOf::DateRangeValue(a.into()))
            // }
            Some(pb_typ_p::value::ValueOf::ArrayValues(a)) => {
                let mut aaa = vec![];

                // value::ValueOf::ArrayValue
                for b in a.values {
                    let c: Value = b.into();

                    if let Some(d) = c.value_of {
                        aaa.push(d);
                    }
                }
                // let dd = value::ValueOf::ArrayValue(aaa);

                // let ee = ArrayValue { values: aaa };

                let fff = ValueOf::ArrayValue(aaa);

                Some(fff)
            }
            _ => None,
        };

        let a = Self { value_of: value_of };

        return a;
    }
}

impl From<Value> for pb_Value {
    fn from(v: Value) -> pb_Value {
        let value_of = match v.value_of {
            Some(ValueOf::BoolValue(a)) => Some(pb_typ_p::value::ValueOf::BoolValue(a)),
            Some(ValueOf::StringValue(a)) => Some(pb_typ_p::value::ValueOf::StringValue(a)),
            Some(ValueOf::BytesValue(a)) => Some(pb_typ_p::value::ValueOf::BytesValue(a)),
            Some(ValueOf::UuidValue(a)) => Some(pb_typ_p::value::ValueOf::UuidValue(a.into())),
            Some(ValueOf::NumberValue(a)) => Some(pb_typ_p::value::ValueOf::NumberValue(a.into())),
            // Some(value::ValueOf::DateTimeValue(a)) => {
            //     Some(pb_basic::value::ValueOf::DateTimeValue(a.into()))
            // }
            Some(ValueOf::DateValue(a)) => Some(pb_typ_p::value::ValueOf::DateValue(a.into())),
            // Some(value::ValueOf::DateRangeValue(a)) => {
            //     Some(pb_basic::value::ValueOf::DateRangeValue(a.into()))
            // }
            Some(ValueOf::ArrayValue(a)) => {
                let mut b = vec![];
                for aa in a.iter() {
                    let kk = Value {
                        value_of: Some(aa.clone()),
                    };
                    b.push(kk.into())
                }
                let pb = pb_ArrayValue { values: b };
                Some(pb_typ_p::value::ValueOf::ArrayValues(pb))
            }
            _ => None,
        };

        let a = pb_Value { value_of: value_of };

        return a;
    }
}

// use chrono::prelude::{DateTime, Utc};

// Primitive(PartialOrd), Array, Object => AnyValue

/*
impl Value {
  pub fn new<T: ?Sized + Any>(v: &dyn Any) -> Self {
    if let Some(v) = v.downcast_ref::<String>() {
      return Self {
        value_type: 0,
        bool_value: false,
        string_value: v.to_string(),
        bytes_value: vec![],
        uuid_value: vec![],
        int_value: 0,
        double_value: 0.0,
        timestamp_value: None,
        date_value: None,
        date_range_value: None,
      };
    }

    return Self {
      value_type: 0,
      bool_value: false,
      string_value: String::from(""),
      bytes_value: vec![],
      uuid_value: vec![],
      int_value: 0,
      double_value: 0.0,
      timestamp_value: None,
      date_value: None,
      date_range_value: None,
    };
  }
}
*/

// ----- ----- FROM ----- ----- //
/*
impl From<f32> for Value {
  fn from(f: f32) -> Self {
    From::from(f as f64)
  }
}
*/

/*
impl From<f64> for Value {
  fn from(f: f64) -> Self {
    Number::from_f64(f).map_or(Value::Null, Value::Number)
  }
}
*/

impl From<ValueOf> for pb_Value {
    fn from(v: ValueOf) -> Self {
        let vv: Value = Value::new(v);

        return vv.into();
    }
}

impl<'a, T: Clone + Into<ValueOf>> From<&'a [T]> for ValueOf {
    fn from(f: &'a [T]) -> Self {
        let av = f.iter().cloned().map(Into::into).collect();
        ValueOf::ArrayValue(av)
    }
}

impl From<()> for ValueOf {
    fn from((): ()) -> Self {
        ValueOf::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn it_works() {
        let uuids = vec![uuid::Uuid::new_v4(), uuid::Uuid::new_v4()];

        let value: ValueOf = uuids.into();
        println!("{:?}", value);
        assert_eq!(2 + 2, 4);
    }
     */
}
