from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class number(_message.Message):
    __slots__ = ("int32_value", "int64_value", "float_value", "double_value")
    INT32_VALUE_FIELD_NUMBER: _ClassVar[int]
    INT64_VALUE_FIELD_NUMBER: _ClassVar[int]
    FLOAT_VALUE_FIELD_NUMBER: _ClassVar[int]
    DOUBLE_VALUE_FIELD_NUMBER: _ClassVar[int]
    int32_value: int
    int64_value: int
    float_value: float
    double_value: float
    def __init__(self, int32_value: _Optional[int] = ..., int64_value: _Optional[int] = ..., float_value: _Optional[float] = ..., double_value: _Optional[float] = ...) -> None: ...
