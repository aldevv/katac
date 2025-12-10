"""
LRU (Least Recently Used) Cache implementation.
Combines hash map with doubly-linked list for O(1) operations.
"""

from typing import TypeVar, Generic, Optional, Dict
from dataclasses import dataclass

K = TypeVar('K')
V = TypeVar('V')


@dataclass
class Node(Generic[K, V]):
    """Node in the LRU cache's doubly-linked list."""
    key: K
    value: V
    prev: Optional['Node[K, V]'] = None
    next: Optional['Node[K, V]'] = None


class LRU(Generic[K, V]):
    """LRU cache with fixed capacity."""

    def __init__(self, capacity: int):
        self._length = 0
        self._capacity = capacity
        self._head: Optional[Node[K, V]] = None
        self._tail: Optional[Node[K, V]] = None
        self._lookup: Dict[K, Node[K, V]] = {}

    def __len__(self) -> int:
        return self._length

    def _detach(self, node: Node[K, V]) -> None:
        """Remove node from the linked list."""
        if node.prev is not None:
            node.prev.next = node.next
        else:
            self._head = node.next

        if node.next is not None:
            node.next.prev = node.prev
        else:
            self._tail = node.prev

    def _prepend(self, node: Node[K, V]) -> None:
        """Add node to the front of the list (most recently used)."""
        if self._head is None:
            self._head = node
            self._tail = node
            node.prev = None
            node.next = None
            return

        node.next = self._head
        node.prev = None
        self._head.prev = node
        self._head = node

    def _trim_cache(self) -> None:
        """Remove least recently used item if over capacity."""
        if self._length <= self._capacity:
            return

        tail = self._tail
        if tail is not None:
            self._detach(tail)
            del self._lookup[tail.key]
            self._length -= 1

    def get(self, key: K) -> Optional[V]:
        """Get value for key. Returns None if key not found."""
        if key not in self._lookup:
            return None

        node = self._lookup[key]
        self._detach(node)
        self._prepend(node)

        return node.value

    def update(self, key: K, value: V) -> None:
        """Update or insert key-value pair. Evicts LRU item if at capacity."""
        if key in self._lookup:
            node = self._lookup[key]
            node.value = value
            self._detach(node)
            self._prepend(node)
            return

        node = Node(key=key, value=value)
        self._length += 1
        self._prepend(node)
        self._trim_cache()
        self._lookup[key] = node
