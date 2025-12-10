package arraylist

type ArrayList[T comparable] struct {
	length int
	data   []T
}

func (l *ArrayList[T]) Len() int { return l.length }

func NewArrayList[T comparable](capacity int) *ArrayList[T] {
	return &ArrayList[T]{
		length: 0,
		data:   make([]T, capacity),
	}
}

func (l *ArrayList[T]) grow() {
	newCap := len(l.data) * 2
	if newCap == 0 {
		newCap = 1
	}
	newData := make([]T, newCap)
	copy(newData, l.data)
	l.data = newData
}

func (l *ArrayList[T]) Prepend(item T) {
	if l.length >= len(l.data) {
		l.grow()
	}
	copy(l.data[1:], l.data[0:l.length])
	l.data[0] = item
	l.length++
}

func (l *ArrayList[T]) InsertAt(item T, idx int) {
	if idx < 0 || idx > l.length {
		return
	}
	if l.length >= len(l.data) {
		l.grow()
	}
	copy(l.data[idx+1:], l.data[idx:l.length])
	l.data[idx] = item
	l.length++
}

func (l *ArrayList[T]) Append(item T) {
	if l.length >= len(l.data) {
		l.grow()
	}
	l.data[l.length] = item
	l.length++
}

func (l *ArrayList[T]) Remove(item T) (T, bool) {
	var zero T
	for i := 0; i < l.length; i++ {
		if l.data[i] == item {
			val := l.data[i]
			copy(l.data[i:], l.data[i+1:l.length])
			l.length--
			l.data[l.length] = zero
			return val, true
		}
	}
	return zero, false
}

func (l *ArrayList[T]) Get(idx int) (T, bool) {
	var zero T
	if idx < 0 || idx >= l.length {
		return zero, false
	}
	return l.data[idx], true
}

func (l *ArrayList[T]) RemoveAt(idx int) (T, bool) {
	var zero T
	if idx < 0 || idx >= l.length {
		return zero, false
	}
	val := l.data[idx]
	copy(l.data[idx:], l.data[idx+1:l.length])
	l.length--
	l.data[l.length] = zero
	return val, true
}
