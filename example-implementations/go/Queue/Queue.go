package queue

type Node[T any] struct {
	Value T
	Next  *Node[T]
}

type Queue[T any] struct {
	Length int
	head   *Node[T]
	tail   *Node[T]
}

func NewQueue[T any]() *Queue[T] {
	return &Queue[T]{
		Length: 0,
		head:   nil,
		tail:   nil,
	}
}

func (q *Queue[T]) Enqueue(item T) {
	node := &Node[T]{Value: item}

	if q.tail == nil {
		q.head = node
		q.tail = node
	} else {
		q.tail.Next = node
		q.tail = node
	}
	q.Length++
}

func (q *Queue[T]) Deque() (T, bool) {
	var zero T
	if q.head == nil {
		return zero, false
	}

	val := q.head.Value
	q.head = q.head.Next

	if q.head == nil {
		q.tail = nil
	}

	q.Length--
	return val, true
}

func (q *Queue[T]) Peek() (T, bool) {
	var zero T
	if q.head == nil {
		return zero, false
	}
	return q.head.Value, true
}
