// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: osmosis/accum/v1beta1/accum.proto

package accum

import (
	fmt "fmt"
	github_com_cosmos_cosmos_sdk_types "github.com/cosmos/cosmos-sdk/types"
	types "github.com/cosmos/cosmos-sdk/types"
	_ "github.com/gogo/protobuf/gogoproto"
	proto "github.com/gogo/protobuf/proto"
	io "io"
	math "math"
	math_bits "math/bits"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.GoGoProtoPackageIsVersion3 // please upgrade the proto package

type AccumulatorContent struct {
	AccumValue github_com_cosmos_cosmos_sdk_types.DecCoins `protobuf:"bytes,1,rep,name=accum_value,json=accumValue,proto3,castrepeated=github.com/cosmos/cosmos-sdk/types.DecCoins" json:"accum_value"`
}

func (m *AccumulatorContent) Reset()         { *m = AccumulatorContent{} }
func (m *AccumulatorContent) String() string { return proto.CompactTextString(m) }
func (*AccumulatorContent) ProtoMessage()    {}
func (*AccumulatorContent) Descriptor() ([]byte, []int) {
	return fileDescriptor_4866f7c74a169dc2, []int{0}
}
func (m *AccumulatorContent) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *AccumulatorContent) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_AccumulatorContent.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *AccumulatorContent) XXX_Merge(src proto.Message) {
	xxx_messageInfo_AccumulatorContent.Merge(m, src)
}
func (m *AccumulatorContent) XXX_Size() int {
	return m.Size()
}
func (m *AccumulatorContent) XXX_DiscardUnknown() {
	xxx_messageInfo_AccumulatorContent.DiscardUnknown(m)
}

var xxx_messageInfo_AccumulatorContent proto.InternalMessageInfo

func (m *AccumulatorContent) GetAccumValue() github_com_cosmos_cosmos_sdk_types.DecCoins {
	if m != nil {
		return m.AccumValue
	}
	return nil
}

type Record struct {
	NumShares        github_com_cosmos_cosmos_sdk_types.Dec      `protobuf:"bytes,1,opt,name=num_shares,json=numShares,proto3,customtype=github.com/cosmos/cosmos-sdk/types.Dec" json:"num_shares"`
	InitAccumValue   github_com_cosmos_cosmos_sdk_types.DecCoins `protobuf:"bytes,2,rep,name=init_accum_value,json=initAccumValue,proto3,castrepeated=github.com/cosmos/cosmos-sdk/types.DecCoins" json:"init_accum_value"`
	UnclaimedRewards github_com_cosmos_cosmos_sdk_types.DecCoins `protobuf:"bytes,3,rep,name=unclaimed_rewards,json=unclaimedRewards,proto3,castrepeated=github.com/cosmos/cosmos-sdk/types.DecCoins" json:"unclaimed_rewards"`
}

func (m *Record) Reset()         { *m = Record{} }
func (m *Record) String() string { return proto.CompactTextString(m) }
func (*Record) ProtoMessage()    {}
func (*Record) Descriptor() ([]byte, []int) {
	return fileDescriptor_4866f7c74a169dc2, []int{1}
}
func (m *Record) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *Record) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_Record.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *Record) XXX_Merge(src proto.Message) {
	xxx_messageInfo_Record.Merge(m, src)
}
func (m *Record) XXX_Size() int {
	return m.Size()
}
func (m *Record) XXX_DiscardUnknown() {
	xxx_messageInfo_Record.DiscardUnknown(m)
}

var xxx_messageInfo_Record proto.InternalMessageInfo

func (m *Record) GetInitAccumValue() github_com_cosmos_cosmos_sdk_types.DecCoins {
	if m != nil {
		return m.InitAccumValue
	}
	return nil
}

func (m *Record) GetUnclaimedRewards() github_com_cosmos_cosmos_sdk_types.DecCoins {
	if m != nil {
		return m.UnclaimedRewards
	}
	return nil
}

func init() {
	proto.RegisterType((*AccumulatorContent)(nil), "osmosis.accum.v1beta1.AccumulatorContent")
	proto.RegisterType((*Record)(nil), "osmosis.accum.v1beta1.Record")
}

func init() { proto.RegisterFile("osmosis/accum/v1beta1/accum.proto", fileDescriptor_4866f7c74a169dc2) }

