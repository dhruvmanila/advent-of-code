package set

import (
	"testing"
)

func TestSetNew(t *testing.T) {
	s := New[int]()
	if s == nil {
		t.Fatal("failed to create a new set")
	}

	if length := s.Len(); length != 0 {
		t.Errorf("empty set; expected length: 0, actual: %v\n", length)
	}

	s = New(1, 2, 3)
	if length := s.Len(); length != 3 {
		t.Errorf("new set with elements; expected length: 3, actual: %v\n", length)
	}

	s = NewFromSlice([]int{1, 2})
	if length := s.Len(); length != 2 {
		t.Errorf("new set from slice; expected length: 2, actual: %v\n", length)
	}
}

func TestSetContains(t *testing.T) {
	s := New(1, 2)

	if actual := s.Contains(1); actual != true {
		t.Errorf("set contains element; actual: %v\n", actual)
	}

	if actual := s.Contains(3); actual != false {
		t.Errorf("set does not contain element; actual: %v\n", actual)
	}
}

func TestSetAdd(t *testing.T) {
	s := New[int]()

	s.Add(1)
	if !s.Contains(1) {
		t.Error("set add failed to add element")
	}

	s.Add(2, 3)
	if !s.Contains(2) || !s.Contains(3) {
		t.Error("set add failed to add multiple elements")
	}

	s.Add(2, 3, 3, 4)
	if s.Len() != 4 {
		t.Error("set add repeated elements added multiple times")
	}
	if !s.Contains(4) {
		t.Error("set add failed to add element")
	}
}

func TestSetRemove(t *testing.T) {
	s := New(1, 2, 3)

	s.Remove(4)
	if s.Len() != 3 {
		t.Error("set remove deleted non-existing element")
	}

	s.Remove(1)
	if s.Contains(1) {
		t.Error("set failed to remove a single element")
	}

	s.Remove(1, 2, 3)
	if s.Contains(2) || s.Contains(3) {
		t.Error("set failed to remove multiple elements")
	}
}

func TestSetClear(t *testing.T) {
	s := New(1, 2, 3)

	s.Clear()
	if s.Len() != 0 {
		t.Error("failed to clear set")
	}
}

func TestSetUnion(t *testing.T) {
	s1 := New[int]()
	s2 := New[int]()

	if s := s1.Union(s2); s.Len() != 0 {
		t.Errorf("union of empty sets contains elements: %v\n", s)
	}

	s1.Add(1, 2, 3)
	if s := s1.Union(s2); !s.Contains(1) || !s.Contains(2) || !s.Contains(3) {
		t.Errorf("failed to union sets: %v\n", s)
	}

	s2.Add(2, 3, 4)
	if s := s1.Union(s2); s.Len() != 4 {
		t.Errorf("union of sets with common elements: %v\n", s)
	}
}
