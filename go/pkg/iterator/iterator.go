// Package iterator implements a generic iterator protocol.
package iterator

type Iterator[T any] struct {
	idx  int
	data []T
}

// New creates a new iterator for the given slice of elements.
func New[T any](data []T) *Iterator[T] {
	return &Iterator[T]{data: data, idx: -1}
}

// Next returns true if there are any elements remaining to iterate, false
// otherwise.
func (it *Iterator[T]) Next() bool {
	it.idx++
	return it.idx < len(it.data)
}

// Value returns the element at the current iterator index.
func (it *Iterator[T]) Value() T {
	return it.data[it.idx]
}

// Reset resets the iterator index.
func (it *Iterator[T]) Reset() {
	it.idx = -1
}
