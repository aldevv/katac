"""
Queue implementation using a linked list.
FIFO (First In, First Out) data structure.
"""

from typing import TypeVar, Optional, Generic
from dataclasses import dataclass

T = TypeVar('T')


@dataclass
class Node(Generic[T]):
    """Node in the queue's linked list."""
    value: T
    next: Optional['Node[T]'] = None


class Queue(Generic[T]):
    """Queue implementation using a singly-linked list."""

    def __init__(self):
        self.length = 0
        # TODO: Initialize head and tail

    def enqueue(self, item: T) -> None:
        """Add item to the back of the queue."""
        pass

    def deque(self) -> Optional[T]:
        """Remove and return item from the front of the queue."""
        pass

    def peek(self) -> Optional[T]:
        """Return item from the front without removing it."""
        pass
