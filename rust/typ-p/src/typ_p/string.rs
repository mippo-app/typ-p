use std::borrow::Cow;

use crate::typ_p::value_of::ValueOf;
use crate::typ_p::Value;

impl From<String> for ValueOf {
    fn from(f: String) -> Self {
        ValueOf::StringValue(f)
    }
}

impl From<String> for Value {
    fn from(f: String) -> Self {
        Value::new(f.into())
    }
}

impl TryFrom<ValueOf> for String {
    type Error = ();

    fn try_from(value_of: ValueOf) -> Result<Self, Self::Error> {
        match value_of {
            ValueOf::StringValue(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl TryFrom<&ValueOf> for String {
    type Error = ();

    fn try_from(value_of: &ValueOf) -> Result<Self, Self::Error> {
        match value_of {
            ValueOf::StringValue(v) => Ok(v.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for String {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Some(value_of) = value.value_of {
            return value_of.try_into();
        } else {
            return Err(());
        }
    }
}

impl TryFrom<&Value> for String {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(value_of) = &value.value_of {
            return value_of.try_into();
        } else {
            return Err(());
        }
    }
}

/********** **********/

impl<'a> From<&'a str> for ValueOf {
    fn from(f: &str) -> Self {
        ValueOf::StringValue(f.to_string())
    }
}

impl<'a> From<Cow<'a, str>> for ValueOf {
    fn from(f: Cow<'a, str>) -> Self {
        ValueOf::StringValue(f.into_owned())
    }
}
