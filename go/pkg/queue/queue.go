package queue

import "fmt"

// Queue represents a simple queue data structure. It is backed by a slice of
// an unconstrained type T.
type Queue[T any] []T

// New returns an initialized queue, optionally with the given elements. The
// elements are added in the same order as provided.
func New[T any](es ...T) *Queue[T] {
	q := new(Queue[T])
	if es != nil {
		q.Enqueue(es...)
	}
	return q
}

// Enqueue is used to enqueue all the given elements to the queue. Multiple
// elements are added in the same order as provided.
func (q *Queue[T]) Enqueue(es ...T) {
	*q = append(*q, es...)
}

// Dequeue is used to dequeue or remove the frontmost element from the queue
// and return it.
//
// An attempt to dequeue when the queue is empty will return the zero value for
// the type of the elements in the queue. Using multiple assignment, one can
// distinguish a missing entry from a zero value. This is referred to as the
// "comma ok" idiom.
func (q *Queue[T]) Dequeue() (e T, ok bool) {
	sl := *q
	if len(sl) == 0 {
		return e, false
	}
	e = sl[0]
	if len(sl) == 1 {
		*q = nil // Clear the slice
	} else {
		*q = sl[1:]
	}
	return e, true
}

// Peek returns the frontmost element of the queue without removing it.
//
// An attempt to peek when the queue is empty will return the zero value for
// the type of the elements in the queue. Using multiple assignment, one can
// distinguish a missing entry from a zero value. This is referred to as the
// "comma ok" idiom.
func (q *Queue[T]) Peek() (e T, ok bool) {
	sl := *q
	if len(sl) == 0 {
		return e, false
	}
	return sl[0], true
}

// Len returns the number of elements in the queue.
func (q *Queue[T]) Len() int {
	return len(*q)
}

// IsEmpty is used to check whether the queue is empty or not.
func (q *Queue[T]) IsEmpty() bool {
	return q.Len() == 0
}

// ToSlice returns a slice containing the elements of the queue where the first
// element is the start of the queue. Mutating the returned slice will not
// affect the underlying implementation.
func (q *Queue[T]) ToSlice() []T {
	sl := make([]T, q.Len())
	copy(sl, *q)
	return sl
}

func (q *Queue[T]) String() string {
	return fmt.Sprintf("Queue%v", *q)
}
