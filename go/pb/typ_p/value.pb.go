// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        (unknown)
// source: typ_p/value.proto

package typ_p

import (
	protoreflect "google.golang.org/protobuf/reflect/protoreflect"
	protoimpl "google.golang.org/protobuf/runtime/protoimpl"
	reflect "reflect"
	sync "sync"
)

const (
	// Verify that this generated code is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(20 - protoimpl.MinVersion)
	// Verify that runtime/protoimpl is sufficiently up-to-date.
	_ = protoimpl.EnforceVersion(protoimpl.MaxVersion - 20)
)

type ValueType int32

const (
	ValueType_VALUE_TYPE_UNKNOWN      ValueType = 0
	ValueType_VALUE_TYPE_BOOL         ValueType = 1
	ValueType_VALUE_TYPE_STRING       ValueType = 2
	ValueType_VALUE_TYPE_BYTES        ValueType = 3
	ValueType_VALUE_TYPE_UUID         ValueType = 4
	ValueType_VALUE_TYPE_NUMBER       ValueType = 5
	ValueType_VALUE_TYPE_DATETIME     ValueType = 6
	ValueType_VALUE_TYPE_DATE         ValueType = 7
	ValueType_VALUE_TYPE_DATE_RANGE   ValueType = 8
	ValueType_VALUE_TYPE_ARRAY_VALUES ValueType = 10
)

// Enum value maps for ValueType.
var (
	ValueType_name = map[int32]string{
		0:  "VALUE_TYPE_UNKNOWN",
		1:  "VALUE_TYPE_BOOL",
		2:  "VALUE_TYPE_STRING",
		3:  "VALUE_TYPE_BYTES",
		4:  "VALUE_TYPE_UUID",
		5:  "VALUE_TYPE_NUMBER",
		6:  "VALUE_TYPE_DATETIME",
		7:  "VALUE_TYPE_DATE",
		8:  "VALUE_TYPE_DATE_RANGE",
		10: "VALUE_TYPE_ARRAY_VALUES",
	}
	ValueType_value = map[string]int32{
		"VALUE_TYPE_UNKNOWN":      0,
		"VALUE_TYPE_BOOL":         1,
		"VALUE_TYPE_STRING":       2,
		"VALUE_TYPE_BYTES":        3,
		"VALUE_TYPE_UUID":         4,
		"VALUE_TYPE_NUMBER":       5,
		"VALUE_TYPE_DATETIME":     6,
		"VALUE_TYPE_DATE":         7,
		"VALUE_TYPE_DATE_RANGE":   8,
		"VALUE_TYPE_ARRAY_VALUES": 10,
	}
)

func (x ValueType) Enum() *ValueType {
	p := new(ValueType)
	*p = x
	return p
}

func (x ValueType) String() string {
	return protoimpl.X.EnumStringOf(x.Descriptor(), protoreflect.EnumNumber(x))
}

func (ValueType) Descriptor() protoreflect.EnumDescriptor {
	return file_typ_p_value_proto_enumTypes[0].Descriptor()
}

func (ValueType) Type() protoreflect.EnumType {
	return &file_typ_p_value_proto_enumTypes[0]
}

func (x ValueType) Number() protoreflect.EnumNumber {
	return protoreflect.EnumNumber(x)
}

// Deprecated: Use ValueType.Descriptor instead.
func (ValueType) EnumDescriptor() ([]byte, []int) {
	return file_typ_p_value_proto_rawDescGZIP(), []int{0}
}

type Value struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	// Types that are assignable to ValueOf:
	//
	//	*Value_BoolValue
	//	*Value_StringValue
	//	*Value_BytesValue
	//	*Value_UuidValue
	//	*Value_NumberValue
	//	*Value_DateTimeValue
	//	*Value_DateValue
	//	*Value_DateRangeValue
	//	*Value_ArrayValues
	ValueOf isValue_ValueOf `protobuf_oneof:"value_of"`
}

