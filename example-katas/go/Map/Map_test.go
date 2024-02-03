package mapp

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMap(t *testing.T) {
	mapp := NewMap[string, int](8)

	mapp.Set("foo", 55)
	assert.Equal(t, 1, mapp.Len(), "Expected: 1, got: %v", mapp.Len())

	mapp.Set("fool", 75)
	assert.Equal(t, 2, mapp.Len(), "Expected: 2, got: %v", mapp.Len())

	mapp.Set("foolish", 105)
	assert.Equal(t, 3, mapp.Len(), "Expected: 3, got: %v", mapp.Len())

	mapp.Set("bar", 69)
	assert.Equal(t, 4, mapp.Len(), "Expected: 4, got: %v", mapp.Len())

	result, ok := mapp.Get("bar")
	assert.True(t, ok, "Expected: true, got: %v", ok)
	assert.Equal(t, 69, result, "Expected: 69, got: %v", result)

	result, ok = mapp.Get("blaz")
	assert.False(t, ok, "Expected: false, got: %v", ok)

	mapp.Delete("barblarbr")
	assert.Equal(t, 4, mapp.Len(), "Expected: 4, got: %v", mapp.Len())

	_, ok = mapp.Delete("meh")
	assert.False(t, ok, "Delete should return false, but returned true")

	mapp.Set("meh", 420)
	assert.Equal(t, 5, mapp.Len(), "Expected: 5, got: %v", mapp.Len())

	result, ok = mapp.Get("meh")
	assert.True(t, ok, "Expected: true, got: %v", ok)
	assert.Equal(t, 420, result, "Expected: 420, got: %v", result)

	mapp.Delete("bar")
	assert.Equal(t, 4, mapp.Len(), "Expected: 4, got: %v", mapp.Len())

	result, ok = mapp.Get("bar")
	assert.False(t, ok, "Expected: false, got: %v", ok)

	mapp.Set("heh", 255)
	assert.Equal(t, 5, mapp.Len(), "Expected: 5, got: %v", mapp.Len())

	mapp.Set("doggo", 1020)
	assert.Equal(t, 6, mapp.Len(), "Expected: 6, got: %v", mapp.Len())

	mapp.Set("monst", 1020)
	assert.Equal(t, 7, mapp.Len(), "Expected: 7, got: %v", mapp.Len())

	mapp.Set("oothe", 1)
	assert.Equal(t, 8, mapp.Len(), "Expected: 8, got: %v", mapp.Len())

	mapp.Set("other", 128)
	assert.Equal(t, 9, mapp.Len(), "Expected: 9, got: %v", mapp.Len())

	mapp.Set("some", 514)
	assert.Equal(t, 10, mapp.Len(), "Expected: 10, got: %v", mapp.Len())

	mapp.Set("same", 12)
	assert.Equal(t, 11, mapp.Len(), "Expected: 11, got: %v", mapp.Len())
	// assert.Equal(t, 16, mapp.capacity, "Expected: 16, got: %v", mapp.capacity)
	// s, _ := json.MarshalIndent(mapp.data, "", "\t")
	// fmt.Print(string(s))
}
