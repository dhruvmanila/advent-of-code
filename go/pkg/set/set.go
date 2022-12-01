// Package set implements a set of any comparable type.
//
// A set is an unordered list of elements with no duplicates.
package set

import (
	"fmt"
	"strings"
)

var exist = struct{}{}

// Set represents an unordered list of elements.
type Set[T comparable] map[T]struct{}

// New create and returns an initialized Set, optionally with the given elements.
func New[T comparable](es ...T) Set[T] {
	return NewFromSlice(es)
}

// NewWithSize create and returns an initialized and empty set with a given
// size.
func NewWithSize[T comparable](size int) Set[T] {
	return make(Set[T], size)
}

// NewFromSlice create and returns a new set from an existing slice.
func NewFromSlice[T comparable](sl []T) Set[T] {
	s := NewWithSize[T](len(sl))
	s.Add(sl...)
	return s
}

// Add adds all the given elements to the set. If any element is already in
// the set, Add is a no-op for that element.
func (s Set[T]) Add(es ...T) {
	for _, e := range es {
		s[e] = exist
	}
}

// Remove deletes all the given elements from the set. If there is no such
// element, Remove is a no-op.
func (s Set[T]) Remove(es ...T) {
	for _, e := range es {
		delete(s, e)
	}
}

// Pop removes and returns an arbitrary item from the set. If the set is empty,
// then it will return the zero value for the constrained type.
func (s Set[T]) Pop() T {
	for e := range s {
		s.Remove(e)
		return e
	}
	var e T // zero value of type T
	return e
}

// Contains check if the given element exists in the set.
func (s Set[T]) Contains(e T) bool {
	_, c := s[e]
	return c
}

// Clear removes all the elements from the set.
func (s *Set[T]) Clear() {
	*s = make(Set[T])
}

// Len returns the number of elements in the set.
func (s Set[T]) Len() int {
	return len(s)
}

// ForEach is used to iterate over every element of the set by calling a
// user-defined function with every element.
func (s Set[T]) ForEach(f func(e T)) {
	for e := range s {
		f(e)
	}
}

// Iter is used to iterate over every element of the set. It returns a
// receive-only buffered channel whose size is half of set length.
func (s Set[T]) Iter() <-chan T {
	// Use a buffered channel to avoid blocking the main goroutine.
	ch := make(chan T, s.Len()/2)
	go func() {
		for e := range s {
			ch <- e
		}
		close(ch)
	}()
	return ch
}

// Union returns a new set with elements from s and other.
func (s Set[T]) Union(other Set[T]) Set[T] {
	n := New[T]()
	for e := range s {
		n.Add(e)
	}
	for e := range other {
		n.Add(e)
	}
	return n
}

// Intersection returns a new set with elements common to s and other.
func (s Set[T]) Intersection(other Set[T]) Set[T] {
	n := New[T]()
	// Loop over the smaller set.
	if s.Len() < other.Len() {
		other, s = s, other
	}
	for e := range other {
		if s.Contains(e) {
			n.Add(e)
		}
	}
	return n
}

// Difference returns a new set with elements in s that are not in other.
func (s Set[T]) Difference(other Set[T]) Set[T] {
	n := New[T]()
	for e := range s {
		if other.Contains(e) {
			continue
		}
		n.Add(e)
	}
	return n
}

// SymmetricDifference returns a new set with elements in either s or other
// but not in both.
func (s Set[T]) SymmetricDifference(other Set[T]) Set[T] {
	return s.Difference(other).Union(other.Difference(s))
}

// IsSubset returns true if every element in s is in other, false otherwise.
func (s Set[T]) IsSubset(other Set[T]) bool {
	if s.Len() > other.Len() {
		return false
	}
	for e := range s {
		if !other.Contains(e) {
			return false
		}
	}
	return true
}

// IsProperSubset returns true if s is a proper subset of other, that is, s is
// a subset of other and length of s is strictly less than other.
func (s Set[T]) IsProperSubset(other Set[T]) bool {
	return s.IsSubset(other) && s.Len() < other.Len()
}

// IsSuperset returns true if every element in other is in s, false otherwise.
func (s Set[T]) IsSuperset(other Set[T]) bool {
	return other.IsSubset(s)
}

// IsProperSuperset returns true if s is a proper superset of other, that is,
// s is a superset of other and length of s is strictly greater than other.
func (s Set[T]) IsProperSuperset(other Set[T]) bool {
	return s.IsSuperset(other) && s.Len() > other.Len()
}

// IsDisjoint returns true if s has no elements in common with other. Sets are
// disjoint if and only if their intersection is an empty set.
func (s Set[T]) IsDisjoint(other Set[T]) bool {
	return s.Intersection(other).Len() == 0
}

// IsEqual returns true if s is equal to other. Sets are equal if their lengths
// are equal and every element of s is in other.
func (s Set[T]) IsEqual(other Set[T]) bool {
	if s.Len() != other.Len() {
		return false
	}
	for e := range s {
		if !other.Contains(e) {
			return false
		}
	}
	return true
}

// ToSlice returns the members of the receiver set as a slice.
func (s Set[T]) ToSlice() []T {
	slice := make([]T, 0, s.Len())
	for e := range s {
		slice = append(slice, e)
	}
	return slice
}

func (s Set[T]) String() string {
	items := make([]string, 0, s.Len())
	for e := range s {
		items = append(items, fmt.Sprintf("%v", e))
	}
	return fmt.Sprintf("Set[%s]", strings.Join(items, " "))
}