var fileDescriptor_4866f7c74a169dc2 = []byte{
	// 350 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0xb4, 0x92, 0x4f, 0x4b, 0x02, 0x41,
	0x18, 0x87, 0x77, 0x15, 0x04, 0x47, 0x08, 0x5b, 0x0a, 0x44, 0x62, 0x35, 0x0f, 0x21, 0x84, 0x3b,
	0x98, 0x9f, 0x40, 0xed, 0xd2, 0xa1, 0xcb, 0x06, 0x1d, 0xba, 0x2c, 0xb3, 0xb3, 0x83, 0x0e, 0xed,
	0xce, 0xc8, 0xfc, 0x31, 0x22, 0xe8, 0xdc, 0xb1, 0xcf, 0xd1, 0x27, 0xf1, 0xe8, 0xa1, 0x43, 0x74,
	0xb0, 0xd0, 0x2f, 0x12, 0x3b, 0xb3, 0x8a, 0xc7, 0x2e, 0x9e, 0xde, 0x79, 0x67, 0x7e, 0xf3, 0x3c,
	0xbc, 0xf0, 0x82, 0x73, 0x2e, 0x33, 0x2e, 0xa9, 0x84, 0x08, 0x63, 0x9d, 0xc1, 0x79, 0x3f, 0x26,
	0x0a, 0xf5, 0x6d, 0x17, 0xcc, 0x04, 0x57, 0xdc, 0x3b, 0x2d, 0x22, 0x81, 0xbd, 0x2c, 0x22, 0xcd,
	0x93, 0x09, 0x9f, 0x70, 0x93, 0x80, 0xf9, 0xc9, 0x86, 0x9b, 0x3e, 0x36, 0x69, 0x18, 0x23, 0x49,
	0x76, 0x34, 0xcc, 0x29, 0xb3, 0xef, 0x9d, 0x37, 0x17, 0x78, 0xc3, 0x9c, 0xa3, 0x53, 0xa4, 0xb8,
	0x18, 0x73, 0xa6, 0x08, 0x53, 0x9e, 0x00, 0x35, 0x43, 0x8f, 0xe6, 0x28, 0xd5, 0xa4, 0xe1, 0xb6,
	0xcb, 0xdd, 0xda, 0xd5, 0x59, 0x60, 0x61, 0x41, 0x0e, 0xdb, 0x7a, 0x83, 0x6b, 0x82, 0xc7, 0x9c,
	0xb2, 0xd1, 0x60, 0xb1, 0x6a, 0x39, 0x1f, 0x3f, 0xad, 0xcb, 0x09, 0x55, 0x53, 0x1d, 0x07, 0x98,
	0x67, 0xb0, 0x90, 0xdb, 0xd2, 0x93, 0xc9, 0x23, 0x54, 0xcf, 0x33, 0x22, 0xb7, 0x7f, 0x64, 0x08,
	0x8c, 0xe5, 0x3e, 0x97, 0x74, 0x3e, 0x4b, 0xa0, 0x12, 0x12, 0xcc, 0x45, 0xe2, 0xdd, 0x02, 0xc0,
	0x74, 0x16, 0xc9, 0x29, 0x12, 0x44, 0x36, 0xdc, 0xb6, 0xdb, 0xad, 0x8e, 0x82, 0x9c, 0xff, 0xbd,
	0x6a, 0x5d, 0xfc, 0x8f, 0x1f, 0x56, 0x99, 0xce, 0xee, 0x0c, 0xc0, 0x7b, 0x01, 0x75, 0xca, 0xa8,
	0x8a, 0xf6, 0x47, 0x2a, 0x1d, 0x6a, 0xa4, 0xa3, 0x5c, 0x35, 0xdc, 0x8d, 0xe5, 0xbd, 0x82, 0x63,
	0xcd, 0x70, 0x8a, 0x68, 0x46, 0x92, 0x48, 0x90, 0x27, 0x24, 0x12, 0xd9, 0x28, 0x1f, 0xca, 0x5e,
	0xdf, 0xb9, 0x42, 0xab, 0x1a, 0xdd, 0x2c, 0xd6, 0xbe, 0xbb, 0x5c, 0xfb, 0xee, 0xef, 0xda, 0x77,
	0xdf, 0x37, 0xbe, 0xb3, 0xdc, 0xf8, 0xce, 0xd7, 0xc6, 0x77, 0x1e, 0xe0, 0x1e, 0xb8, 0xd8, 0xa9,
	0x5e, 0x8a, 0x62, 0xb9, 0x6d, 0x4c, 0xd5, 0x8a, 0xa6, 0xc5, 0x36, 0xc6, 0x15, 0xb3, 0x33, 0x83,
	0xbf, 0x00, 0x00, 0x00, 0xff, 0xff, 0xf1, 0xc5, 0x7b, 0x28, 0xa5, 0x02, 0x00, 0x00,
}

