use std::fmt::Display;

use derive_pb::Pb;

use super::{super::pb::typ_p::Uuid as pb_Uuid, Value};

use serde::{Deserialize, Serialize};

use crate::typ_p::value_of::ValueOf;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Pb, Hash, PartialOrd, Ord)]
#[pb(
    pb_name = "pb_Uuid",
    module_path = "super::super::super::pb",
    trait_path = "super::super::super::pb"
)]
pub struct Uuid {
    pub uuid_value: Vec<u8>,
}

impl Uuid {
    pub fn new_v4() -> Self {
        let id = uuid::Uuid::new_v4();
        let id_byte = id.as_bytes();

        return Self {
            uuid_value: id_byte.to_vec(),
        };
    }

    pub fn new_nil() -> Self {
        return Self { uuid_value: vec![] };
    }

    pub fn is_nil(&self) -> bool {
        if self.uuid_value.len() > 0 {
            return true;
        } else {
            return false;
        }
    }
}

impl Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let u: uuid::Uuid = self.clone().into();

        write!(f, "{}", u)?;

        Ok(())
    }
}

impl From<Vec<u8>> for Uuid {
    fn from(v: Vec<u8>) -> Self {
        return Self {
            uuid_value: v.to_vec(),
        };
    }
}

impl From<Uuid> for Vec<u8> {
    fn from(v: Uuid) -> Self {
        return v.uuid_value;
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(v: uuid::Uuid) -> Self {
        return Self {
            uuid_value: v.as_bytes().to_vec(),
        };
    }
}

impl From<Uuid> for uuid::Uuid {
    fn from(v: Uuid) -> Self {
        return uuid::Uuid::from_slice(&v.uuid_value).unwrap();
    }
}

impl From<Uuid> for ValueOf {
    fn from(v: Uuid) -> Self {
        return ValueOf::UuidValue(v.into());
    }
}

impl From<Uuid> for Value {
    fn from(v: Uuid) -> Self {
        return Value::new(v.into());
    }
}

impl From<uuid::Uuid> for ValueOf {
    fn from(v: uuid::Uuid) -> Self {
        return ValueOf::UuidValue(v.into());
    }
}

impl From<uuid::Uuid> for Value {
    fn from(v: uuid::Uuid) -> Self {
        return Value::new(v.into());
    }
}
