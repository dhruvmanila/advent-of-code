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

// Len returns the remaining number of items to be iterated over.
func (it *Iterator[T]) Len() int {
	if it.idx >= len(it.data) {
		return 0
	}
	return len(it.data[it.idx+1:])
}

// Next returns true if there are any elements remaining to iterate, false
// otherwise.
func (it *Iterator[T]) Next() bool {
	it.idx++
	return it.idx < len(it.data)
}

// Value returns the element at the current iterator index. Next must have
// been called prior to a call to Value. If the iterator is exhausted, which
// is saying the the current index is out of bounds for the data slice, it
// will return the zero value for the type T.
func (it *Iterator[T]) Value() T {
	if it.idx >= len(it.data) || it.idx < 0 {
		var v T
		return v
	}
	return it.data[it.idx]
}

// Move moves the iterator i number of elements forward.
func (it *Iterator[T]) Move(i int) {
	it.idx += i
}

// Reset resets the iterator index.
func (it *Iterator[T]) Reset() {
	it.idx = -1
}
