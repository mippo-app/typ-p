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

import 'date.pb.dart' as $0;
import 'date_range.pb.dart' as $4;
import 'date_time.pb.dart' as $3;
import 'number.pb.dart' as $2;
import 'uuid.pb.dart' as $1;

export 'value.pbenum.dart';

enum value_ValueOf {
  boolValue, 
  stringValue, 
  bytesValue, 
  uuidValue, 
  numberValue, 
  dateTimeValue, 
  dateValue, 
  dateRangeValue, 
  arrayValues, 
  notSet
}

class value extends $pb.GeneratedMessage {
  factory value({
    $core.bool? boolValue,
    $core.String? stringValue,
    $core.List<$core.int>? bytesValue,
    $1.uuid? uuidValue,
    $2.number? numberValue,
    $3.date_time? dateTimeValue,
    $0.date? dateValue,
    $4.date_range? dateRangeValue,
    array_value? arrayValues,
  }) {
    final $result = create();
    if (boolValue != null) {
      $result.boolValue = boolValue;
    }
    if (stringValue != null) {
      $result.stringValue = stringValue;
    }
    if (bytesValue != null) {
      $result.bytesValue = bytesValue;
    }
    if (uuidValue != null) {
      $result.uuidValue = uuidValue;
    }
    if (numberValue != null) {
      $result.numberValue = numberValue;
    }
    if (dateTimeValue != null) {
      $result.dateTimeValue = dateTimeValue;
    }
    if (dateValue != null) {
      $result.dateValue = dateValue;
    }
    if (dateRangeValue != null) {
      $result.dateRangeValue = dateRangeValue;
    }
    if (arrayValues != null) {
      $result.arrayValues = arrayValues;
    }
    return $result;
  }
  value._() : super();
  factory value.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory value.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, value_ValueOf> _value_ValueOfByTag = {
    1 : value_ValueOf.boolValue,
    2 : value_ValueOf.stringValue,
    3 : value_ValueOf.bytesValue,
    4 : value_ValueOf.uuidValue,
    5 : value_ValueOf.numberValue,
    8 : value_ValueOf.dateTimeValue,
    9 : value_ValueOf.dateValue,
    10 : value_ValueOf.dateRangeValue,
    11 : value_ValueOf.arrayValues,
    0 : value_ValueOf.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'value', package: const $pb.PackageName(_omitMessageNames ? '' : 'typ_p'), createEmptyInstance: create)
    ..oo(0, [1, 2, 3, 4, 5, 8, 9, 10, 11])
    ..aOB(1, _omitFieldNames ? '' : 'boolValue')
    ..aOS(2, _omitFieldNames ? '' : 'stringValue')
    ..a<$core.List<$core.int>>(3, _omitFieldNames ? '' : 'bytesValue', $pb.PbFieldType.OY)
    ..aOM<$1.uuid>(4, _omitFieldNames ? '' : 'uuidValue', subBuilder: $1.uuid.create)
    ..aOM<$2.number>(5, _omitFieldNames ? '' : 'numberValue', subBuilder: $2.number.create)
    ..aOM<$3.date_time>(8, _omitFieldNames ? '' : 'dateTimeValue', subBuilder: $3.date_time.create)
    ..aOM<$0.date>(9, _omitFieldNames ? '' : 'dateValue', subBuilder: $0.date.create)
    ..aOM<$4.date_range>(10, _omitFieldNames ? '' : 'dateRangeValue', subBuilder: $4.date_range.create)
    ..aOM<array_value>(11, _omitFieldNames ? '' : 'arrayValues', subBuilder: array_value.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  value clone() => value()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  value copyWith(void Function(value) updates) => super.copyWith((message) => updates(message as value)) as value;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static value create() => value._();
  value createEmptyInstance() => create();
  static $pb.PbList<value> createRepeated() => $pb.PbList<value>();
  @$core.pragma('dart2js:noInline')
  static value getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<value>(create);
  static value? _defaultInstance;

  value_ValueOf whichValueOf() => _value_ValueOfByTag[$_whichOneof(0)]!;
  void clearValueOf() => clearField($_whichOneof(0));

  @$pb.TagNumber(1)
  $core.bool get boolValue => $_getBF(0);
  @$pb.TagNumber(1)
  set boolValue($core.bool v) { $_setBool(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasBoolValue() => $_has(0);
  @$pb.TagNumber(1)
  void clearBoolValue() => clearField(1);

  @$pb.TagNumber(2)
  $core.String get stringValue => $_getSZ(1);
  @$pb.TagNumber(2)
  set stringValue($core.String v) { $_setString(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasStringValue() => $_has(1);
  @$pb.TagNumber(2)
  void clearStringValue() => clearField(2);

  /// byte
  @$pb.TagNumber(3)
  $core.List<$core.int> get bytesValue => $_getN(2);
  @$pb.TagNumber(3)
  set bytesValue($core.List<$core.int> v) { $_setBytes(2, v); }
  @$pb.TagNumber(3)
  $core.bool hasBytesValue() => $_has(2);
  @$pb.TagNumber(3)
  void clearBytesValue() => clearField(3);

  /// uuid
  @$pb.TagNumber(4)
  $1.uuid get uuidValue => $_getN(3);
  @$pb.TagNumber(4)
  set uuidValue($1.uuid v) { setField(4, v); }
  @$pb.TagNumber(4)
  $core.bool hasUuidValue() => $_has(3);
  @$pb.TagNumber(4)
  void clearUuidValue() => clearField(4);
  @$pb.TagNumber(4)
  $1.uuid ensureUuidValue() => $_ensure(3);

  @$pb.TagNumber(5)
  $2.number get numberValue => $_getN(4);
  @$pb.TagNumber(5)
  set numberValue($2.number v) { setField(5, v); }
  @$pb.TagNumber(5)
  $core.bool hasNumberValue() => $_has(4);
  @$pb.TagNumber(5)
  void clearNumberValue() => clearField(5);
  @$pb.TagNumber(5)
  $2.number ensureNumberValue() => $_ensure(4);

  @$pb.TagNumber(8)
  $3.date_time get dateTimeValue => $_getN(5);
  @$pb.TagNumber(8)
  set dateTimeValue($3.date_time v) { setField(8, v); }
  @$pb.TagNumber(8)
  $core.bool hasDateTimeValue() => $_has(5);
  @$pb.TagNumber(8)
  void clearDateTimeValue() => clearField(8);
  @$pb.TagNumber(8)
  $3.date_time ensureDateTimeValue() => $_ensure(5);

  @$pb.TagNumber(9)
  $0.date get dateValue => $_getN(6);
  @$pb.TagNumber(9)
  set dateValue($0.date v) { setField(9, v); }
  @$pb.TagNumber(9)
  $core.bool hasDateValue() => $_has(6);
  @$pb.TagNumber(9)
  void clearDateValue() => clearField(9);
  @$pb.TagNumber(9)
  $0.date ensureDateValue() => $_ensure(6);

  @$pb.TagNumber(10)
  $4.date_range get dateRangeValue => $_getN(7);
  @$pb.TagNumber(10)
  set dateRangeValue($4.date_range v) { setField(10, v); }
  @$pb.TagNumber(10)
  $core.bool hasDateRangeValue() => $_has(7);
  @$pb.TagNumber(10)
  void clearDateRangeValue() => clearField(10);
  @$pb.TagNumber(10)
  $4.date_range ensureDateRangeValue() => $_ensure(7);

  @$pb.TagNumber(11)
  array_value get arrayValues => $_getN(8);
  @$pb.TagNumber(11)
  set arrayValues(array_value v) { setField(11, v); }
  @$pb.TagNumber(11)
  $core.bool hasArrayValues() => $_has(8);
  @$pb.TagNumber(11)
  void clearArrayValues() => clearField(11);
  @$pb.TagNumber(11)
  array_value ensureArrayValues() => $_ensure(8);
}

class array_value extends $pb.GeneratedMessage {
  factory array_value({
    $core.Iterable<value>? values,
  }) {
    final $result = create();
    if (values != null) {
      $result.values.addAll(values);
    }
    return $result;
  }
  array_value._() : super();
  factory array_value.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory array_value.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'array_value', package: const $pb.PackageName(_omitMessageNames ? '' : 'typ_p'), createEmptyInstance: create)
    ..pc<value>(1, _omitFieldNames ? '' : 'values', $pb.PbFieldType.PM, subBuilder: value.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  array_value clone() => array_value()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  array_value copyWith(void Function(array_value) updates) => super.copyWith((message) => updates(message as array_value)) as array_value;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static array_value create() => array_value._();
  array_value createEmptyInstance() => create();
  static $pb.PbList<array_value> createRepeated() => $pb.PbList<array_value>();
  @$core.pragma('dart2js:noInline')
  static array_value getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<array_value>(create);
  static array_value? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<value> get values => $_getList(0);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
