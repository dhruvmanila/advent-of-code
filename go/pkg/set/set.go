// Package set implements a set which is an unordered list of elements.
//
// This is a generic implementation where each value is contained within an
// Element struct similar to that of the standard library container/list. To
// access the underlying value, use the Value method.
package set

type empty struct{}

// Set represents an unordered list of elements.
type Set struct {
	list map[interface{}]empty
}

// New returns an initialized Set.
func New() *Set {
	return &Set{list: make(map[interface{}]empty)}
}

// Add adds the value to the set.
func (s *Set) Add(v interface{}) {
	s.list[v] = empty{}
}

// Contains check if the given value exists in the set.
func (s *Set) Contains(v interface{}) bool {
	_, exist := s.list[v]
	return exist
}

// Remove deletes the value from the set. If there is no such value, Remove is
// a no-op.
func (s *Set) Remove(v interface{}) {
	delete(s.list, v)
}

// Clear empties the set.
func (s *Set) Clear() {
	s.list = make(map[interface{}]empty)
}

// Len returns the number of elements in the set.
func (s *Set) Len() int {
	return len(s.list)
}

// Union returns the union of s with other as a new set.
func (s *Set) Union(other *Set) *Set {
	n := New()
	for e := range s.list {
		n.Add(e)
	}
	for e := range other.list {
		n.Add(e)
	}
	return n
}

// Intersection returns an intersection of s with other as a new set.
func (s *Set) Intersection(other *Set) *Set {
	n := New()
	for e := range s.list {
		if other.Contains(e) {
			n.Add(e)
		}
	}
	return n
}

// Difference returns the difference of s with other as a new set.
func (s *Set) Difference(other *Set) *Set {
	n := New()
	for e := range s.list {
		if other.Contains(e) {
			continue
		}
		n.Add(e)
	}
	return n
}
