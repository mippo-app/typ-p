package typ_p

import (
	"time"

	pb "gitlab.com/mippo.app/typ-p.git/go/pb/typ_p"
)

type DateTime struct {
	Year   int32
	Month  int32
	Day    int32
	Hour   int32
	Minute int32
	Second int32
}

func (h *DateTime) GetPureValue() interface{} {
	a := time.Date(int(h.Year), time.Month(h.Month), int(h.Day), int(h.Hour), int(h.Minute), int(h.Second), 0, time.UTC)
	return &a
}

func DateTimeFromPb(v *pb.DateTime) *DateTime {
	r := &DateTime{
		Year:   v.Year,
		Month:  v.Month,
		Day:    v.Day,
		Hour:   v.Hour,
		Minute: v.Minute,
		Second: v.Second,
	}

	return r
}

func (v *DateTime) ToPb() *pb.DateTime {
	r := &pb.DateTime{
		Year:   v.Year,
		Month:  v.Month,
		Day:    v.Day,
		Hour:   v.Hour,
		Minute: v.Minute,
		Second: v.Second,
	}

	return r
}
