//
//  Generated code. Do not modify.
//  source: typ_p/value.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class ValueType extends $pb.ProtobufEnum {
  static const ValueType VALUE_TYPE_UNKNOWN = ValueType._(0, _omitEnumNames ? '' : 'VALUE_TYPE_UNKNOWN');
  static const ValueType VALUE_TYPE_BOOL = ValueType._(1, _omitEnumNames ? '' : 'VALUE_TYPE_BOOL');
  static const ValueType VALUE_TYPE_STRING = ValueType._(2, _omitEnumNames ? '' : 'VALUE_TYPE_STRING');
  static const ValueType VALUE_TYPE_BYTES = ValueType._(3, _omitEnumNames ? '' : 'VALUE_TYPE_BYTES');
  static const ValueType VALUE_TYPE_UUID = ValueType._(4, _omitEnumNames ? '' : 'VALUE_TYPE_UUID');
  static const ValueType VALUE_TYPE_NUMBER = ValueType._(5, _omitEnumNames ? '' : 'VALUE_TYPE_NUMBER');
  static const ValueType VALUE_TYPE_DATETIME = ValueType._(6, _omitEnumNames ? '' : 'VALUE_TYPE_DATETIME');
  static const ValueType VALUE_TYPE_DATE = ValueType._(7, _omitEnumNames ? '' : 'VALUE_TYPE_DATE');
  static const ValueType VALUE_TYPE_DATE_RANGE = ValueType._(8, _omitEnumNames ? '' : 'VALUE_TYPE_DATE_RANGE');
  static const ValueType VALUE_TYPE_ARRAY_VALUES = ValueType._(10, _omitEnumNames ? '' : 'VALUE_TYPE_ARRAY_VALUES');

  static const $core.List<ValueType> values = <ValueType> [
    VALUE_TYPE_UNKNOWN,
    VALUE_TYPE_BOOL,
    VALUE_TYPE_STRING,
    VALUE_TYPE_BYTES,
    VALUE_TYPE_UUID,
    VALUE_TYPE_NUMBER,
    VALUE_TYPE_DATETIME,
    VALUE_TYPE_DATE,
    VALUE_TYPE_DATE_RANGE,
    VALUE_TYPE_ARRAY_VALUES,
  ];

  static final $core.Map<$core.int, ValueType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static ValueType? valueOf($core.int value) => _byValue[value];

  const ValueType._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
