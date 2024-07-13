package typ_p

import (
	pb "gitlab.com/mippo.app/typ-p.git/go/pb/typ_p"
)

type ArrayValue struct {
	Values []*Value
}

func ArrayValueFromPb(v *pb.ArrayValue) *ArrayValue {
	r := &ArrayValue{
		Values: []*Value{},
	}

	for _, v := range v.Values {
		a := ValueFromPb(v)

		r.Values = append(r.Values, a)
	}

	return r
}

func (h *ArrayValue) ToPb() *pb.ArrayValue {
	r := &pb.ArrayValue{}

	for _, v := range h.Values {
		a := v.ToPb()

		r.Values = append(r.Values, a)
	}

	return r
}
