from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class date_delta(_message.Message):
    __slots__ = ["year", "month", "week", "day"]
    YEAR_FIELD_NUMBER: _ClassVar[int]
    MONTH_FIELD_NUMBER: _ClassVar[int]
    WEEK_FIELD_NUMBER: _ClassVar[int]
    DAY_FIELD_NUMBER: _ClassVar[int]
    year: int
    month: int
    week: int
    day: int
    def __init__(self, year: _Optional[int] = ..., month: _Optional[int] = ..., week: _Optional[int] = ..., day: _Optional[int] = ...) -> None: ...
