package ringbuffer

type RingBuffer[T any] struct {
	length int
}

func (r *RingBuffer[T]) Len() int { return r.length }

func NewRingBuffer[T any](capacity int) *RingBuffer[T] {

}

func (r *RingBuffer[T]) Push(item T) {

}

func (r *RingBuffer[T]) Pop() (T, bool) {

}

func (r *RingBuffer[T]) Get(idx int) (T, bool) {

}

func (r *RingBuffer[T]) IsFull() bool {

}

func (r *RingBuffer[T]) IsEmpty() bool {

}
