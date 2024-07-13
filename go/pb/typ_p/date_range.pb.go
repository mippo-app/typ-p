// Code generated by protoc-gen-go. DO NOT EDIT.
// versions:
// 	protoc-gen-go v1.31.0
// 	protoc        (unknown)
// source: typ_p/date_range.proto

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

type DateRange struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	DateFrom *Date `protobuf:"bytes,1,opt,name=date_from,json=dateFrom,proto3" json:"date_from,omitempty"`
	DateTo   *Date `protobuf:"bytes,2,opt,name=date_to,json=dateTo,proto3" json:"date_to,omitempty"`
}

func (x *DateRange) Reset() {
	*x = DateRange{}
	if protoimpl.UnsafeEnabled {
		mi := &file_typ_p_date_range_proto_msgTypes[0]
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		ms.StoreMessageInfo(mi)
	}
}

func (x *DateRange) String() string {
	return protoimpl.X.MessageStringOf(x)
}

func (*DateRange) ProtoMessage() {}

func (x *DateRange) ProtoReflect() protoreflect.Message {
	mi := &file_typ_p_date_range_proto_msgTypes[0]
	if protoimpl.UnsafeEnabled && x != nil {
		ms := protoimpl.X.MessageStateOf(protoimpl.Pointer(x))
		if ms.LoadMessageInfo() == nil {
			ms.StoreMessageInfo(mi)
		}
		return ms
	}
	return mi.MessageOf(x)
}

// Deprecated: Use DateRange.ProtoReflect.Descriptor instead.
func (*DateRange) Descriptor() ([]byte, []int) {
	return file_typ_p_date_range_proto_rawDescGZIP(), []int{0}
}

func (x *DateRange) GetDateFrom() *Date {
	if x != nil {
		return x.DateFrom
	}
	return nil
}

func (x *DateRange) GetDateTo() *Date {
	if x != nil {
		return x.DateTo
	}
	return nil
}

var File_typ_p_date_range_proto protoreflect.FileDescriptor

var file_typ_p_date_range_proto_rawDesc = []byte{
	0x0a, 0x16, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2f, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x72, 0x61, 0x6e,
	0x67, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x05, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x1a,
	0x10, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2f, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
	0x6f, 0x22, 0x5c, 0x0a, 0x0a, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x12,
	0x28, 0x0a, 0x09, 0x64, 0x61, 0x74, 0x65, 0x5f, 0x66, 0x72, 0x6f, 0x6d, 0x18, 0x01, 0x20, 0x01,
	0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x2e, 0x64, 0x61, 0x74, 0x65, 0x52,
	0x08, 0x64, 0x61, 0x74, 0x65, 0x46, 0x72, 0x6f, 0x6d, 0x12, 0x24, 0x0a, 0x07, 0x64, 0x61, 0x74,
	0x65, 0x5f, 0x74, 0x6f, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x74, 0x79, 0x70,
	0x5f, 0x70, 0x2e, 0x64, 0x61, 0x74, 0x65, 0x52, 0x06, 0x64, 0x61, 0x74, 0x65, 0x54, 0x6f, 0x42,
	0x2c, 0x5a, 0x2a, 0x67, 0x69, 0x74, 0x6c, 0x61, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x6d, 0x69,
	0x70, 0x70, 0x6f, 0x2e, 0x61, 0x70, 0x70, 0x2f, 0x74, 0x79, 0x70, 0x2d, 0x70, 0x2e, 0x67, 0x69,
	0x74, 0x2f, 0x67, 0x6f, 0x2f, 0x70, 0x62, 0x2f, 0x74, 0x79, 0x70, 0x5f, 0x70, 0x62, 0x06, 0x70,
	0x72, 0x6f, 0x74, 0x6f, 0x33,
}

var (
	file_typ_p_date_range_proto_rawDescOnce sync.Once
	file_typ_p_date_range_proto_rawDescData = file_typ_p_date_range_proto_rawDesc
)

func file_typ_p_date_range_proto_rawDescGZIP() []byte {
	file_typ_p_date_range_proto_rawDescOnce.Do(func() {
		file_typ_p_date_range_proto_rawDescData = protoimpl.X.CompressGZIP(file_typ_p_date_range_proto_rawDescData)
	})
	return file_typ_p_date_range_proto_rawDescData
}

var file_typ_p_date_range_proto_msgTypes = make([]protoimpl.MessageInfo, 1)
var file_typ_p_date_range_proto_goTypes = []interface{}{
	(*DateRange)(nil), // 0: typ_p.date_range
	(*Date)(nil),      // 1: typ_p.date
}
var file_typ_p_date_range_proto_depIdxs = []int32{
	1, // 0: typ_p.date_range.date_from:type_name -> typ_p.date
	1, // 1: typ_p.date_range.date_to:type_name -> typ_p.date
	2, // [2:2] is the sub-list for method output_type
	2, // [2:2] is the sub-list for method input_type
	2, // [2:2] is the sub-list for extension type_name
	2, // [2:2] is the sub-list for extension extendee
	0, // [0:2] is the sub-list for field type_name
}

func init() { file_typ_p_date_range_proto_init() }
func file_typ_p_date_range_proto_init() {
	if File_typ_p_date_range_proto != nil {
		return
	}
	file_typ_p_date_proto_init()
	if !protoimpl.UnsafeEnabled {
		file_typ_p_date_range_proto_msgTypes[0].Exporter = func(v interface{}, i int) interface{} {
			switch v := v.(*DateRange); i {
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
	type x struct{}
	out := protoimpl.TypeBuilder{
		File: protoimpl.DescBuilder{
			GoPackagePath: reflect.TypeOf(x{}).PkgPath(),
			RawDescriptor: file_typ_p_date_range_proto_rawDesc,
			NumEnums:      0,
			NumMessages:   1,
			NumExtensions: 0,
			NumServices:   0,
		},
		GoTypes:           file_typ_p_date_range_proto_goTypes,
		DependencyIndexes: file_typ_p_date_range_proto_depIdxs,
		MessageInfos:      file_typ_p_date_range_proto_msgTypes,
	}.Build()
	File_typ_p_date_range_proto = out.File
	file_typ_p_date_range_proto_rawDesc = nil
	file_typ_p_date_range_proto_goTypes = nil
	file_typ_p_date_range_proto_depIdxs = nil
}
