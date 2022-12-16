// Package stack implements a generic stack data structure.
package stack

import "fmt"

// Stack represents the stack data structure.
type Stack[T any] []T

// New returns an initialized stack, optionally with the given elements.
// The elements are pushed in the same order as provided.
func New[T any](es ...T) *Stack[T] {
	s := new(Stack[T])
	if es != nil {
		s.Push(es...)
	}
	return s
}

// Push adds an element to the top of a stack. Multiple elements
// are added in the same order as provided.
func (s *Stack[T]) Push(e ...T) {
	*s = append(*s, e...)
}

// Pop removes the top element on the stack and returns it, or nil if the stack
// is empty.
//
// An attempt to pop when the stack is empty will return the zero value for
// the type of the elements in the stack. Using multiple assignment, one can
// distinguish a missing entry from a zero value. This is referred to as the
// "comma ok" idiom.
func (s *Stack[T]) Pop() (e T, ok bool) {
	sl := *s
	if len(sl) == 0 {
		return e, false
	}
	e = sl[len(sl)-1]
	if len(sl) == 1 {
		*s = nil // Clear the slice
	} else {
		*s = sl[:len(sl)-1]
	}
	return e, true
}

// Peek returns the top element on the stack without removing it, or nil if the
// stack is empty.
//
// An attempt to peek when the stack is empty will return the zero value for
// the type of the elements in the stack. Using multiple assignment, one can
// distinguish a missing entry from a zero value. This is referred to as the
// "comma ok" idiom.
func (s *Stack[T]) Peek() (e T, ok bool) {
	sl := *s
	if len(sl) == 0 {
		return e, false
	}
	return sl[len(sl)-1], true
}

// Len returns the number of elements on the stack.
func (s *Stack[T]) Len() int {
	return len(*s)
}

// IsEmpty is used to check whether the stack is empty or not.
func (s *Stack[T]) IsEmpty() bool {
	return s.Len() == 0
}

func (s *Stack[T]) String() string {
	return fmt.Sprintf("Stack%v", *s)
}
