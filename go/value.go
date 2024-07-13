package typ_p

import (
	pb "gitlab.com/mippo.app/typ-p.git/go/pb/typ_p"
)

type Value struct {
	ValueOf isValue_ValueOf
}

func (h *Value) GetPureValue() interface{} {
	switch vv := h.ValueOf.(type) {
	case *Value_BoolValue:
		return vv.BoolValue
	case *Value_StringValue:
		return vv.StringValue
	case *Value_BytesValue:
		return vv.BytesValue
	case *Value_UuidValue:
		return vv.UuidValue.UuidValue
	case *Value_NumberValue:
		return vv.NumberValue.GetPureValue()
	case *Value_DateTimeValue:
		return vv.DateTimeValue.GetPureValue()
	case *Value_DateValue:
		return vv.DateValue.GetPureValue()
	case *Value_DateRangeValue:
		return vv.DateRangeValue
	case *Value_ArrayValues:
		r := []interface{}{}
		for _, a := range vv.ArrayValues.Values {
			r = append(r, a.GetPureValue())
		}

		return r
	default:
		panic("")
	}

}

func (h *Value) ToPb() *pb.Value {
	r := &pb.Value{}

	switch vv := h.ValueOf.(type) {
	case *Value_BoolValue:
		a := &pb.Value_BoolValue{
			BoolValue: vv.BoolValue,
		}

		r.ValueOf = a
	case *Value_StringValue:
		a := &pb.Value_StringValue{
			StringValue: vv.StringValue,
		}

		r.ValueOf = a
	case *Value_BytesValue:
		a := &pb.Value_BytesValue{
			BytesValue: vv.BytesValue,
		}

		r.ValueOf = a
	case *Value_UuidValue:
		a := &pb.Value_UuidValue{
			UuidValue: vv.UuidValue.ToPb(),
		}

		r.ValueOf = a
	case *Value_NumberValue:
		a := &pb.Value_NumberValue{
			NumberValue: vv.NumberValue.ToPb(),
		}

		r.ValueOf = a
	case *Value_DateTimeValue:
		a := &pb.Value_DateTimeValue{
			DateTimeValue: vv.DateTimeValue.ToPb(),
		}

		r.ValueOf = a
	case *Value_DateValue:
		a := &pb.Value_DateValue{
			DateValue: vv.DateValue.ToPb(),
		}

		r.ValueOf = a
	case *Value_DateRangeValue:
		a := &pb.Value_DateRangeValue{
			DateRangeValue: vv.DateRangeValue.ToPb(),
		}

		r.ValueOf = a
	case *Value_ArrayValues:
		a := &pb.Value_ArrayValues{
			ArrayValues: vv.ArrayValues.ToPb(),
		}

		r.ValueOf = a
	default:
		panic("")
	}

	return r
}

func ValueFromPb(v *pb.Value) *Value {
	r := &Value{}

	switch vv := v.ValueOf.(type) {
	case *pb.Value_BoolValue:
		a := &Value_BoolValue{
			BoolValue: vv.BoolValue,
		}

		r.ValueOf = a
	case *pb.Value_StringValue:
		a := &Value_StringValue{
			StringValue: vv.StringValue,
		}

		r.ValueOf = a
	case *pb.Value_BytesValue:
		a := &Value_BytesValue{
			BytesValue: vv.BytesValue,
		}

		r.ValueOf = a
	case *pb.Value_UuidValue:
		a := &Value_UuidValue{
			UuidValue: UuidFromPb(vv.UuidValue),
		}

		r.ValueOf = a
	case *pb.Value_NumberValue:
		a := &Value_NumberValue{
			NumberValue: NumberFromPb(vv.NumberValue),
		}

		r.ValueOf = a
	case *pb.Value_DateTimeValue:
		a := &Value_DateTimeValue{
			DateTimeValue: DateTimeFromPb(vv.DateTimeValue),
		}

		r.ValueOf = a
	case *pb.Value_DateValue:
		a := &Value_DateValue{
			DateValue: DateFromPb(vv.DateValue),
		}

		r.ValueOf = a
	case *pb.Value_DateRangeValue:
		a := &Value_DateRangeValue{
			DateRangeValue: DateRangeFromPb(vv.DateRangeValue),
		}

		r.ValueOf = a
	case *pb.Value_ArrayValues:
		a := &Value_ArrayValues{
			ArrayValues: ArrayValueFromPb(vv.ArrayValues),
		}

		r.ValueOf = a
	default:
		panic("")
	}

	return r
}

type isValue_ValueOf interface {
	isValue_ValueOf()
}

type Value_BoolValue struct {
	BoolValue bool
}

type Value_StringValue struct {
	StringValue string
}

type Value_BytesValue struct {
	// byte
	BytesValue []byte
}

type Value_UuidValue struct {
	// uuid
	UuidValue *Uuid
}

type Value_NumberValue struct {
	NumberValue *Number
}

type Value_DateTimeValue struct {
	DateTimeValue *DateTime
}

type Value_DateValue struct {
	DateValue *Date
}

type Value_DateRangeValue struct {
	DateRangeValue *DateRange
}

type Value_ArrayValues struct {
	ArrayValues *ArrayValue
}

func (*Value_BoolValue) isValue_ValueOf() {}

func (*Value_StringValue) isValue_ValueOf() {}

func (*Value_BytesValue) isValue_ValueOf() {}

func (*Value_UuidValue) isValue_ValueOf() {}

func (*Value_NumberValue) isValue_ValueOf() {}

func (*Value_DateTimeValue) isValue_ValueOf() {}

func (*Value_DateValue) isValue_ValueOf() {}

func (*Value_DateRangeValue) isValue_ValueOf() {}

func (*Value_ArrayValues) isValue_ValueOf() {}
