package typ_p

import (
	pb "gitlab.com/mippo.app/typ-p.git/go/pb/typ_p"
)

type Uuid struct {
	UuidValue []byte
}

func (h *Uuid) ToPb() *pb.Uuid {
	r := &pb.Uuid{
		UuidValue: h.UuidValue,
	}

	return r
}

func UuidFromPb(v *pb.Uuid) *Uuid {
	r := &Uuid{
		UuidValue: v.UuidValue,
	}

	return r
}
