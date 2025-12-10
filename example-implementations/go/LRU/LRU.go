package lru

type Node[K comparable, V comparable] struct {
	Key   K
	Value V
	Prev  *Node[K, V]
	Next  *Node[K, V]
}

type LRU[K comparable, V comparable] struct {
	length   int
	capacity int
	head     *Node[K, V]
	tail     *Node[K, V]
	lookup   map[K]*Node[K, V]
}

func (l *LRU[K, V]) Len() int { return l.length }

func NewLRU[K comparable, V comparable](capacity int) *LRU[K, V] {
	return &LRU[K, V]{
		length:   0,
		capacity: capacity,
		head:     nil,
		tail:     nil,
		lookup:   make(map[K]*Node[K, V]),
	}
}

func (l *LRU[K, V]) detach(node *Node[K, V]) {
	if node.Prev != nil {
		node.Prev.Next = node.Next
	} else {
		l.head = node.Next
	}

	if node.Next != nil {
		node.Next.Prev = node.Prev
	} else {
		l.tail = node.Prev
	}
}

func (l *LRU[K, V]) prepend(node *Node[K, V]) {
	if l.head == nil {
		l.head = node
		l.tail = node
		node.Prev = nil
		node.Next = nil
		return
	}

	node.Next = l.head
	node.Prev = nil
	l.head.Prev = node
	l.head = node
}

func (l *LRU[K, V]) trimCache() {
	if l.length <= l.capacity {
		return
	}

	tail := l.tail
	l.detach(tail)

	delete(l.lookup, tail.Key)
	l.length--
}

func (l *LRU[K, V]) Get(key K) (V, bool) {
	var zero V
	node, exists := l.lookup[key]
	if !exists {
		return zero, false
	}

	l.detach(node)
	l.prepend(node)

	return node.Value, true
}

func (l *LRU[K, V]) Update(key K, value V) {
	node, exists := l.lookup[key]

	if exists {
		node.Value = value
		l.detach(node)
		l.prepend(node)
		return
	}

	node = &Node[K, V]{
		Key:   key,
		Value: value,
	}

	l.length++
	l.prepend(node)
	l.trimCache()

	l.lookup[key] = node
}
