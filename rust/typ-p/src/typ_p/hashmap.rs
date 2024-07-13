use std::collections::HashMap;

use crate::typ_p::value_of::ValueOf;
use crate::typ_p::Value;

impl From<HashMap<String, ValueOf>> for ValueOf {
    fn from(f: HashMap<String, ValueOf>) -> Self {
        ValueOf::Object(f)
    }
}
