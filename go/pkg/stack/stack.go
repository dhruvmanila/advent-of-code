// Package stack implements a generic stack data structure.
package stack

import (
	"container/list"
)

// Stack represents the stack data structure.
type Stack struct {
	list *list.List
}

// New returns an initialized and empty stack.
func New() *Stack {
	return &Stack{list: list.New()}
}

// Push adds a value to the top of a stack.
func (s *Stack) Push(v interface{}) {
	s.list.PushFront(v)
}

// Pop removes the top element on the stack and returns it, or nil if the stack
// is empty.
func (s *Stack) Pop() interface{} {
	if e := s.list.Front(); e != nil {
		s.list.Remove(e)
		return e.Value
	}
	return nil
}

// Peek returns the top element on the stack without removing it, or nil if the
// stack is empty.
func (s *Stack) Peek() interface{} {
	if e := s.list.Front(); e != nil {
		return e.Value
	}
	return nil
}

// Len returns the number of elements on the stack.
func (s *Stack) Len() int {
	return s.list.Len()
}

// IsEmpty is used to check whether the stack is empty or not.
func (s *Stack) IsEmpty() bool {
	return s.Len() == 0
}
