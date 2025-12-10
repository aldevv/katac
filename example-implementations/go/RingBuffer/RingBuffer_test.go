package ringbuffer

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRingBuffer(t *testing.T) {
	rb := NewRingBuffer[int](3)

	assert.True(t, rb.IsEmpty(), "RingBuffer should be empty initially")
	assert.False(t, rb.IsFull(), "RingBuffer should not be full initially")

	rb.Push(1)
	rb.Push(2)
	rb.Push(3)

	assert.True(t, rb.IsFull(), "RingBuffer should be full after 3 pushes")
	assert.Equal(t, 3, rb.Len(), "Length should be 3")

	val, ok := rb.Get(0)
	assert.True(t, ok, "Get(0) should return true")
	assert.Equal(t, 1, val, "Get(0) should return 1")

	val, ok = rb.Get(2)
	assert.True(t, ok, "Get(2) should return true")
	assert.Equal(t, 3, val, "Get(2) should return 3")

	rb.Push(4)
	assert.True(t, rb.IsFull(), "RingBuffer should still be full")
	assert.Equal(t, 3, rb.Len(), "Length should still be 3")

	val, ok = rb.Get(0)
	assert.True(t, ok, "Get(0) should return true")
	assert.Equal(t, 2, val, "Get(0) should now return 2 (oldest was overwritten)")

	val, ok = rb.Pop()
	assert.True(t, ok, "Pop should return true")
	assert.Equal(t, 2, val, "Pop should return 2")
	assert.Equal(t, 2, rb.Len(), "Length should be 2 after pop")

	val, ok = rb.Pop()
	assert.True(t, ok, "Pop should return true")
	assert.Equal(t, 3, val, "Pop should return 3")

	val, ok = rb.Pop()
	assert.True(t, ok, "Pop should return true")
	assert.Equal(t, 4, val, "Pop should return 4")

	assert.True(t, rb.IsEmpty(), "RingBuffer should be empty after all pops")

	val, ok = rb.Pop()
	assert.False(t, ok, "Pop on empty buffer should return false")
}
