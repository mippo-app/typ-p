from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Optional as _Optional

DESCRIPTOR: _descriptor.FileDescriptor

class uuid(_message.Message):
    __slots__ = ["uuid_value"]
    UUID_VALUE_FIELD_NUMBER: _ClassVar[int]
    uuid_value: bytes
    def __init__(self, uuid_value: _Optional[bytes] = ...) -> None: ...
