from typ_p import date_pb2 as _date_pb2
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class date_range(_message.Message):
    __slots__ = ("date_from", "date_to")
    DATE_FROM_FIELD_NUMBER: _ClassVar[int]
    DATE_TO_FIELD_NUMBER: _ClassVar[int]
    date_from: _date_pb2.Date
    date_to: _date_pb2.Date
    def __init__(self, date_from: _Optional[_Union[_date_pb2.Date, _Mapping]] = ..., date_to: _Optional[_Union[_date_pb2.Date, _Mapping]] = ...) -> None: ...
