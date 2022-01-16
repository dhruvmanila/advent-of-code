// Package counter implements a generic counter for counting items.
package counter

import (
	"fmt"
	"math"
	"strings"
)

// Counter is a generic counter for counting items.
type Counter[T comparable] map[T]int

// New creates and returns a new counter, optionally with the given items.
func New[T comparable](items ...T) Counter[T] {
	return NewFromSlice(items)
}

// NewFromMap creates and returns a new counter from an existing map.
// This creates a copy of the given map which induces a runtime cost.
func NewFromMap[T comparable](m map[T]int) Counter[T] {
	mc := make(map[T]int, len(m))
	for item, count := range m {
		mc[item] = count
	}
	return mc
}

// NewFromSlice creates and returns a new counter from an existing slice.
func NewFromSlice[T comparable](sl []T) Counter[T] {
	c := make(Counter[T], len(sl))
	c.Increment(sl...)
	return c
}

// Increment increments the count of all the given items by 1, else initiating
// the count to 1.
func (c Counter[T]) Increment(items ...T) {
	for _, item := range items {
		c.IncrementBy(item, 1)
	}
}

// IncrementBy is used to add count for an item if it exists, else initiating the
// item with given count.
func (c Counter[T]) IncrementBy(item T, count int) {
	if c.contains(item) {
		c[item] += count
	} else {
		c[item] = count
	}
}

// Decrement decrements the count of all the given items by 1. This won't let
// the count of an item be negative. Use counter.DecrementBy() to decrement the
// count below 0.
func (c Counter[T]) Decrement(items ...T) {
	for _, item := range items {
		if c.Get(item) > 0 {
			c.DecrementBy(item, 1)
		}
	}
}

// DecrementBy is used to subtract count for an item if it exists, else
// initiating the item with given count. The count can be reduced to zero
// or negative.
func (c Counter[T]) DecrementBy(item T, count int) {
	if c.contains(item) {
		c[item] -= count
	} else {
		c[item] = -count
	}
}

// Delete deletes all the items from the counter completely. To decrement the
// count of an item, use counter.Decrement() or counter.DecrementBy() instead.
func (c Counter[T]) Delete(items ...T) {
	for _, item := range items {
		delete(c, item)
	}
}

// Update updates the current counter with the counts from the other counter.
func (c Counter[T]) Update(other Counter[T]) {
	for item, count := range other {
		if c.contains(item) {
			c[item] += count
		} else {
			c[item] = count
		}
	}
}

// Get is used to get the count for an item, 0 if the item does not exists.
func (c Counter[T]) Get(item T) int {
	if c.contains(item) {
		return c[item]
	}
	return 0
}

// MostCommon is used to get the most common (highest count) item. Use
// Counter.Get() to get the count of that item.
func (c Counter[T]) MostCommon() T {
	var i T
	max := math.MinInt
	for item, count := range c {
		if count > max {
			max = count
			i = item
		}
	}
	return i
}

// LeastCommon is used to get the least common (lowest count) item. Use
// Counter.Get() to get the count of that item.
func (c Counter[T]) LeastCommon() T {
	var i T
	min := math.MaxInt
	for item, count := range c {
		if count < min {
			min = count
			i = item
		}
	}
	return i
}

// Len returns the number of elements in the counter.
func (c Counter[T]) Len() int {
	return len(c)
}

// Total returns the total count of all the elements in the counter.
func (c Counter[T]) Total() int {
	total := 0
	for _, count := range c {
		total += count
	}
	return total
}

// ForEach is used to iterate over every item of the counter by calling a
// user-defined function with every item and its count.
func (c Counter[T]) ForEach(f func(item T, count int)) {
	for item, count := range c {
		f(item, count)
	}
}

// Iter is used to iterate over every item of the counter. It returns a
// receive-only buffered channel whose size is half of the counter length.
//
// This can be used as:
//
//   for item := range counter.iter {
//     count := counter.Get(item)
//     // do something with item and count
//   }
func (c Counter[T]) Iter() <-chan T {
	// Use a buffered channel to avoid blocking the main goroutine.
	ch := make(chan T, c.Len()/2)
	go func() {
		for item := range c {
			ch <- item
		}
		close(ch)
	}()
	return ch
}

func (c Counter[T]) String() string {
	counts := make([]string, 0, c.Len())
	for item, count := range c {
		counts = append(counts, fmt.Sprintf("%v:%d", item, count))
	}
	return fmt.Sprintf("Counter{%s}", strings.Join(counts, " "))
}

// Internal method to ease up checking whether an item exists in the counter.
func (c Counter[T]) contains(item T) bool {
	_, exist := c[item]
	return exist
}
