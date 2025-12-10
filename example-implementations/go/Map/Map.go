package mapp

import "hash/fnv"

type Entry[K comparable, V comparable] struct {
	Key   K
	Value V
	Next  *Entry[K, V]
}

type Map[K comparable, V comparable] struct {
	length   int
	capacity int
	buckets  []*Entry[K, V]
}

func (m *Map[K, V]) Len() int { return m.length }

func NewMap[K comparable, V comparable](capacity int) *Map[K, V] {
	return &Map[K, V]{
		length:   0,
		capacity: capacity,
		buckets:  make([]*Entry[K, V], capacity),
	}
}

func (m *Map[K, V]) hash(key K) int {
	h := fnv.New32a()
	h.Write([]byte(toString(key)))
	return int(h.Sum32()) % m.capacity
}

func toString[K comparable](key K) string {
	return any(key).(string)
}

func (m *Map[K, V]) Get(key K) (V, bool) {
	var zero V
	idx := m.hash(key)
	entry := m.buckets[idx]

	for entry != nil {
		if entry.Key == key {
			return entry.Value, true
		}
		entry = entry.Next
	}

	return zero, false
}

func (m *Map[K, V]) Set(key K, value V) {
	idx := m.hash(key)
	entry := m.buckets[idx]

	for entry != nil {
		if entry.Key == key {
			entry.Value = value
			return
		}
		entry = entry.Next
	}

	newEntry := &Entry[K, V]{
		Key:   key,
		Value: value,
		Next:  m.buckets[idx],
	}
	m.buckets[idx] = newEntry
	m.length++
}

func (m *Map[K, V]) Delete(key K) (V, bool) {
	var zero V
	idx := m.hash(key)
	entry := m.buckets[idx]
	var prev *Entry[K, V]

	for entry != nil {
		if entry.Key == key {
			if prev == nil {
				m.buckets[idx] = entry.Next
			} else {
				prev.Next = entry.Next
			}
			m.length--
			return entry.Value, true
		}
		prev = entry
		entry = entry.Next
	}

	return zero, false
}
