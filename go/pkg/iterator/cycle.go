package iterator

type Cycle[T any] struct {
	Iterator[T]
}

// NewCycle creates a new cycle iterator for the given slice of elements.
func NewCycle[T any](data []T) *Cycle[T] {
	return &Cycle[T]{
		Iterator: *New(data),
	}
}

// Next increments the iterator index. It always returns true as it resets
// the index to cycle around.
func (it *Cycle[T]) Next() bool {
	if it.idx >= len(it.data)-1 {
		it.Reset()
	}
	return it.Iterator.Next()
}

// Value returns the element at the current iterator index. Next must have
// been called prior to a call to Value.
func (it *Cycle[T]) Value() T {
	// No need to check for out of bounds as the Next method resets once
	// the iterator is exhausted.
	return it.data[it.idx]
}
