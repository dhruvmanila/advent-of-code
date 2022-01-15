package queue

import "fmt"

// Queue represents a simple queue data structure.
type Queue []interface{}

// New returns an initialized queue, optionally with the given elements. The
// elements are added in the same order as provided.
func New(es ...interface{}) *Queue {
	q := new(Queue)
	if es != nil {
		q.Enqueue(es...)
	}
	return q
}

// Enqueue is used to enqueue all the given elements to the queue. Multiple
// elements are added in the same order as provided.
func (q *Queue) Enqueue(es ...interface{}) {
	*q = append(*q, es...)
}

// Dequeue is used to dequeue or remove the frontmost element from the queue
// and returns it, or nil if the queue is empty.
func (q *Queue) Dequeue() interface{} {
	sl := *q
	if len(sl) == 0 {
		return nil
	}
	e := sl[0]
	if len(sl) == 1 {
		// Clear the slice
		*q = nil
	} else {
		*q = sl[1:]
	}
	return e
}

// Peek returns the frontmost element of the queue without removing it, or nil
// if the queue is empty.
func (q *Queue) Peek() interface{} {
	sl := *q
	if len(sl) == 0 {
		return nil
	}
	return sl[0]
}

// Len returns the number of elements in the queue.
func (q *Queue) Len() int {
	return len(*q)
}

// IsEmpty is used to check whether the queue is empty or not.
func (q *Queue) IsEmpty() bool {
	return q.Len() == 0
}

// ToSlice returns a slice containing the elements of the queue where the first
// element is the start of the queue. Mutating the returned slice will not
// affect the underlying implementation.
func (q *Queue) ToSlice() []interface{} {
	sl := make([]interface{}, 0, q.Len())
	copy(sl, *q)
	return sl
}

func (q *Queue) String() string {
	return fmt.Sprintf("Queue%v", *q)
}
