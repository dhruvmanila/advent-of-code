// Package counter implements a generic counter for counting items.
package counter

import (
	"math"
)

// Counter is a generic counter for counting items.
type Counter struct {
	m map[interface{}]int
}

// New creates and returns a new, empty counter.
func New() *Counter {
	return &Counter{m: make(map[interface{}]int)}
}

// NewWith creates and returns a new counter with the given items.
func NewWith(items ...interface{}) *Counter {
	return NewFromSlice(items)
}

// NewFromMap creates and returns a new counter from an existing map.
// This creates a copy of the given map which induces a runtime cost.
func NewFromMap(m map[interface{}]int) *Counter {
	mc := make(map[interface{}]int, len(m))
	for item, count := range m {
		mc[item] = count
	}
	return &Counter{m: mc}
}

// NewFromSlice creates and returns a new counter from an existing slice.
func NewFromSlice(s []interface{}) *Counter {
	c := New()
	c.Add(s...)
	return c
}

// Add adds all the items to the counter increasing the count for an existing
// item, else initiating the count to 1.
func (c *Counter) Add(items ...interface{}) {
	for _, item := range items {
		c.AddCount(item, 1)
	}
}

// Remove removes all the items from the counter by decreasing its count by 1.
// This does not actually remove an item from the counter. Use counter.Delete()
// to remove it entirely.
//
// Contrary to counter.SubtractCount(), this won't let the count of an item to
// be negative. If the count of an item is 0 or negative and Remove is called
// for that item, this function will be a no-op.
func (c *Counter) Remove(items ...interface{}) {
	for _, item := range items {
		if c.Get(item) > 0 {
			c.SubtractCount(item, 1)
		}
	}
}

// Delete deletes all the items from the counter completely. To decrement the
// count of an item, use counter.Remove() instead.
func (c *Counter) Delete(items ...interface{}) {
	for _, item := range items {
		delete(c.m, item)
	}
}

// AddCount is used to add count for an item if it exists, else initiating the
// item with given count.
func (c *Counter) AddCount(item interface{}, count int) {
	if c.contains(item) {
		c.m[item] += count
	} else {
		c.m[item] = count
	}
}

// SubtractCount is used to subtract count for an item if it exists, else
// initiating the item with given count. The count can be reduced to zero
// or negative.
func (c *Counter) SubtractCount(item interface{}, count int) {
	if c.contains(item) {
		c.m[item] -= count
	} else {
		c.m[item] = -count
	}
}

// Update updates the current counter with the counts from the other counter.
func (c *Counter) Update(other *Counter) {
	for item, count := range other.m {
		if c.contains(item) {
			c.m[item] += count
		} else {
			c.m[item] = count
		}
	}
}

// Get is used to get the count for an item, 0 if the item does not exists.
func (c *Counter) Get(item interface{}) int {
	if c.contains(item) {
		return c.m[item]
	}
	return 0
}

// MostCommon is used to get the most common (highest count) item. Use
// Counter.Get() to get the count of that item.
func (c *Counter) MostCommon() interface{} {
	var i interface{}
	max := math.MinInt
	for item, count := range c.m {
		if count > max {
			max = count
			i = item
		}
	}
	return i
}

// LeastCommon is used to get the least common (lowest count) item. Use
// Counter.Get() to get the count of that item.
func (c *Counter) LeastCommon() interface{} {
	var i interface{}
	min := math.MaxInt
	for item, count := range c.m {
		if count < min {
			min = count
			i = item
		}
	}
	return i
}

// Len returns the number of elements in the counter.
func (c *Counter) Len() int {
	return len(c.m)
}

// Total returns the total count of all the elements in the counter.
func (c *Counter) Total() int {
	total := 0
	for _, count := range c.m {
		total += count
	}
	return total
}

// Items returns a map of item to its count. This will return a copy of the
// internal map and so mutating it won't affect the internal state of the
// counter. This adds a runtime overhead because of the copying step.
//
// This function can be used directly with the range keyword as follows:
//
//   for item, count := range counter.Items() {
//     // do something with item and count
//   }
func (c *Counter) Items() map[interface{}]int {
	mc := make(map[interface{}]int, c.Len())
	for item, count := range c.m {
		mc[item] = count
	}
	return mc
}

// FIXME: Make Counter thread safe
// This will be used as:
//   for item := range counter.iter {
//     count := counter.Get(item)
//     // do something with item and count
//   }
func (c *Counter) iter() <-chan interface{} {
	ch := make(chan interface{})
	go func() {
		for item := range c.m {
			ch <- item
		}
		close(ch)
	}()
	return ch
}

// Internal method to ease up checking whether an item exists in the counter.
func (c *Counter) contains(item interface{}) bool {
	_, exist := c.m[item]
	return exist
}
