"""
Ring Buffer (Circular Buffer) implementation.
Fixed-size buffer that wraps around when full.
"""

from typing import TypeVar, Generic, Optional, List, Any

T = TypeVar('T')


class RingBuffer(Generic[T]):
    """Circular buffer with fixed capacity."""

    def __init__(self, capacity: int):
        self._length = 0
        self._capacity = capacity
        self._head = 0
        self._tail = 0
        self._data: List[Any] = [None] * capacity

    def __len__(self) -> int:
        return self._length

    def push(self, item: T) -> None:
        """Add item to buffer. Overwrites oldest item if full."""
        self._data[self._tail] = item
        self._tail = (self._tail + 1) % self._capacity

        if self._length < self._capacity:
            self._length += 1
        else:
            # Buffer is full, move head forward (overwrite oldest)
            self._head = (self._head + 1) % self._capacity

    def pop(self) -> Optional[T]:
        """Remove and return oldest item. Returns None if empty."""
        if self._length == 0:
            return None

        val = self._data[self._head]
        self._head = (self._head + 1) % self._capacity
        self._length -= 1

        return val

    def get(self, idx: int) -> Optional[T]:
        """Get item at logical index. Returns None if out of bounds."""
        if idx < 0 or idx >= self._length:
            return None

        actual_idx = (self._head + idx) % self._capacity
        return self._data[actual_idx]

    def is_full(self) -> bool:
        """Check if buffer is at capacity."""
        return self._length == self._capacity

    def is_empty(self) -> bool:
        """Check if buffer is empty."""
        return self._length == 0
