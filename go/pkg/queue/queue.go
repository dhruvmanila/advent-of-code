package queue

import "container/list"

// Queue represents a simple queue data structure.
type Queue struct {
	list *list.List
}

// New returns an initialized and empty simple queue.
func New() *Queue {
	return &Queue{list: list.New()}
}

// Enqueue is used to enqueue an element to the queue.
func (q *Queue) Enqueue(e interface{}) {
	q.list.PushBack(e)
}

// Dequeue is used to dequeue or remove the frontmost element from the queue
// and returns it, or nil if the queue is empty.
func (q *Queue) Dequeue() interface{} {
	if e := q.list.Front(); e != nil {
		q.list.Remove(e)
		return e.Value
	}
	return nil
}

// Peek returns the frontmost element of the queue without removing it, or nil
// if the queue is empty.
func (q *Queue) Peek() interface{} {
	if e := q.list.Front(); e != nil {
		return e.Value
	}
	return nil
}

// Len returns the number of elements in the queue.
func (q *Queue) Len() int {
	return q.list.Len()
}

// IsEmpty is used to check whether the queue is empty or not.
func (q *Queue) IsEmpty() bool {
	return q.Len() == 0
}