func (x *Value) Reset() {
	*x = Value{}
	if protoimpl.UnsafeEnabled {
		mi := &file_typ_p_value_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *Value) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*Value) ProtoMessage() {}

func (x *Value) ProtoReflect() protoreflect.Message {
	mi := &file_typ_p_value_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use Value.ProtoReflect.Descriptor instead.
func (*Value) Descriptor() ([]byte, []int) {
	return file_typ_p_value_proto_rawDescGZIP(), []int{0}
}

func (m *Value) GetValueOf() isValue_ValueOf {
	if m != nil {
		return m.ValueOf
	}
	return nil
}

func (x *Value) GetBoolValue() bool {
	if x, ok := x.GetValueOf().(*Value_BoolValue); ok {
		return x.BoolValue
	}
	return false
}

func (x *Value) GetStringValue() string {
	if x, ok := x.GetValueOf().(*Value_StringValue); ok {
		return x.StringValue
	}
	return ""
}

func (x *Value) GetBytesValue() []byte {
	if x, ok := x.GetValueOf().(*Value_BytesValue); ok {
		return x.BytesValue
	}
	return nil
}

func (x *Value) GetUuidValue() *Uuid {
	if x, ok := x.GetValueOf().(*Value_UuidValue); ok {
		return x.UuidValue
	}
	return nil
}

func (x *Value) GetNumberValue() *Number {
	if x, ok := x.GetValueOf().(*Value_NumberValue); ok {
		return x.NumberValue
	}
	return nil
}

func (x *Value) GetDateTimeValue() *DateTime {
	if x, ok := x.GetValueOf().(*Value_DateTimeValue); ok {
		return x.DateTimeValue
	}
	return nil
}

func (x *Value) GetDateValue() *Date {
	if x, ok := x.GetValueOf().(*Value_DateValue); ok {
		return x.DateValue
	}
	return nil
}

func (x *Value) GetDateRangeValue() *DateRange {
	if x, ok := x.GetValueOf().(*Value_DateRangeValue); ok {
		return x.DateRangeValue
	}
	return nil
}

func (x *Value) GetArrayValues() *ArrayValue {
	if x, ok := x.GetValueOf().(*Value_ArrayValues); ok {
		return x.ArrayValues
	}
	return nil
}

type isValue_ValueOf interface {
	isValue_ValueOf()
}

type Value_BoolValue struct {
	BoolValue bool `protobuf:"varint,1,opt,name=bool_value,json=boolValue,proto3,oneof"`
}

type Value_StringValue struct {
	StringValue string `protobuf:"bytes,2,opt,name=string_value,json=stringValue,proto3,oneof"`
}

type Value_BytesValue struct {
	// byte
	BytesValue []byte `protobuf:"bytes,3,opt,name=bytes_value,json=bytesValue,proto3,oneof"`
}

type Value_UuidValue struct {
	// uuid
	UuidValue *Uuid `protobuf:"bytes,4,opt,name=uuid_value,json=uuidValue,proto3,oneof"`
}

type Value_NumberValue struct {
	NumberValue *Number `protobuf:"bytes,5,opt,name=number_value,json=numberValue,proto3,oneof"`
}

type Value_DateTimeValue struct {
	DateTimeValue *DateTime `protobuf:"bytes,8,opt,name=date_time_value,json=dateTimeValue,proto3,oneof"`
}

type Value_DateValue struct {
	DateValue *Date `protobuf:"bytes,9,opt,name=date_value,json=dateValue,proto3,oneof"`
}

type Value_DateRangeValue struct {
	DateRangeValue *DateRange `protobuf:"bytes,10,opt,name=date_range_value,json=dateRangeValue,proto3,oneof"`
}

type Value_ArrayValues struct {
	ArrayValues *ArrayValue `protobuf:"bytes,11,opt,name=array_values,json=arrayValues,proto3,oneof"`
}

func (*Value_BoolValue) isValue_ValueOf() {}

func (*Value_StringValue) isValue_ValueOf() {}

func (*Value_BytesValue) isValue_ValueOf() {}

func (*Value_UuidValue) isValue_ValueOf() {}

func (*Value_NumberValue) isValue_ValueOf() {}

func (*Value_DateTimeValue) isValue_ValueOf() {}

func (*Value_DateValue) isValue_ValueOf() {}

func (*Value_DateRangeValue) isValue_ValueOf() {}

func (*Value_ArrayValues) isValue_ValueOf() {}

type ArrayValue struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	Values []*Value `protobuf:"bytes,1,rep,name=values,proto3" json:"values,omitempty"`
}

