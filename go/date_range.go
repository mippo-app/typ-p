package typ_p

import (
	pb "gitlab.com/mippo.app/typ-p.git/go/pb/typ_p"
)

type DateRange struct {
	DateFrom *Date
	DateTo   *Date
}

func DateRangeFromPb(v *pb.DateRange) *DateRange {
	r := &DateRange{
		DateFrom: DateFromPb(v.DateFrom),
		DateTo:   DateFromPb(v.DateTo),
	}

	return r
}

func (h *DateRange) ToPb() *pb.DateRange {
	r := &pb.DateRange{
		DateFrom: h.DateFrom.ToPb(),
		DateTo:   h.DateTo.ToPb(),
	}

	return r
}
