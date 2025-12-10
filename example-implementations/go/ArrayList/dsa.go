package arraylist

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

// graph

type GraphEdge struct {
	To     int
	Weight int
}

type (
	WeightedAdjacencyList   [][]GraphEdge
	WeightedAdjacencyMatrix [][]int
)

var AdjList1 WeightedAdjacencyList = WeightedAdjacencyList{
	{
		{To: 1, Weight: 3},
		{To: 2, Weight: 1},
	},
	{
		{To: 0, Weight: 3},
		{To: 2, Weight: 4},
		{To: 4, Weight: 1},
	},
	{
		{To: 1, Weight: 4},
		{To: 3, Weight: 7},
		{To: 0, Weight: 1},
	},
	{
		{To: 2, Weight: 7},
		{To: 4, Weight: 5},
		{To: 6, Weight: 1},
	},
	{
		{To: 1, Weight: 1},
		{To: 3, Weight: 5},
		{To: 5, Weight: 2},
	},
	{
		{To: 6, Weight: 1},
		{To: 4, Weight: 2},
		{To: 2, Weight: 18},
	},
	{
		{To: 3, Weight: 1},
		{To: 5, Weight: 1},
	},
}

var AdjList2 WeightedAdjacencyList = WeightedAdjacencyList{
	{
		{To: 1, Weight: 3},
		{To: 2, Weight: 1},
	},
	{
		{To: 4, Weight: 1},
	},
	{
		{To: 3, Weight: 7},
	},
	{},
	{
		{To: 1, Weight: 1},
		{To: 3, Weight: 5},
		{To: 5, Weight: 2},
	},
	{
		{To: 2, Weight: 18},
		{To: 6, Weight: 1},
	},
	{
		{To: 3, Weight: 1},
	},
}

var AdjMatrix1 WeightedAdjacencyMatrix = WeightedAdjacencyMatrix{
	{0, 3, 1, 0, 0, 0, 0},
	{0, 0, 0, 0, 1, 0, 0},
	{0, 0, 7, 0, 0, 0, 0},
	{0, 0, 0, 0, 0, 0, 0},
	{0, 1, 0, 5, 0, 2, 0},
	{0, 0, 18, 0, 0, 0, 1},
	{0, 0, 0, 1, 0, 0, 1},
}

// helpers
func ZeroValue[T any]() T {
	var zero T
	return zero
}

// list
type List[T comparable] interface {
	Len() int

	Prepend(item T)
	InsertAt(item T, idx int)
	Append(item T)
	Remove(item T) (T, bool)
	Get(idx int) (T, bool)
	RemoveAt(idx int) (T, bool)
}

func TestList(t *testing.T, list List[int]) {
	t.Helper()

	// append

	list.Append(5)
	list.Append(7)
	list.Append(9)

	val, ok := list.Get(2)
	assert.Equal(t, 9, val, "Value should be 9, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")

	val, ok = list.RemoveAt(1)
	assert.Equal(t, 7, val, "Value should be 7, but is %d", val)
	assert.True(t, ok, "RemoveAt should return true, but returned false")
	assert.Equal(t, 2, list.Len(), "Length should be 2, but is %d", list.Len())

	// remove

	list.Append(11)
	val, ok = list.RemoveAt(1)
	assert.Equal(t, 9, val, "Value should be 9, but is %d", val)
	assert.True(t, ok, "RemoveAt should return true, but returned false")

	val, ok = list.Remove(9)
	assert.Equal(t, 0, val, "Value should be 0, but is %d", val)
	assert.False(t, ok, "Remove should return false, but returned true")

	val, ok = list.RemoveAt(0)
	assert.Equal(t, 5, val, "Value should be 5, but is %d", val)
	assert.True(t, ok, "RemoveAt should return true, but returned false")

	val, ok = list.RemoveAt(0)
	assert.Equal(t, 11, val, "Value should be 11, but is %d", val)
	assert.True(t, ok, "RemoveAt should return true, but returned false")
	assert.Equal(t, 0, list.Len(), "Length should be 0, but is %d", list.Len())

	// prepend

	list.Prepend(5)
	list.Prepend(7)
	list.Prepend(9)

	val, ok = list.Get(2)
	assert.Equal(t, 5, val, "Value should be 5, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")

	val, ok = list.Get(0)
	assert.Equal(t, 9, val, "Value should be 9, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")

	val, ok = list.Remove(9)
	assert.Equal(t, 9, val, "Value should be 9, but is %d", val)
	assert.True(t, ok, "Remove should return true, but returned false")

	assert.Equal(t, 2, list.Len(), "Length should be 2, but is %d", list.Len())

	val, ok = list.Get(0)
	assert.Equal(t, 7, val, "Value should be 7, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")

	// insert

	list.InsertAt(10, 1)

	val, ok = list.Get(1)
	assert.Equal(t, 10, val, "Value should be 10, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")

	val, ok = list.Get(2)
	assert.Equal(t, 5, val, "Value should be 5, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")

	list.InsertAt(20, 2)
	val, ok = list.Get(2)
	assert.Equal(t, 20, val, "Value should be 20, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")
	val, ok = list.Get(3)
	assert.Equal(t, 5, val, "Value should be 5, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")

	list.InsertAt(30, 4)
	val, ok = list.Get(4)
	assert.Equal(t, 30, val, "Value should be 30, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")
	val, ok = list.Get(3)
	assert.Equal(t, 5, val, "Value should be 5, but is %d", val)
	assert.True(t, ok, "Get should return true, but returned false")
}

// maze
type Point struct {
	X int
	Y int
}

// tree
type BinaryNode[T comparable] struct {
	Value T
	Left  *BinaryNode[T]
	Right *BinaryNode[T]
}

var Tree1 BinaryNode[int] = BinaryNode[int]{
	Value: 20,
	Right: &BinaryNode[int]{
		Value: 50,
		Right: &BinaryNode[int]{
			Value: 100,
			Right: nil,
			Left:  nil,
		},
		Left: &BinaryNode[int]{
			Value: 30,
			Right: &BinaryNode[int]{
				Value: 45,
				Right: nil,
				Left:  nil,
			},
			Left: &BinaryNode[int]{
				Value: 29,
				Right: nil,
				Left:  nil,
			},
		},
	},
	Left: &BinaryNode[int]{
		Value: 10,
		Right: &BinaryNode[int]{
			Value: 15,
			Right: nil,
			Left:  nil,
		},
		Left: &BinaryNode[int]{
			Value: 5,
			Right: &BinaryNode[int]{
				Value: 7,
				Right: nil,
				Left:  nil,
			},
			Left: nil,
		},
	},
}

var Tree2 BinaryNode[int] = BinaryNode[int]{
	Value: 20,
	Right: &BinaryNode[int]{
		Value: 50,
		Right: nil,
		Left: &BinaryNode[int]{
			Value: 30,
			Right: &BinaryNode[int]{
				Value: 45,
				Right: &BinaryNode[int]{
					Value: 49,
					Left:  nil,
					Right: nil,
				},
				Left: nil,
			},
			Left: &BinaryNode[int]{
				Value: 29,
				Right: nil,
				Left: &BinaryNode[int]{
					Value: 21,
					Right: nil,
					Left:  nil,
				},
			},
		},
	},
	Left: &BinaryNode[int]{
		Value: 10,
		Right: &BinaryNode[int]{
			Value: 15,
			Right: nil,
			Left:  nil,
		},
		Left: &BinaryNode[int]{
			Value: 5,
			Right: &BinaryNode[int]{
				Value: 7,
				Right: nil,
				Left:  nil,
			},
			Left: nil,
		},
	},
}
