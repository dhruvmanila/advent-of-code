// Package counter implements a generic counter for counting items.
package counter

import (
	"math"
)

// Counter is a generic counter for counting items.
type Counter struct {
	m map[interface{}]int
}

// New creates a new, empty counter.
func New() *Counter {
	return &Counter{m: make(map[interface{}]int)}
}

// Add adds all the items to the counter increasing the count for an existing
// item, else initiating the count to 1.
func (c *Counter) Add(items ...interface{}) {
	for _, item := range items {
		if c.has(item) {
			c.m[item]++
		} else {
			c.m[item] = 1
		}
	}
}

// Update updates the current counter with the counts from the other counter.
func (c *Counter) Update(other *Counter) {
	for item, count := range other.m {
		if c.has(item) {
			c.m[item] += count
		} else {
			c.m[item] = count
		}
	}
}

// Get is used to get the count for an item, 0 if the item does not exists.
func (c *Counter) Get(item interface{}) int {
	if c.has(item) {
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

// Internal method to ease up checking whether the item exists in the counter.
func (c *Counter) has(item interface{}) bool {
	_, exist := c.m[item]
	return exist
}
