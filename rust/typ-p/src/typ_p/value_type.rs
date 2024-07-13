use derive_pb::Pb;

use crate::pb::typ_p::ValueType as pb_ValueType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Pb)]
#[pb(pb_name = "pb_ValueType")]
#[repr(i32)]
pub enum ValueType {
    Unknown = 0,
    Bool = 1,
    String = 2,
    Bytes = 3,
    Uuid = 4,
    Number = 5,
    Datetime = 6,
    Date = 7,
    DateRange = 8,
    ArrayValues = 10,
}
