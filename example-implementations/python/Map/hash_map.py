"""
Hash Map implementation using chaining for collision resolution.
"""

from typing import TypeVar, Generic, Optional, List, Any
from dataclasses import dataclass

K = TypeVar('K')
V = TypeVar('V')


@dataclass
class Entry(Generic[K, V]):
    """Entry in the hash map's bucket chain."""
    key: K
    value: V
    next: Optional['Entry[K, V]'] = None


class HashMap(Generic[K, V]):
    """Hash map implementation using chaining."""

    def __init__(self, capacity: int = 16):
        self._length = 0
        self._capacity = capacity
        self._buckets: List[Optional[Entry[K, V]]] = [None] * capacity

    def __len__(self) -> int:
        return self._length

    def _hash(self, key: K) -> int:
        """Hash function to map keys to bucket indices."""
        return hash(key) % self._capacity

    def get(self, key: K) -> Optional[V]:
        """Get value for key. Returns None if key not found."""
        idx = self._hash(key)
        entry = self._buckets[idx]

        while entry is not None:
            if entry.key == key:
                return entry.value
            entry = entry.next

        return None

    def set(self, key: K, value: V) -> None:
        """Set value for key. Updates if key exists, inserts if new."""
        idx = self._hash(key)
        entry = self._buckets[idx]

        # Check if key already exists and update
        while entry is not None:
            if entry.key == key:
                entry.value = value
                return
            entry = entry.next

        # Key doesn't exist, insert new entry at head of chain
        new_entry = Entry(key=key, value=value, next=self._buckets[idx])
        self._buckets[idx] = new_entry
        self._length += 1

    def delete(self, key: K) -> Optional[V]:
        """Delete key and return its value. Returns None if key not found."""
        idx = self._hash(key)
        entry = self._buckets[idx]
        prev: Optional[Entry[K, V]] = None

        while entry is not None:
            if entry.key == key:
                if prev is None:
                    self._buckets[idx] = entry.next
                else:
                    prev.next = entry.next
                self._length -= 1
                return entry.value

            prev = entry
            entry = entry.next

        return None
