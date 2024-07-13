use crate::typ_p::value_of::ValueOf;
use crate::typ_p::Value;

impl From<bool> for ValueOf {
    fn from(f: bool) -> Self {
        ValueOf::BoolValue(f)
    }
}

impl From<bool> for Value {
    fn from(f: bool) -> Self {
        Value::new(f.into())
    }
}

impl TryFrom<ValueOf> for bool {
    type Error = ();

    fn try_from(value_of: ValueOf) -> Result<Self, Self::Error> {
        match value_of {
            ValueOf::BoolValue(v) => Ok(v),
            _ => Err(()),
        }
    }
}

impl TryFrom<Value> for bool {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        if let Some(value_of) = value.value_of {
            return value_of.try_into();
        } else {
            return Err(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_1() {
        let a = true;

        let v: ValueOf = a.into();

        assert!(v == ValueOf::BoolValue(true));
    }

    #[test]
    fn it_works_2() {
        let a = ValueOf::BoolValue(true);

        let v: bool = a.try_into().unwrap();

        assert!(v == true);
    }
}
