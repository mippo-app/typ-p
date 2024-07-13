//
//  Generated code. Do not modify.
//  source: typ_p/uuid.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class uuid extends $pb.GeneratedMessage {
  factory uuid({
    $core.List<$core.int>? uuidValue,
  }) {
    final $result = create();
    if (uuidValue != null) {
      $result.uuidValue = uuidValue;
    }
    return $result;
  }
  uuid._() : super();
  factory uuid.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory uuid.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'uuid', package: const $pb.PackageName(_omitMessageNames ? '' : 'typ_p'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'uuidValue', $pb.PbFieldType.OY)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  uuid clone() => uuid()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  uuid copyWith(void Function(uuid) updates) => super.copyWith((message) => updates(message as uuid)) as uuid;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static uuid create() => uuid._();
  uuid createEmptyInstance() => create();
  static $pb.PbList<uuid> createRepeated() => $pb.PbList<uuid>();
  @$core.pragma('dart2js:noInline')
  static uuid getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<uuid>(create);
  static uuid? _defaultInstance;

  @$pb.TagNumber(1)
  $core.List<$core.int> get uuidValue => $_getN(0);
  @$pb.TagNumber(1)
  set uuidValue($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasUuidValue() => $_has(0);
  @$pb.TagNumber(1)
  void clearUuidValue() => clearField(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
