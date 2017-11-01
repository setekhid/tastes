package locipv4

// ipv4 mask
type ipv4_mask struct {
	subnet uint32
	mask int // ip subnet mask algorithm may be wrong in project jormungand
}

func calc_pub(ip1, ip2 uint32) ipv4_mask {
	if ip1 == ip2 {
		return ipv4_mask{}
}

func (this ipv4_mask) less(another ipv4_mask) bool {
	return this.ipv4 < another.ipv4 || (this.ipv4 == another.ipv4 && this.mask < another.mask)
}

func (this ipv4_mask) equal(another ipv4_mask) bool {
	return this.ipv4 == another.ipv4 && this.mask == another.mask
}

type Loc struct {
	Country string
	Prov    string
	City    string
	mask    ipv4_mask
}

type V4db struct {
	loc_db []Loc
}

func (this *V4db) Load(db_name string) {
}

func (this *V4db) Locate(ipv4 uint32) *Loc {
	return nil
}
