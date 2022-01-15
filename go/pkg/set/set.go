// Package set implements a set which is an unordered list of elements with no
// duplicates.
package set

import (
	"fmt"
	"strings"
)

var exist = struct{}{}

// Set represents an unordered list of elements.
type Set struct {
	m map[interface{}]struct{}
}

// New create and returns a set, optionally with the given elements.
func New(sl ...interface{}) *Set {
	return NewFromSlice(sl)
}

// NewWithSize create and returns an initialized and empty set with a given
// size.
func NewWithSize(size int) *Set {
	return &Set{m: make(map[interface{}]struct{}, size)}
}

// NewFromSlice create and returns a new set from an existing slice.
func NewFromSlice(sl []interface{}) *Set {
	s := NewWithSize(len(sl))
	s.Add(sl...)
	return s
}

// Add adds all the given elements to the set.
func (s *Set) Add(es ...interface{}) {
	for _, e := range es {
		s.m[e] = exist
	}
}

// Remove deletes all the given elements from the set. If there is no such
// element, Remove is a no-op.
func (s *Set) Remove(es ...interface{}) {
	for _, e := range es {
		delete(s.m, e)
	}
}

// Pop removes and returns an arbitrary item from the set.
func (s *Set) Pop() interface{} {
	for e := range s.m {
		s.Remove(e)
		return e
	}
	return nil
}

// Contains check if the given element exists in the set.
func (s *Set) Contains(e interface{}) bool {
	_, c := s.m[e]
	return c
}

// Clear removes all the elements from the set.
func (s *Set) Clear() {
	s.m = make(map[interface{}]struct{})
}

// Len returns the number of elements in the set.
func (s *Set) Len() int {
	return len(s.m)
}

// ForEach is used to iterate over every element of the set by calling a
// user-defined function with every element.
func (s *Set) ForEach(f func(e interface{})) {
	for e := range s.m {
		f(e)
	}
}

// Iter is used to iterate over every element of the set. It returns a
// receive-only buffered channel whose size is half of set length.
func (s *Set) Iter() <-chan interface{} {
	// Use a buffered channel to avoid blocking the main goroutine.
	ch := make(chan interface{}, s.Len()/2)
	go func() {
		for e := range s.m {
			ch <- e
		}
		close(ch)
	}()
	return ch
}

// Union returns a new set with elements from s and other.
func (s *Set) Union(other *Set) *Set {
	n := New()
	for e := range s.m {
		n.Add(e)
	}
	for e := range other.m {
		n.Add(e)
	}
	return n
}

// Intersection returns a new set with elements common to s and other.
func (s *Set) Intersection(other *Set) *Set {
	n := New()
	// Loop over the smaller set.
	if s.Len() < other.Len() {
		for e := range s.m {
			if other.Contains(e) {
				n.Add(e)
			}
		}
	} else {
		for e := range other.m {
			if s.Contains(e) {
				n.Add(e)
			}
		}
	}
	return n
}

// Difference returns a new set with elements in s that are not in other.
func (s *Set) Difference(other *Set) *Set {
	n := New()
	for e := range s.m {
		if other.Contains(e) {
			continue
		}
		n.Add(e)
	}
	return n
}

// SymmetricDifference returns a new set with elements in either s or other
// but not in both.
func (s *Set) SymmetricDifference(other *Set) *Set {
	return s.Difference(other).Union(other.Difference(s))
}

// IsSubset returns true if every element in s is in other, false otherwise.
func (s *Set) IsSubset(other *Set) bool {
	if s.Len() > other.Len() {
		return false
	}
	for e := range s.m {
		if !other.Contains(e) {
			return false
		}
	}
	return true
}

// IsProperSubset returns true if s is a proper subset of other, that is, s is
// a subset of other and length of s is strictly less than other.
func (s *Set) IsProperSubset(other *Set) bool {
	return s.IsSubset(other) && s.Len() < other.Len()
}

// IsSuperset returns true if every element in other is in s, false otherwise.
func (s *Set) IsSuperset(other *Set) bool {
	return other.IsSubset(s)
}

// IsProperSuperset returns true if s is a proper superset of other, that is,
// s is a superset of other and length of s is strictly greater than other.
func (s *Set) IsProperSuperset(other *Set) bool {
	return s.IsSuperset(other) && s.Len() > other.Len()
}

// IsDisjoint returns true if s has no elements in common with other. Sets are
// disjoint if and only if their intersection is an empty set.
func (s *Set) IsDisjoint(other *Set) bool {
	return s.Intersection(other).Len() == 0
}

// IsEqual returns true if s is equal to other. Sets are equal if their lengths
// are equal and every element of s is in other.
func (s *Set) IsEqual(other *Set) bool {
	if s.Len() != other.Len() {
		return false
	}
	for e := range s.m {
		if !other.Contains(e) {
			return false
		}
	}
	return true
}

// ToSlice returns the members of the receiver set as a slice.
func (s *Set) ToSlice() []interface{} {
	slice := make([]interface{}, 0, s.Len())
	for e := range s.m {
		slice = append(slice, e)
	}
	return slice
}

func (s *Set) String() string {
	items := make([]string, 0, s.Len())
	for e := range s.m {
		items = append(items, fmt.Sprintf("%v", e))
	}
	return fmt.Sprintf("Set{%s}", strings.Join(items, ", "))
}
