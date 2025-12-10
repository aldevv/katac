"""
DoublyLinkedList implementation.
Each node has references to both previous and next nodes.
"""

from typing import TypeVar, Optional, Generic
from dataclasses import dataclass
import sys
import os

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from dsa_base import List

T = TypeVar('T')


@dataclass
class Node(Generic[T]):
    """Node in the doubly-linked list."""
    value: T
    prev: Optional['Node[T]'] = None
    next: Optional['Node[T]'] = None


class DoublyLinkedList(List[T], Generic[T]):
    """Doubly-linked list implementation."""

    def __init__(self):
        self._length = 0
        self._head: Optional[Node[T]] = None
        self._tail: Optional[Node[T]] = None

    def __len__(self) -> int:
        return self._length

    def prepend(self, item: T) -> None:
        """Add item to the beginning of the list."""
        node = Node(value=item)

        if self._head is None:
            self._head = node
            self._tail = node
        else:
            node.next = self._head
            self._head.prev = node
            self._head = node

        self._length += 1

    def insert_at(self, item: T, idx: int) -> None:
        """Insert item at the given index."""
        if idx < 0 or idx > self._length:
            return

        if idx == 0:
            self.prepend(item)
            return

        if idx == self._length:
            self.append(item)
            return

        curr = self._head
        for _ in range(idx):
            curr = curr.next

        node = Node(value=item)
        node.next = curr
        node.prev = curr.prev
        curr.prev.next = node
        curr.prev = node
        self._length += 1

    def append(self, item: T) -> None:
        """Add item to the end of the list."""
        node = Node(value=item)

        if self._tail is None:
            self._head = node
            self._tail = node
        else:
            node.prev = self._tail
            self._tail.next = node
            self._tail = node

        self._length += 1

    def remove(self, item: T) -> Optional[T]:
        """Remove first occurrence of item. Returns the item if found, None otherwise."""
        curr = self._head

        while curr is not None:
            if curr.value == item:
                if curr.prev is not None:
                    curr.prev.next = curr.next
                else:
                    self._head = curr.next

                if curr.next is not None:
                    curr.next.prev = curr.prev
                else:
                    self._tail = curr.prev

                self._length -= 1
                return curr.value

            curr = curr.next

        return None

    def get(self, idx: int) -> Optional[T]:
        """Get item at index. Returns None if index is out of bounds."""
        if idx < 0 or idx >= self._length:
            return None

        curr = self._head
        for _ in range(idx):
            curr = curr.next

        return curr.value

    def remove_at(self, idx: int) -> Optional[T]:
        """Remove item at index. Returns the item if found, None otherwise."""
        if idx < 0 or idx >= self._length:
            return None

        curr = self._head
        for _ in range(idx):
            curr = curr.next

        if curr.prev is not None:
            curr.prev.next = curr.next
        else:
            self._head = curr.next

        if curr.next is not None:
            curr.next.prev = curr.prev
        else:
            self._tail = curr.prev

        self._length -= 1
        return curr.value