func (x *ArrayValue) Reset() {
	*x = ArrayValue{}
	if protoimpl.UnsafeEnabled {
		mi := &file_typ_p_value_proto_msgTypes[1]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *ArrayValue) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*ArrayValue) ProtoMessage() {}

func (x *ArrayValue) ProtoReflect() protoreflect.Message {
	mi := &file_typ_p_value_proto_msgTypes[1]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use ArrayValue.ProtoReflect.Descriptor instead.
func (*ArrayValue) Descriptor() ([]byte, []int) {
	return file_typ_p_value_proto_rawDescGZIP(), []int{1}
}

func (x *ArrayValue) GetValues() []*Value {
	if x != nil {
		return x.Values
	}
	return nil
}

var File_typ_p_value_proto protoreflect.FileDescriptor

var file_typ_p_value_proto_rawDesc = []byte{
	0x0a, 0x11, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x12, 0x05, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x1a, 0x16, 0x74, 0x79, 0x70, 0x5f,
	0x70, 0x2f, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x64, 0x65, 0x6c, 0x74, 0x61, 0x2e, 0x70, 0x72, 0x6f,
	0x74, 0x6f, 0x1a, 0x16, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2f, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x72,
	0x61, 0x6e, 0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x15, 0x74, 0x79, 0x70, 0x5f,
	0x70, 0x2f, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x1a, 0x10, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2f, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72,
	0x6f, 0x74, 0x6f, 0x1a, 0x12, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2f, 0x6e, 0x75, 0x6d, 0x62, 0x65,
	0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x10, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2f, 0x75,
	0x75, 0x69, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc0, 0x03, 0x0a, 0x05, 0x76, 0x61,
	0x6c, 0x75, 0x65, 0x12, 0x1f, 0x0a, 0x0a, 0x62, 0x6f, 0x6f, 0x6c, 0x5f, 0x76, 0x61, 0x6c, 0x75,
	0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x48, 0x00, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x6c, 0x56,
	0x61, 0x6c, 0x75, 0x65, 0x12, 0x23, 0x0a, 0x0c, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67, 0x5f, 0x76,
	0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x48, 0x00, 0x52, 0x0b, 0x73, 0x74,
	0x72, 0x69, 0x6e, 0x67, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x21, 0x0a, 0x0b, 0x62, 0x79, 0x74,
	0x65, 0x73, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x48, 0x00,
	0x52, 0x0a, 0x62, 0x79, 0x74, 0x65, 0x73, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x2c, 0x0a, 0x0a,
	0x75, 0x75, 0x69, 0x64, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x0b, 0x2e, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2e, 0x75, 0x75, 0x69, 0x64, 0x48, 0x00, 0x52,
	0x09, 0x75, 0x75, 0x69, 0x64, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x32, 0x0a, 0x0c, 0x6e, 0x75,
	0x6d, 0x62, 0x65, 0x72, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
	0x32, 0x0d, 0x2e, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2e, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x48,
	0x00, 0x52, 0x0b, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x3a,
	0x0a, 0x0f, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x75,
	0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2e,
	0x64, 0x61, 0x74, 0x65, 0x5f, 0x74, 0x69, 0x6d, 0x65, 0x48, 0x00, 0x52, 0x0d, 0x64, 0x61, 0x74,
	0x65, 0x54, 0x69, 0x6d, 0x65, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x2c, 0x0a, 0x0a, 0x64, 0x61,
	0x74, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b,
	0x2e, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2e, 0x64, 0x61, 0x74, 0x65, 0x48, 0x00, 0x52, 0x09, 0x64,
	0x61, 0x74, 0x65, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x3d, 0x0a, 0x10, 0x64, 0x61, 0x74, 0x65,
	0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x0a, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x11, 0x2e, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2e, 0x64, 0x61, 0x74, 0x65, 0x5f,
	0x72, 0x61, 0x6e, 0x67, 0x65, 0x48, 0x00, 0x52, 0x0e, 0x64, 0x61, 0x74, 0x65, 0x52, 0x61, 0x6e,
	0x67, 0x65, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x37, 0x0a, 0x0c, 0x61, 0x72, 0x72, 0x61, 0x79,
	0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e,
	0x74, 0x79, 0x70, 0x5f, 0x70, 0x2e, 0x61, 0x72, 0x72, 0x61, 0x79, 0x5f, 0x76, 0x61, 0x6c, 0x75,
	0x65, 0x48, 0x00, 0x52, 0x0b, 0x61, 0x72, 0x72, 0x61, 0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x73,
	0x42, 0x0a, 0x0a, 0x08, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x5f, 0x6f, 0x66, 0x22, 0x33, 0x0a, 0x0b,
	0x61, 0x72, 0x72, 0x61, 0x79, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x24, 0x0a, 0x06, 0x76,
	0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x74, 0x79,
	0x70, 0x5f, 0x70, 0x2e, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x52, 0x06, 0x76, 0x61, 0x6c, 0x75, 0x65,
	0x73, 0x2a, 0xf7, 0x01, 0x0a, 0x09, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
	0x16, 0x0a, 0x12, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x4e,
	0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10, 0x00, 0x12, 0x13, 0x0a, 0x0f, 0x56, 0x41, 0x4c, 0x55, 0x45,
	0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x42, 0x4f, 0x4f, 0x4c, 0x10, 0x01, 0x12, 0x15, 0x0a, 0x11,
	0x56, 0x41, 0x4c, 0x55, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x53, 0x54, 0x52, 0x49, 0x4e,
	0x47, 0x10, 0x02, 0x12, 0x14, 0x0a, 0x10, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x5f, 0x54, 0x59, 0x50,
	0x45, 0x5f, 0x42, 0x59, 0x54, 0x45, 0x53, 0x10, 0x03, 0x12, 0x13, 0x0a, 0x0f, 0x56, 0x41, 0x4c,
	0x55, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x55, 0x55, 0x49, 0x44, 0x10, 0x04, 0x12, 0x15,
	0x0a, 0x11, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x4e, 0x55, 0x4d,
	0x42, 0x45, 0x52, 0x10, 0x05, 0x12, 0x17, 0x0a, 0x13, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x5f, 0x54,
	0x59, 0x50, 0x45, 0x5f, 0x44, 0x41, 0x54, 0x45, 0x54, 0x49, 0x4d, 0x45, 0x10, 0x06, 0x12, 0x13,
	0x0a, 0x0f, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x44, 0x41, 0x54,
	0x45, 0x10, 0x07, 0x12, 0x19, 0x0a, 0x15, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x5f, 0x54, 0x59, 0x50,
	0x45, 0x5f, 0x44, 0x41, 0x54, 0x45, 0x5f, 0x52, 0x41, 0x4e, 0x47, 0x45, 0x10, 0x08, 0x12, 0x1b,
	0x0a, 0x17, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x5f, 0x54, 0x59, 0x50, 0x45, 0x5f, 0x41, 0x52, 0x52,
	0x41, 0x59, 0x5f, 0x56, 0x41, 0x4c, 0x55, 0x45, 0x53, 0x10, 0x0a, 0x42, 0x2c, 0x5a, 0x2a, 0x67,
	0x69, 0x74, 0x6c, 0x61, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6d, 0x69, 0x70, 0x70, 0x6f, 0x2e,
	0x61, 0x70, 0x70, 0x2f, 0x74, 0x79, 0x70, 0x2d, 0x70, 0x2e, 0x67, 0x69, 0x74, 0x2f, 0x67, 0x6f,
	0x2f, 0x70, 0x62, 0x2f, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f,
	0x33,
}

var (
	file_typ_p_value_proto_rawDescOnce sync.Once
	file_typ_p_value_proto_rawDescData = file_typ_p_value_proto_rawDesc
)

func file_typ_p_value_proto_rawDescGZIP() []byte {
	file_typ_p_value_proto_rawDescOnce.Do(func() {
		file_typ_p_value_proto_rawDescData = protoimpl.X.CompressGZIP(file_typ_p_value_proto_rawDescData)
	})
	return file_typ_p_value_proto_rawDescData
}

var file_typ_p_value_proto_enumTypes = make([]protoimpl.EnumInfo, 1)
var file_typ_p_value_proto_msgTypes = make([]protoimpl.MessageInfo, 2)
var file_typ_p_value_proto_goTypes = []interface{}{
	(ValueType)(0),     // 0: typ_p.ValueType
	(*Value)(nil),      // 1: typ_p.value
	(*ArrayValue)(nil), // 2: typ_p.array_value
	(*Uuid)(nil),       // 3: typ_p.uuid
	(*Number)(nil),     // 4: typ_p.number
	(*DateTime)(nil),   // 5: typ_p.date_time
	(*Date)(nil),       // 6: typ_p.date
	(*DateRange)(nil),  // 7: typ_p.date_range
}
var file_typ_p_value_proto_depIdxs = []int32{
	3, // 0: typ_p.value.uuid_value:type_name -> typ_p.uuid
	4, // 1: typ_p.value.number_value:type_name -> typ_p.number
	5, // 2: typ_p.value.date_time_value:type_name -> typ_p.date_time
	6, // 3: typ_p.value.date_value:type_name -> typ_p.date
	7, // 4: typ_p.value.date_range_value:type_name -> typ_p.date_range
	2, // 5: typ_p.value.array_values:type_name -> typ_p.array_value
	1, // 6: typ_p.array_value.values:type_name -> typ_p.value
	7, // [7:7] is the sub-list for method output_type
	7, // [7:7] is the sub-list for method input_type
	7, // [7:7] is the sub-list for extension type_name
	7, // [7:7] is the sub-list for extension extendee
	0, // [0:7] is the sub-list for field type_name
}

func init() { file_typ_p_value_proto_init() }
func file_typ_p_value_proto_init() {
	if File_typ_p_value_proto != nil {
		return
	}
	file_typ_p_date_delta_proto_init()
	file_typ_p_date_range_proto_init()
	file_typ_p_date_time_proto_init()
	file_typ_p_date_proto_init()
	file_typ_p_number_proto_init()
	file_typ_p_uuid_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_typ_p_value_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*Value); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
		file_typ_p_value_proto_msgTypes[1].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*ArrayValue); i {
			case 0:
				return &v.state
			case 1:
				return &v.sizeCache
			case 2:
				return &v.unknownFields
			default:
				return nil
			}
		}
	}
	file_typ_p_value_proto_msgTypes[0].OneofWrappers = []interface{}{
		(*Value_BoolValue)(nil),
		(*Value_StringValue)(nil),
		(*Value_BytesValue)(nil),
		(*Value_UuidValue)(nil),
		(*Value_NumberValue)(nil),
		(*Value_DateTimeValue)(nil),
		(*Value_DateValue)(nil),
		(*Value_DateRangeValue)(nil),
		(*Value_ArrayValues)(nil),
	}
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_typ_p_value_proto_rawDesc,
			NumEnums:      1,
			NumMessages:   2,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_typ_p_value_proto_goTypes,
		DependencyIndexes: file_typ_p_value_proto_depIdxs,
		EnumInfos:         file_typ_p_value_proto_enumTypes,
		MessageInfos:      file_typ_p_value_proto_msgTypes,
	}.Build()
	File_typ_p_value_proto = out.File
	file_typ_p_value_proto_rawDesc = nil
	file_typ_p_value_proto_goTypes = nil
	file_typ_p_value_proto_depIdxs = nil
}