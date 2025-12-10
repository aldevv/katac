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
        # TODO: Initialize head and tail

    def __len__(self) -> int:
        return self._length

    def prepend(self, item: T) -> None:
        """Add item to the beginning of the list."""
        pass

    def insert_at(self, item: T, idx: int) -> None:
        """Insert item at the given index."""
        pass

    def append(self, item: T) -> None:
        """Add item to the end of the list."""
        pass

    def remove(self, item: T) -> Optional[T]:
        """Remove first occurrence of item. Returns the item if found, None otherwise."""
        pass

    def get(self, idx: int) -> Optional[T]:
        """Get item at index. Returns None if index is out of bounds."""
        pass

    def remove_at(self, idx: int) -> Optional[T]:
        """Remove item at index. Returns the item if found, None otherwise."""
        pass
