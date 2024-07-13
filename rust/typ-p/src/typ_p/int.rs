use crate::typ_p::value_of::ValueOf;
use crate::typ_p::Value;

impl From<i32> for ValueOf {
    fn from(f: i32) -> Self {
        ValueOf::IntValue(f.into())
    }
}

impl From<i32> for Value {
    fn from(f: i32) -> Self {
        return Value::new(ValueOf::IntValue(f.into()));
    }
}

impl From<i64> for ValueOf {
    fn from(f: i64) -> Self {
        ValueOf::IntValue(f.into())
    }
}

impl From<i64> for Value {
    fn from(f: i64) -> Self {
        return Value::new(ValueOf::IntValue(f.into()));
    }
}

impl TryFrom<&ValueOf> for i64 {
    type Error = ();

    fn try_from(value_of: &ValueOf) -> Result<Self, Self::Error> {
        match value_of {
            ValueOf::IntValue(v) => Ok(v.clone()),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Value> for i64 {
    type Error = ();

    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        if let Some(value_of) = &value.value_of {
            return value_of.try_into();
        } else {
            return Err(());
        }
    }
}
