package stack

import "testing"

func TestStackNew(t *testing.T) {
	s := New()
	if s == nil {
		t.Fatal("failed to create a new stack")
	}

	if length := s.Len(); length != 0 {
		t.Errorf("empty stack; expected length: 0, actual: %d\n", length)
	}
}

func TestStackPush(t *testing.T) {
	s := New()

	s.Push(1)
	if length := s.Len(); length != 1 {
		t.Errorf("s.Push(1); expected length: 1, actual: %d\n", length)
	}

	s.Push(4)
	s.Push(7)
	if length := s.Len(); length != 3 {
		t.Errorf("s.Push(1); expected length: 3, actual: %d\n", length)
	}
}

func TestStackPop(t *testing.T) {
	s := New()

	if e := s.Pop(); e != nil {
		t.Errorf("s.Pop() empty stack; expected: nil, actual: %v\n", e)
	}

	s.Push(1)
	e := s.Pop().(int)
	if length := s.Len(); length != 0 {
		t.Errorf("s.Pop(); expected length: 0, actual: %d\n", length)
	}
	if e != 1 {
		t.Errorf("s.Pop(); expected element: 1, actual: %v\n", e)
	}

	s.Push(2)
	s.Push(4)
	s.Push(8)

	e1 := s.Pop().(int)
	e2 := s.Pop().(int)
	if length := s.Len(); length != 1 {
		t.Errorf("s.Peek(); expected length: 1, actual: %d\n", length)
	}
	if e1 != 8 || e2 != 4 {
		t.Errorf("s.Pop(); expected e1=8 e2=4, actual e1=%v e2=%v\n", e1, e2)
	}

	e3 := s.Pop().(int)
	if e3 != 2 {
		t.Errorf("s.Pop(); expected element: 2, actual %v\n", e3)
	}
	if length := s.Len(); length != 0 {
		t.Errorf("s.Peek(); expected length: 0, actual: %d\n", length)
	}
}

func TestStackPeek(t *testing.T) {
	s := New()

	if e := s.Peek(); e != nil {
		t.Errorf("s.Peek() empty stack; expected: nil, actual: %v\n", e)
	}

	s.Push(1)

	if e := s.Peek().(int); e != 1 {
		t.Errorf("s.Peek(); expected element: 1, actual: %v\n", e)
	}

	s.Push(2)
	s.Push(4)
	if e := s.Peek().(int); e != 4 {
		t.Errorf("s.Peek(); expected element: 4, actual: %v\n", e)
	}
}

func TestStackEmpty(t *testing.T) {
	s := New()

	if b := s.IsEmpty(); b != true {
		t.Errorf("s.IsEmpty() empty stack; expected: true, actual: %v\n", b)
	}

	s.Push(1)
	if b := s.IsEmpty(); b != false {
		t.Errorf("s.IsEmpty(); expected: false, actual: %v\n", b)
	}
}