func (m *AccumulatorContent) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *AccumulatorContent) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *AccumulatorContent) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.AccumValue) > 0 {
		for iNdEx := len(m.AccumValue) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.AccumValue[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintAccum(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0xa
		}
	}
	return len(dAtA) - i, nil
}

func (m *Record) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *Record) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *Record) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if len(m.UnclaimedRewards) > 0 {
		for iNdEx := len(m.UnclaimedRewards) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.UnclaimedRewards[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintAccum(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0x1a
		}
	}
	if len(m.InitAccumValue) > 0 {
		for iNdEx := len(m.InitAccumValue) - 1; iNdEx >= 0; iNdEx-- {
			{
				size, err := m.InitAccumValue[iNdEx].MarshalToSizedBuffer(dAtA[:i])
				if err != nil {
					return 0, err
				}
				i -= size
				i = encodeVarintAccum(dAtA, i, uint64(size))
			}
			i--
			dAtA[i] = 0x12
		}
	}
	{
		size := m.NumShares.Size()
		i -= size
		if _, err := m.NumShares.MarshalTo(dAtA[i:]); err != nil {
			return 0, err
		}
		i = encodeVarintAccum(dAtA, i, uint64(size))
	}
	i--
	dAtA[i] = 0xa
	return len(dAtA) - i, nil
}

func encodeVarintAccum(dAtA []byte, offset int, v uint64) int {
	offset -= sovAccum(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *AccumulatorContent) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	if len(m.AccumValue) > 0 {
		for _, e := range m.AccumValue {
			l = e.Size()
			n += 1 + l + sovAccum(uint64(l))
		}
	}
	return n
}

func (m *Record) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = m.NumShares.Size()
	n += 1 + l + sovAccum(uint64(l))
	if len(m.InitAccumValue) > 0 {
		for _, e := range m.InitAccumValue {
			l = e.Size()
			n += 1 + l + sovAccum(uint64(l))
		}
	}
	if len(m.UnclaimedRewards) > 0 {
		for _, e := range m.UnclaimedRewards {
			l = e.Size()
			n += 1 + l + sovAccum(uint64(l))
		}
	}
	return n
}

func sovAccum(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozAccum(x uint64) (n int) {
	return sovAccum(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *AccumulatorContent) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowAccum
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: AccumulatorContent: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: AccumulatorContent: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field AccumValue", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAccum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthAccum
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthAccum
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.AccumValue = append(m.AccumValue, types.DecCoin{})
			if err := m.AccumValue[len(m.AccumValue)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipAccum(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthAccum
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func (m *Record) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowAccum
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: Record: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: Record: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field NumShares", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAccum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthAccum
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthAccum
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			if err := m.NumShares.Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 2:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field InitAccumValue", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAccum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthAccum
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthAccum
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.InitAccumValue = append(m.InitAccumValue, types.DecCoin{})
			if err := m.InitAccumValue[len(m.InitAccumValue)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		case 3:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field UnclaimedRewards", wireType)
			}
			var msglen int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowAccum
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				msglen |= int(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if msglen < 0 {
				return ErrInvalidLengthAccum
			}
			postIndex := iNdEx + msglen
			if postIndex < 0 {
				return ErrInvalidLengthAccum
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.UnclaimedRewards = append(m.UnclaimedRewards, types.DecCoin{})
			if err := m.UnclaimedRewards[len(m.UnclaimedRewards)-1].Unmarshal(dAtA[iNdEx:postIndex]); err != nil {
				return err
			}
			iNdEx = postIndex
		default:
			iNdEx = preIndex
			skippy, err := skipAccum(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthAccum
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipAccum(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowAccum
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowAccum
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
		case 1:
			iNdEx += 8
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowAccum
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if length < 0 {
				return 0, ErrInvalidLengthAccum
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupAccum
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthAccum
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthAccum        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowAccum          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupAccum = fmt.Errorf("proto: unexpected end of group")
)