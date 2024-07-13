//
//  Generated code. Do not modify.
//  source: typ_p/value.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:convert' as $convert;
import 'dart:core' as $core;
import 'dart:typed_data' as $typed_data;

@$core.Deprecated('Use valueTypeDescriptor instead')
const ValueType$json = {
  '1': 'ValueType',
  '2': [
    {'1': 'VALUE_TYPE_UNKNOWN', '2': 0},
    {'1': 'VALUE_TYPE_BOOL', '2': 1},
    {'1': 'VALUE_TYPE_STRING', '2': 2},
    {'1': 'VALUE_TYPE_BYTES', '2': 3},
    {'1': 'VALUE_TYPE_UUID', '2': 4},
    {'1': 'VALUE_TYPE_NUMBER', '2': 5},
    {'1': 'VALUE_TYPE_DATETIME', '2': 6},
    {'1': 'VALUE_TYPE_DATE', '2': 7},
    {'1': 'VALUE_TYPE_DATE_RANGE', '2': 8},
    {'1': 'VALUE_TYPE_ARRAY_VALUES', '2': 10},
  ],
};

/// Descriptor for `ValueType`. Decode as a `google.protobuf.EnumDescriptorProto`.
final $typed_data.Uint8List valueTypeDescriptor = $convert.base64Decode(
    'CglWYWx1ZVR5cGUSFgoSVkFMVUVfVFlQRV9VTktOT1dOEAASEwoPVkFMVUVfVFlQRV9CT09MEA'
    'ESFQoRVkFMVUVfVFlQRV9TVFJJTkcQAhIUChBWQUxVRV9UWVBFX0JZVEVTEAMSEwoPVkFMVUVf'
    'VFlQRV9VVUlEEAQSFQoRVkFMVUVfVFlQRV9OVU1CRVIQBRIXChNWQUxVRV9UWVBFX0RBVEVUSU'
    '1FEAYSEwoPVkFMVUVfVFlQRV9EQVRFEAcSGQoVVkFMVUVfVFlQRV9EQVRFX1JBTkdFEAgSGwoX'
    'VkFMVUVfVFlQRV9BUlJBWV9WQUxVRVMQCg==');

@$core.Deprecated('Use valueDescriptor instead')
const value$json = {
  '1': 'value',
  '2': [
    {'1': 'bool_value', '3': 1, '4': 1, '5': 8, '9': 0, '10': 'boolValue'},
    {'1': 'string_value', '3': 2, '4': 1, '5': 9, '9': 0, '10': 'stringValue'},
    {'1': 'bytes_value', '3': 3, '4': 1, '5': 12, '9': 0, '10': 'bytesValue'},
    {'1': 'uuid_value', '3': 4, '4': 1, '5': 11, '6': '.typ_p.uuid', '9': 0, '10': 'uuidValue'},
    {'1': 'number_value', '3': 5, '4': 1, '5': 11, '6': '.typ_p.number', '9': 0, '10': 'numberValue'},
    {'1': 'date_time_value', '3': 8, '4': 1, '5': 11, '6': '.typ_p.date_time', '9': 0, '10': 'dateTimeValue'},
    {'1': 'date_value', '3': 9, '4': 1, '5': 11, '6': '.typ_p.date', '9': 0, '10': 'dateValue'},
    {'1': 'date_range_value', '3': 10, '4': 1, '5': 11, '6': '.typ_p.date_range', '9': 0, '10': 'dateRangeValue'},
    {'1': 'array_values', '3': 11, '4': 1, '5': 11, '6': '.typ_p.array_value', '9': 0, '10': 'arrayValues'},
  ],
  '8': [
    {'1': 'value_of'},
  ],
};

/// Descriptor for `value`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List valueDescriptor = $convert.base64Decode(
    'CgV2YWx1ZRIfCgpib29sX3ZhbHVlGAEgASgISABSCWJvb2xWYWx1ZRIjCgxzdHJpbmdfdmFsdW'
    'UYAiABKAlIAFILc3RyaW5nVmFsdWUSIQoLYnl0ZXNfdmFsdWUYAyABKAxIAFIKYnl0ZXNWYWx1'
    'ZRIsCgp1dWlkX3ZhbHVlGAQgASgLMgsudHlwX3AudXVpZEgAUgl1dWlkVmFsdWUSMgoMbnVtYm'
    'VyX3ZhbHVlGAUgASgLMg0udHlwX3AubnVtYmVySABSC251bWJlclZhbHVlEjoKD2RhdGVfdGlt'
    'ZV92YWx1ZRgIIAEoCzIQLnR5cF9wLmRhdGVfdGltZUgAUg1kYXRlVGltZVZhbHVlEiwKCmRhdG'
    'VfdmFsdWUYCSABKAsyCy50eXBfcC5kYXRlSABSCWRhdGVWYWx1ZRI9ChBkYXRlX3JhbmdlX3Zh'
    'bHVlGAogASgLMhEudHlwX3AuZGF0ZV9yYW5nZUgAUg5kYXRlUmFuZ2VWYWx1ZRI3CgxhcnJheV'
    '92YWx1ZXMYCyABKAsyEi50eXBfcC5hcnJheV92YWx1ZUgAUgthcnJheVZhbHVlc0IKCgh2YWx1'
    'ZV9vZg==');

@$core.Deprecated('Use array_valueDescriptor instead')
const array_value$json = {
  '1': 'array_value',
  '2': [
    {'1': 'values', '3': 1, '4': 3, '5': 11, '6': '.typ_p.value', '10': 'values'},
  ],
};

/// Descriptor for `array_value`. Decode as a `google.protobuf.DescriptorProto`.
final $typed_data.Uint8List array_valueDescriptor = $convert.base64Decode(
    'CgthcnJheV92YWx1ZRIkCgZ2YWx1ZXMYASADKAsyDC50eXBfcC52YWx1ZVIGdmFsdWVz');

