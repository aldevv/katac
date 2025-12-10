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
        self._head: Optional[Node[T]] = None
        self._tail: Optional[Node[T]] = None

    def enqueue(self, item: T) -> None:
        """Add item to the back of the queue."""
        node = Node(value=item)

        if self._tail is None:
            self._head = node
            self._tail = node
        else:
            self._tail.next = node
            self._tail = node

        self.length += 1

    def deque(self) -> Optional[T]:
        """Remove and return item from the front of the queue."""
        if self._head is None:
            return None

        val = self._head.value
        self._head = self._head.next

        if self._head is None:
            self._tail = None

        self.length -= 1
        return val

    def peek(self) -> Optional[T]:
        """Return item from the front without removing it."""
        if self._head is None:
            return None
        return self._head.value
