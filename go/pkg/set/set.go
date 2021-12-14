// Package set implements a set which is an unordered list of elements.
//
// This is a generic implementation where each value is contained within an
// Element struct similar to that of the standard library container/list. To
// access the underlying value, use the Value method.
package set

type empty struct{}

// Element is an element of a set.
type Element struct {
	// The value stored with this element.
	Value interface{}
}

// Set represents an unordered list of elements.
type Set struct {
	list map[*Element]empty
}

// New returns an initialized Set.
func New() *Set {
	return &Set{list: make(map[*Element]empty)}
}

// Add adds the value to the set.
func (s *Set) Add(value interface{}) {
	s.list[&Element{Value: value}] = empty{}
}

// Contains check if the given value exists in the set.
func (s *Set) Contains(value interface{}) bool {
	_, exist := s.list[&Element{Value: value}]
	return exist
}

// Remove deletes the value from the set. If there is no such value, Remove is
// a no-op.
func (s *Set) Remove(value interface{}) {
	delete(s.list, &Element{Value: value})
}

// Clear empties the set.
func (s *Set) Clear() {
	s.list = make(map[*Element]empty)
}

// Len returns the number of elements in the set.
func (s *Set) Len() int {
	return len(s.list)
}

// Union returns the union of s with other as a new set.
func (s *Set) Union(other *Set) *Set {
	n := New()
	for e := range s.list {
		n.Add(e.Value)
	}
	for e := range other.list {
		n.Add(e.Value)
	}
	return n
}

// Intersection returns an intersection of s with other as a new set.
func (s *Set) Intersection(other *Set) *Set {
	n := New()
	for e := range s.list {
		if other.Contains(e.Value) {
			n.Add(e.Value)
		}
	}
	return n
}

// Difference returns the difference of s with other as a new set.
func (s *Set) Difference(other *Set) *Set {
	n := New()
	for e := range s.list {
		if other.Contains(e.Value) {
			continue
		}
		n.Add(e.Value)
	}
	return n
}
