package iterator

type String struct {
	idx int
	s   string
}

func NewString(s string) *String {
	return &String{s: s, idx: -1}
}

func (it *String) Next() bool {
	it.idx++
	return it.idx < len(it.s)
}

func (it *String) Value() string {
	return string(it.s[it.idx])
}

func (it *String) Remaining() string {
	if it.idx >= len(it.s) {
		return ""
	}
	idx := it.idx
	if idx == -1 {
		idx = 0
	}
	it.idx = len(it.s)
	return it.s[idx:]
}

func (it *String) Len() int {
	switch {
	case it.idx >= len(it.s):
		return 0
	case it.idx <= 0:
		return len(it.s)
	default:
		return len(it.s[it.idx:])
	}
}

func (it *String) Reset() {
	it.idx = -1
}
