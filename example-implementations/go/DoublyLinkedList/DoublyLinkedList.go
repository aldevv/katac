package doublylinkedlist

type Node[T comparable] struct {
	Value T
	Prev  *Node[T]
	Next  *Node[T]
}

type DoublyLinkedList[T comparable] struct {
	length int
	head   *Node[T]
	tail   *Node[T]
}

func (l *DoublyLinkedList[T]) Len() int { return l.length }

func NewDoublyLinkedList[T comparable]() *DoublyLinkedList[T] {
	return &DoublyLinkedList[T]{
		length: 0,
		head:   nil,
		tail:   nil,
	}
}

func (l *DoublyLinkedList[T]) Prepend(item T) {
	node := &Node[T]{Value: item}

	if l.head == nil {
		l.head = node
		l.tail = node
	} else {
		node.Next = l.head
		l.head.Prev = node
		l.head = node
	}
	l.length++
}

func (l *DoublyLinkedList[T]) InsertAt(item T, idx int) {
	if idx < 0 || idx > l.length {
		return
	}

	if idx == 0 {
		l.Prepend(item)
		return
	}

	if idx == l.length {
		l.Append(item)
		return
	}

	curr := l.head
	for i := 0; i < idx; i++ {
		curr = curr.Next
	}

	node := &Node[T]{Value: item}
	node.Next = curr
	node.Prev = curr.Prev
	curr.Prev.Next = node
	curr.Prev = node
	l.length++
}

func (l *DoublyLinkedList[T]) Append(item T) {
	node := &Node[T]{Value: item}

	if l.tail == nil {
		l.head = node
		l.tail = node
	} else {
		node.Prev = l.tail
		l.tail.Next = node
		l.tail = node
	}
	l.length++
}

func (l *DoublyLinkedList[T]) Remove(item T) (T, bool) {
	var zero T
	curr := l.head

	for curr != nil {
		if curr.Value == item {
			if curr.Prev != nil {
				curr.Prev.Next = curr.Next
			} else {
				l.head = curr.Next
			}

			if curr.Next != nil {
				curr.Next.Prev = curr.Prev
			} else {
				l.tail = curr.Prev
			}

			l.length--
			return curr.Value, true
		}
		curr = curr.Next
	}

	return zero, false
}

func (l *DoublyLinkedList[T]) Get(idx int) (T, bool) {
	var zero T
	if idx < 0 || idx >= l.length {
		return zero, false
	}

	curr := l.head
	for i := 0; i < idx; i++ {
		curr = curr.Next
	}

	return curr.Value, true
}

func (l *DoublyLinkedList[T]) RemoveAt(idx int) (T, bool) {
	var zero T
	if idx < 0 || idx >= l.length {
		return zero, false
	}

	curr := l.head
	for i := 0; i < idx; i++ {
		curr = curr.Next
	}

	if curr.Prev != nil {
		curr.Prev.Next = curr.Next
	} else {
		l.head = curr.Next
	}

	if curr.Next != nil {
		curr.Next.Prev = curr.Prev
	} else {
		l.tail = curr.Prev
	}

	l.length--
	return curr.Value, true
}
