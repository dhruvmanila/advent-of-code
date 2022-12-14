package queue

// An Item is something to be managed in a priority queue.
type Item struct {
	// The value of the item; arbitrary.
	Value any

	// The priority of the item in the queue.
	Priority int
}

// A PriorityQueue implements heap.Interface and holds Items. This is a
// min-heap, so the item with the lowest priority is popped first.
type PriorityQueue []*Item

func (pq PriorityQueue) Len() int           { return len(pq) }
func (pq PriorityQueue) Less(i, j int) bool { return pq[i].Priority < pq[j].Priority }
func (pq PriorityQueue) Swap(i, j int)      { pq[i], pq[j] = pq[j], pq[i] }

// IsEmpty is used to check whether the queue is empty or not (length == 0).
func (pq *PriorityQueue) IsEmpty() bool { return pq.Len() == 0 }

// Push pushes the value v in the queue.
func (pq *PriorityQueue) Push(v any) {
	item := v.(*Item)
	*pq = append(*pq, item)
}

// Pop removes and returns the minimum element (according to Item.Priority)
// from the queue.
func (pq *PriorityQueue) Pop() (v any) {
	old := *pq
	v, *pq = old[len(old)-1], old[:len(old)-1]
	return v
}
