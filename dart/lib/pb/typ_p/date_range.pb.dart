//
//  Generated code. Do not modify.
//  source: typ_p/date_range.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

import 'date.pb.dart' as $0;

class date_range extends $pb.GeneratedMessage {
  factory date_range({
    $0.date? dateFrom,
    $0.date? dateTo,
  }) {
    final $result = create();
    if (dateFrom != null) {
      $result.dateFrom = dateFrom;
    }
    if (dateTo != null) {
      $result.dateTo = dateTo;
    }
    return $result;
  }
  date_range._() : super();
  factory date_range.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory date_range.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'date_range', package: const $pb.PackageName(_omitMessageNames ? '' : 'typ_p'), createEmptyInstance: create)
    ..aOM<$0.date>(1, _omitFieldNames ? '' : 'dateFrom', subBuilder: $0.date.create)
    ..aOM<$0.date>(2, _omitFieldNames ? '' : 'dateTo', subBuilder: $0.date.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  date_range clone() => date_range()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  date_range copyWith(void Function(date_range) updates) => super.copyWith((message) => updates(message as date_range)) as date_range;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static date_range create() => date_range._();
  date_range createEmptyInstance() => create();
  static $pb.PbList<date_range> createRepeated() => $pb.PbList<date_range>();
  @$core.pragma('dart2js:noInline')
  static date_range getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<date_range>(create);
  static date_range? _defaultInstance;

  @$pb.TagNumber(1)
  $0.date get dateFrom => $_getN(0);
  @$pb.TagNumber(1)
  set dateFrom($0.date v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasDateFrom() => $_has(0);
  @$pb.TagNumber(1)
  void clearDateFrom() => clearField(1);
  @$pb.TagNumber(1)
  $0.date ensureDateFrom() => $_ensure(0);

  @$pb.TagNumber(2)
  $0.date get dateTo => $_getN(1);
  @$pb.TagNumber(2)
  set dateTo($0.date v) { setField(2, v); }
  @$pb.TagNumber(2)
  $core.bool hasDateTo() => $_has(1);
  @$pb.TagNumber(2)
  void clearDateTo() => clearField(2);
  @$pb.TagNumber(2)
  $0.date ensureDateTo() => $_ensure(1);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
