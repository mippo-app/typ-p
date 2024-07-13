package typ_p

import (
	"time"

	pb "gitlab.com/mippo.app/typ-p.git/go/pb/typ_p"
)

type Date struct {
	y uint16
	m uint8
	d uint8
}

func (h *Date) GetPureValue() interface{} {
	// a := time.Date(int(h.y), time.Month(h.m), int(h.d), 0, 0, 0, 0, time.Local)
	a := time.Date(int(h.y), time.Month(h.m), int(h.d), 0, 0, 0, 0, time.UTC)
	return &a
}

func DateFromPb(v *pb.Date) *Date {
	r := &Date{}

	vv := v.Value

	y_mask := uint32(^uint16(0)) << 16
	m_mask := uint32(^uint16(0)) << 8
	d_mask := uint32(^uint16(0))

	y := vv & y_mask >> 16
	m := vv & m_mask >> 8
	d := vv & d_mask

	r.y = uint16(y)
	r.m = uint8(m)
	r.d = uint8(d)

	return r
}

func (h *Date) ToPb() *pb.Date {
	y_bit := uint32(h.y) << 16
	m_bit := uint32(h.m) << 8
	d_bit := h.d

	v := uint32(y_bit) | uint32(m_bit) | uint32(d_bit)

	r := pb.Date{
		Value: v,
	}

	return &r
}
