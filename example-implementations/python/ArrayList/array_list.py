"""
ArrayList implementation using a dynamic array.
Supports generic types through Python's type system.
"""

from typing import TypeVar, Optional, Generic, Any
import sys
import os

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from dsa_base import List

T = TypeVar('T')


class ArrayList(List[T], Generic[T]):
    """Dynamic array-based list implementation."""

    def __init__(self, capacity: int = 10):
        self._length = 0
        self._data: list[Any] = [None] * capacity

    def __len__(self) -> int:
        return self._length

    def _grow(self) -> None:
        """Double the capacity of the internal array."""
        new_capacity = len(self._data) * 2 if len(self._data) > 0 else 1
        new_data: list[Any] = [None] * new_capacity
        for i in range(self._length):
            new_data[i] = self._data[i]
        self._data = new_data

    def prepend(self, item: T) -> None:
        """Add item to the beginning of the list."""
        if self._length >= len(self._data):
            self._grow()

        # Shift all elements to the right
        for i in range(self._length, 0, -1):
            self._data[i] = self._data[i - 1]

        self._data[0] = item
        self._length += 1

    def insert_at(self, item: T, idx: int) -> None:
        """Insert item at the given index."""
        if idx < 0 or idx > self._length:
            return

        if self._length >= len(self._data):
            self._grow()

        # Shift elements from idx to the right
        for i in range(self._length, idx, -1):
            self._data[i] = self._data[i - 1]

        self._data[idx] = item
        self._length += 1

    def append(self, item: T) -> None:
        """Add item to the end of the list."""
        if self._length >= len(self._data):
            self._grow()

        self._data[self._length] = item
        self._length += 1

    def remove(self, item: T) -> Optional[T]:
        """Remove first occurrence of item. Returns the item if found, None otherwise."""
        for i in range(self._length):
            if self._data[i] == item:
                val = self._data[i]
                # Shift elements left
                for j in range(i, self._length - 1):
                    self._data[j] = self._data[j + 1]
                self._length -= 1
                self._data[self._length] = None
                return val
        return None

    def get(self, idx: int) -> Optional[T]:
        """Get item at index. Returns None if index is out of bounds."""
        if idx < 0 or idx >= self._length:
            return None
        return self._data[idx]

    def remove_at(self, idx: int) -> Optional[T]:
        """Remove item at index. Returns the item if found, None otherwise."""
        if idx < 0 or idx >= self._length:
            return None

        val = self._data[idx]
        # Shift elements left
        for i in range(idx, self._length - 1):
            self._data[i] = self._data[i + 1]
        self._length -= 1
        self._data[self._length] = None
        return val
