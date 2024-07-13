use std::collections::HashMap;

use gcloud_sdk::google::firestore::v1::{MapValue, Value as f_Value};

use gcloud_sdk::google::firestore::v1::value::ValueType;

use gcloud_sdk::google::firestore::v1::structured_query::field_filter::Operator as f_Operator;

use super::operator::Operator;
use super::value_of::ValueOf;
use super::{Date, Value};

impl From<ValueType> for ValueOf {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::NullValue(v) => v.into(),
            ValueType::BooleanValue(v) => v.into(),
            // ValueType::DoubleValue(v) => v.into(),
            ValueType::IntegerValue(v) => v.into(),
            ValueType::StringValue(v) => v.into(),
            // ValueType::ArrayValue(v) => v.into(),
            // ValueType::BytesValue(v) => v.into(),
            // ValueType::GeoPointValue(v) => v.into(),
            ValueType::MapValue(v) => {
                if let Some(t) = v.fields.get("type") {
                    if let Some(ValueType::StringValue(tt)) = t.value_type.clone() {
                        //
                        if tt == String::from("date") {
                            if let Some(vv) = v.fields.get("v") {
                                if let Some(ValueType::StringValue(vvv)) = vv.value_type.clone() {
                                    let d = Date::new_from_string(vvv).unwrap();

                                    return d.into();
                                }
                            }
                        }
                    }
                }

                ValueOf::Null
            }
            ValueType::ReferenceValue(v) => v.into(),
            // ValueType::TimestampValue(v) => v.into(),
            _ => todo!(),
        }
    }
}

pub fn date_to_map(date: Date) -> MapValue {
    let mut r = HashMap::new();
    let v = date.to_string();

    r.insert(
        String::from("type"),
        f_Value {
            value_type: Some(ValueType::StringValue(String::from("date"))),
        },
    );

    r.insert(
        String::from("v"),
        f_Value {
            value_type: Some(ValueType::StringValue(v)),
        },
    );

    return MapValue { fields: r };
}

impl Into<ValueType> for ValueOf {
    fn into(self) -> ValueType {
        match self {
            ValueOf::Null => ValueType::NullValue(0),
            ValueOf::BoolValue(v) => ValueType::BooleanValue(v),
            ValueOf::StringValue(v) => ValueType::StringValue(v),
            ValueOf::BytesValue(v) => ValueType::BytesValue(v),
            ValueOf::UuidValue(_) => todo!(),
            ValueOf::IntValue(v) => ValueType::IntegerValue(v),
            ValueOf::DoubleValue(v) => ValueType::DoubleValue(v),
            ValueOf::NumberValue(_) => todo!(),
            ValueOf::DateTimeValue(_) => todo!(),
            ValueOf::DateValue(v) => ValueType::MapValue(date_to_map(v)),
            ValueOf::DateRangeValue(_) => todo!(),
            ValueOf::ArrayValue(_) => todo!(),
            ValueOf::Object(_) => todo!(),
        }
    }
}

/*
impl Into<ValueType> for Value {
    fn into(self) -> ValueType {
        return self.value_of.unwrap().into();
    }
}

impl From<ValueType> for Value {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::NullValue(v) => ValueOf::from(v).into(),
            ValueType::BooleanValue(v) => ValueOf::from(v).into(),
            // ValueType::DoubleValue(v) => v.into(),
            // ValueType::IntegerValue(v) => v.into(),
            ValueType::StringValue(v) => ValueOf::from(v).into(),
            // ValueType::ArrayValue(v) => v.into(),
            // ValueType::BytesValue(v) => v.into(),
            // ValueType::GeoPointValue(v) => v.into(),
            // ValueType::MapValue(v) => v.into(),
            ValueType::ReferenceValue(v) => ValueOf::from(v).into(),
            // ValueType::TimestampValue(v) => v.into(),
            _ => Value {
                value_of: Some(ValueOf::Null),
            },
        }
    }
}
 */

impl From<f_Value> for Value {
    fn from(value: f_Value) -> Self {
        if let Some(vt) = value.value_type {
            let value_of = vt.into();

            return Value {
                value_of: Some(value_of),
            };
        } else {
            return Value { value_of: None };
        }
    }
}

impl Into<f_Value> for Value {
    fn into(self) -> f_Value {
        if let Some(value_of) = self.value_of {
            let vt = value_of.into();

            return f_Value {
                value_type: Some(vt),
            };
        } else {
            return f_Value { value_type: None };
        }
    }
}

impl From<f_Operator> for Operator {
    fn from(value: f_Operator) -> Self {
        match value {
            f_Operator::Unspecified => Self::Null,
            f_Operator::LessThan => Self::LT,
            f_Operator::LessThanOrEqual => Self::LTE,
            f_Operator::GreaterThan => Self::GT,
            f_Operator::GreaterThanOrEqual => Self::GTE,
            f_Operator::Equal => Self::EQUAL,
            f_Operator::NotEqual => Self::NotEqual,
            // f_Operator::ArrayContains => todo!(),
            f_Operator::In => Self::IN,
            // f_Operator::ArrayContainsAny => todo!(),
            f_Operator::NotIn => Self::NotIn,
            _ => todo!(),
        }
    }
}

impl Into<f_Operator> for Operator {
    fn into(self) -> f_Operator {
        match self {
            Operator::Null => f_Operator::Unspecified,
            Operator::EQUAL => f_Operator::Equal,
            Operator::NotEqual => f_Operator::NotEqual,
            Operator::REGEX => todo!(),
            Operator::GT => f_Operator::GreaterThan,
            Operator::GTE => f_Operator::GreaterThanOrEqual,
            Operator::LT => f_Operator::LessThan,
            Operator::LTE => f_Operator::LessThanOrEqual,
            Operator::IN => f_Operator::In,
            Operator::NotIn => f_Operator::NotEqual,
        }
    }
}
