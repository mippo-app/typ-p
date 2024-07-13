from typ_p import date_delta_pb2 as _date_delta_pb2
from typ_p import date_range_pb2 as _date_range_pb2
from typ_p import date_time_pb2 as _date_time_pb2
from typ_p import date_pb2 as _date_pb2
from typ_p import number_pb2 as _number_pb2
from typ_p import uuid_pb2 as _uuid_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf.internal import enum_type_wrapper as _enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class ValueType(int, metaclass=_enum_type_wrapper.EnumTypeWrapper):
    __slots__ = []
    VALUE_TYPE_UNKNOWN: _ClassVar[ValueType]
    VALUE_TYPE_BOOL: _ClassVar[ValueType]
    VALUE_TYPE_STRING: _ClassVar[ValueType]
    VALUE_TYPE_BYTES: _ClassVar[ValueType]
    VALUE_TYPE_UUID: _ClassVar[ValueType]
    VALUE_TYPE_NUMBER: _ClassVar[ValueType]
    VALUE_TYPE_DATETIME: _ClassVar[ValueType]
    VALUE_TYPE_DATE: _ClassVar[ValueType]
    VALUE_TYPE_DATE_RANGE: _ClassVar[ValueType]
    VALUE_TYPE_ARRAY_VALUES: _ClassVar[ValueType]
VALUE_TYPE_UNKNOWN: ValueType
VALUE_TYPE_BOOL: ValueType
VALUE_TYPE_STRING: ValueType
VALUE_TYPE_BYTES: ValueType
VALUE_TYPE_UUID: ValueType
VALUE_TYPE_NUMBER: ValueType
VALUE_TYPE_DATETIME: ValueType
VALUE_TYPE_DATE: ValueType
VALUE_TYPE_DATE_RANGE: ValueType
VALUE_TYPE_ARRAY_VALUES: ValueType

class value(_message.Message):
    __slots__ = ["bool_value", "string_value", "bytes_value", "uuid_value", "number_value", "date_time_value", "date_value", "date_range_value", "array_values"]
    BOOL_VALUE_FIELD_NUMBER: _ClassVar[int]
    STRING_VALUE_FIELD_NUMBER: _ClassVar[int]
    BYTES_VALUE_FIELD_NUMBER: _ClassVar[int]
    UUID_VALUE_FIELD_NUMBER: _ClassVar[int]
    NUMBER_VALUE_FIELD_NUMBER: _ClassVar[int]
    DATE_TIME_VALUE_FIELD_NUMBER: _ClassVar[int]
    DATE_VALUE_FIELD_NUMBER: _ClassVar[int]
    DATE_RANGE_VALUE_FIELD_NUMBER: _ClassVar[int]
    ARRAY_VALUES_FIELD_NUMBER: _ClassVar[int]
    bool_value: bool
    string_value: str
    bytes_value: bytes
    uuid_value: _uuid_pb2.uuid
    number_value: _number_pb2.number
    date_time_value: _date_time_pb2.date_time
    date_value: _date_pb2.date
    date_range_value: _date_range_pb2.date_range
    array_values: array_value
    def __init__(self, bool_value: bool = ..., string_value: _Optional[str] = ..., bytes_value: _Optional[bytes] = ..., uuid_value: _Optional[_Union[_uuid_pb2.uuid, _Mapping]] = ..., number_value: _Optional[_Union[_number_pb2.number, _Mapping]] = ..., date_time_value: _Optional[_Union[_date_time_pb2.date_time, _Mapping]] = ..., date_value: _Optional[_Union[_date_pb2.date, _Mapping]] = ..., date_range_value: _Optional[_Union[_date_range_pb2.date_range, _Mapping]] = ..., array_values: _Optional[_Union[array_value, _Mapping]] = ...) -> None: ...

class array_value(_message.Message):
    __slots__ = ["values"]
    VALUES_FIELD_NUMBER: _ClassVar[int]
    values: _containers.RepeatedCompositeFieldContainer[value]
    def __init__(self, values: _Optional[_Iterable[_Union[value, _Mapping]]] = ...) -> None: ...
