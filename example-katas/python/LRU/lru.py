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
        # TODO: Initialize head, tail, and lookup

    def __len__(self) -> int:
        return self._length

    def get(self, key: K) -> Optional[V]:
        """Get value for key. Returns None if key not found."""
        pass

    def update(self, key: K, value: V) -> None:
        """Update or insert key-value pair. Evicts LRU item if at capacity."""
        pass
