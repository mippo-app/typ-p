library typ_p;

import 'package:typ_p/pb/typ_p/date.pb.dart';

DateTime conv_date_to_datetime(date a_date) {
  var y_mask = 4294901760;
  var m_mask = 65280;
  var d_mask = 255;

  var v = a_date.value;

  var y = (v & y_mask) >> 16;
  var m = (v & m_mask) >> 8;
  var d = (v & d_mask);

  return DateTime(y, m, d);
}
