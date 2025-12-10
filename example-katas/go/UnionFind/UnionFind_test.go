package unionfind

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestUnionFind(t *testing.T) {
	uf := NewUnionFind(10)

	assert.Equal(t, 10, uf.Count(), "Initial count should be 10")

	assert.False(t, uf.Connected(0, 1), "0 and 1 should not be connected initially")

	uf.Union(0, 1)
	assert.True(t, uf.Connected(0, 1), "0 and 1 should be connected after union")
	assert.Equal(t, 9, uf.Count(), "Count should be 9 after one union")

	uf.Union(1, 2)
	assert.True(t, uf.Connected(0, 2), "0 and 2 should be connected")
	assert.True(t, uf.Connected(1, 2), "1 and 2 should be connected")
	assert.Equal(t, 8, uf.Count(), "Count should be 8 after two unions")

	uf.Union(3, 4)
	assert.True(t, uf.Connected(3, 4), "3 and 4 should be connected")
	assert.False(t, uf.Connected(0, 3), "0 and 3 should not be connected")
	assert.Equal(t, 7, uf.Count(), "Count should be 7 after three unions")

	uf.Union(0, 4)
	assert.True(t, uf.Connected(0, 4), "0 and 4 should be connected")
	assert.True(t, uf.Connected(1, 3), "1 and 3 should be connected through the union")
	assert.True(t, uf.Connected(2, 4), "2 and 4 should be connected through the union")
	assert.Equal(t, 6, uf.Count(), "Count should be 6 after merging two components")

	root0 := uf.Find(0)
	root1 := uf.Find(1)
	root2 := uf.Find(2)
	root3 := uf.Find(3)
	root4 := uf.Find(4)

	assert.Equal(t, root0, root1, "0 and 1 should have the same root")
	assert.Equal(t, root0, root2, "0 and 2 should have the same root")
	assert.Equal(t, root0, root3, "0 and 3 should have the same root")
	assert.Equal(t, root0, root4, "0 and 4 should have the same root")

	assert.False(t, uf.Connected(5, 6), "5 and 6 should not be connected")
	assert.Equal(t, 6, uf.Count(), "Count should still be 6")
}
