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
        # TODO: Initialize data array

    def __len__(self) -> int:
        return self._length

    def push(self, item: T) -> None:
        """Add item to buffer. Overwrites oldest item if full."""
        pass

    def pop(self) -> Optional[T]:
        """Remove and return oldest item. Returns None if empty."""
        pass

    def get(self, idx: int) -> Optional[T]:
        """Get item at logical index. Returns None if out of bounds."""
        pass

    def is_full(self) -> bool:
        """Check if buffer is at capacity."""
        return self._length == self._capacity

    def is_empty(self) -> bool:
        """Check if buffer is empty."""
        return self._length == 0
