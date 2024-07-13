// use super::super::pb::basic::{ArrayValue as pb_ArrayValue, Value as pb_Value};

use std::ops;

use crate::pb::typ_p as pb_typ_p;
use pb_typ_p::ArrayValue as pb_ArrayValue;

use crate::typ_p::Value;

use serde::{Deserialize, Serialize};

use derive_pb::Pb;
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Pb)]
#[pb(pb_name = "pb_ArrayValue")]
pub struct ArrayValue {
    pub values: Vec<Value>,
}

impl ArrayValue {
    pub fn new(values: Vec<Value>) -> Self {
        return Self { values };
    }
}

impl ops::Add<ArrayValue> for ArrayValue {
    type Output = ArrayValue;

    fn add(self, _a: ArrayValue) -> ArrayValue {
        let mut values = Vec::with_capacity(self.values.len() + _a.values.len());

        values.extend(self.values);
        values.extend(_a.values);

        return ArrayValue { values: values };
    }
}

impl ops::Add<&ArrayValue> for &ArrayValue {
    type Output = ArrayValue;

    fn add(self, _a: &ArrayValue) -> ArrayValue {
        let mut values = Vec::with_capacity(self.values.len() + _a.values.len());

        values.extend(self.values.clone());
        values.extend(_a.values.clone());

        return ArrayValue { values: values };
    }
}

impl ops::Add<&pb_ArrayValue> for &pb_ArrayValue {
    type Output = pb_ArrayValue;

    fn add(self, _a: &pb_ArrayValue) -> pb_ArrayValue {
        let mut values = Vec::with_capacity(self.values.len() + _a.values.len());

        values.extend(self.values.clone());
        values.extend(_a.values.clone());

        return pb_ArrayValue { values: values };
    }
}

impl ops::Add<pb_ArrayValue> for pb_ArrayValue {
    type Output = pb_ArrayValue;

    fn add(self, _a: pb_ArrayValue) -> pb_ArrayValue {
        let mut values = Vec::with_capacity(self.values.len() + _a.values.len());

        values.extend(self.values);
        values.extend(_a.values);

        return pb_ArrayValue { values: values };
    }
}
