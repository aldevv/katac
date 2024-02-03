package lru

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestLRU(t *testing.T) {
	lru := NewLRU[string, int](3)

	result, ok := lru.Get("foo")
	assert.False(t, ok, "Expected: false, got: %v", ok)

	lru.Update("foo", 69)
	result, ok = lru.Get("foo")
	assert.True(t, ok, "Expected: true, got: %v", ok)
	assert.Equal(t, 69, result, "Expected: 69, got: %v", result)

	lru.Update("bar", 420)
	result, ok = lru.Get("bar")
	assert.True(t, ok, "Expected: true, got: %v", ok)
	assert.Equal(t, 420, result, "Expected: 420, got: %v", result)
	//
	lru.Update("baz", 1337)
	result, ok = lru.Get("baz")
	assert.True(t, ok, "Expected: true, got: %v", ok)
	assert.Equal(t, 1337, result, "Expected: 1337, got: %v", result)
	//
	lru.Update("ball", 69420)
	result, ok = lru.Get("ball")
	assert.True(t, ok, "Expected: true, got: %v", ok)
	assert.Equal(t, 69420, result, "Expected: 69420, got: %v", result)
	//
	result, ok = lru.Get("foo")
	assert.False(t, ok, "Expected: false, got: %v", ok)
	//
	result, ok = lru.Get("bar")
	assert.True(t, ok, "Expected: true, got: %v", ok)
	assert.Equal(t, 420, result, "Expected: 420, got: %v", result)
	//
	lru.Update("foo", 69)
	result, ok = lru.Get("bar")
	assert.True(t, ok, "Expected: true, got: %v", ok)
	assert.Equal(t, 420, result, "Expected: 420, got: %v", result)

	result, ok = lru.Get("foo")
	assert.True(t, ok, "Expected: true, got: %v", ok)
	assert.Equal(t, 69, result, "Expected: 69, got: %v", result)

	result, ok = lru.Get("baz")
	assert.False(t, ok, "Expected: false, got: %v", ok)
}
