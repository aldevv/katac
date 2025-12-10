package ringbuffer

type RingBuffer[T any] struct {
	length   int
	capacity int
	head     int
	tail     int
	data     []T
}

func (r *RingBuffer[T]) Len() int { return r.length }

func NewRingBuffer[T any](capacity int) *RingBuffer[T] {
	return &RingBuffer[T]{
		length:   0,
		capacity: capacity,
		head:     0,
		tail:     0,
		data:     make([]T, capacity),
	}
}

func (r *RingBuffer[T]) Push(item T) {
	r.data[r.tail] = item
	r.tail = (r.tail + 1) % r.capacity

	if r.length < r.capacity {
		r.length++
	} else {
		r.head = (r.head + 1) % r.capacity
	}
}

func (r *RingBuffer[T]) Pop() (T, bool) {
	var zero T
	if r.length == 0 {
		return zero, false
	}

	val := r.data[r.head]
	r.head = (r.head + 1) % r.capacity
	r.length--

	return val, true
}

func (r *RingBuffer[T]) Get(idx int) (T, bool) {
	var zero T
	if idx < 0 || idx >= r.length {
		return zero, false
	}

	actualIdx := (r.head + idx) % r.capacity
	return r.data[actualIdx], true
}

func (r *RingBuffer[T]) IsFull() bool {
	return r.length == r.capacity
}

func (r *RingBuffer[T]) IsEmpty() bool {
	return r.length == 0
}
