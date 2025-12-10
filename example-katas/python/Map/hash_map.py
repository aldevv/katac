"""
Hash Map implementation using chaining for collision resolution.
"""

from typing import TypeVar, Generic, Optional
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
        # TODO: Initialize buckets

    def __len__(self) -> int:
        return self._length

    def _hash(self, key: K) -> int:
        """Hash function to map keys to bucket indices."""
        return hash(key) % self._capacity

    def get(self, key: K) -> Optional[V]:
        """Get value for key. Returns None if key not found."""
        pass

    def set(self, key: K, value: V) -> None:
        """Set value for key. Updates if key exists, inserts if new."""
        pass

    def delete(self, key: K) -> Optional[V]:
        """Delete key and return its value. Returns None if key not found."""
        pass
