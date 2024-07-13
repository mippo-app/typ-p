package typ_p

import (
	pb "gitlab.com/mippo.app/typ-p.git/go/pb/typ_p"
)

type Number struct {
	ValueOf isNumber_ValueOf
}

func (h *Number) GetPureValue() interface{} {
	switch v := h.ValueOf.(type) {
	case *Number_Int32Value:
		return v.Int32Value
	case *Number_Int64Value:
		return v.Int64Value
	case *Number_FloatValue:
		return v.FloatValue
	case *Number_DoubleValue:
		return v.DoubleValue
	default:
		panic("")
	}

}

func NumberFromPb(v *pb.Number) *Number {
	r := &Number{}

	switch v := v.ValueOf.(type) {
	case *pb.Number_Int32Value:
		a := &Number_Int32Value{
			Int32Value: v.Int32Value,
		}

		r.ValueOf = a
	case *pb.Number_Int64Value:
		a := &Number_Int64Value{
			Int64Value: v.Int64Value,
		}

		r.ValueOf = a
	case *pb.Number_FloatValue:
		a := &Number_FloatValue{
			FloatValue: v.FloatValue,
		}

		r.ValueOf = a
	case *pb.Number_DoubleValue:
		a := &Number_DoubleValue{
			DoubleValue: v.DoubleValue,
		}

		r.ValueOf = a
	default:
		panic("")
	}

	return r
}

func (v *Number) ToPb() *pb.Number {
	r := &pb.Number{}

	switch v := v.ValueOf.(type) {
	case *Number_Int32Value:
		a := &pb.Number_Int32Value{
			Int32Value: v.Int32Value,
		}

		r.ValueOf = a
	case *Number_Int64Value:
		a := &pb.Number_Int64Value{
			Int64Value: v.Int64Value,
		}

		r.ValueOf = a
	case *Number_FloatValue:
		a := &pb.Number_FloatValue{
			FloatValue: v.FloatValue,
		}

		r.ValueOf = a
	case *Number_DoubleValue:
		a := &pb.Number_DoubleValue{
			DoubleValue: v.DoubleValue,
		}

		r.ValueOf = a
	default:
		panic("")
	}

	return r
}

type isNumber_ValueOf interface {
	isNumber_ValueOf()
}

type Number_Int32Value struct {
	Int32Value int32
}

type Number_Int64Value struct {
	Int64Value int64
}

type Number_FloatValue struct {
	FloatValue float32
}

type Number_DoubleValue struct {
	DoubleValue float64
}

func (*Number_Int32Value) isNumber_ValueOf() {}

func (*Number_Int64Value) isNumber_ValueOf() {}

func (*Number_FloatValue) isNumber_ValueOf() {}

func (*Number_DoubleValue) isNumber_ValueOf() {}
