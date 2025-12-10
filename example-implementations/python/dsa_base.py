"""
Data structures and algorithms testing utilities for Python katas.
This file contains shared test helpers and data structures.
"""

from abc import ABC, abstractmethod
from typing import TypeVar, Generic, Optional, Tuple
from dataclasses import dataclass


T = TypeVar('T')


# List interface
class List(ABC, Generic[T]):
    """Abstract base class for list implementations."""

    @abstractmethod
    def __len__(self) -> int:
        """Return the length of the list."""
        pass

    @abstractmethod
    def prepend(self, item: T) -> None:
        """Add item to the beginning of the list."""
        pass

    @abstractmethod
    def insert_at(self, item: T, idx: int) -> None:
        """Insert item at the given index."""
        pass

    @abstractmethod
    def append(self, item: T) -> None:
        """Add item to the end of the list."""
        pass

    @abstractmethod
    def remove(self, item: T) -> Optional[T]:
        """Remove first occurrence of item. Returns the item if found, None otherwise."""
        pass

    @abstractmethod
    def get(self, idx: int) -> Optional[T]:
        """Get item at index. Returns None if index is out of bounds."""
        pass

    @abstractmethod
    def remove_at(self, idx: int) -> Optional[T]:
        """Remove item at index. Returns the item if found, None otherwise."""
        pass


def test_list(list_impl: List[int]) -> None:
    """
    Standard test suite for List implementations.
    Tests append, prepend, insert, remove, get, and remove_at operations.
    """
    # Append
    list_impl.append(5)
    list_impl.append(7)
    list_impl.append(9)

    val = list_impl.get(2)
    assert val == 9, f"Value should be 9, but is {val}"

    val = list_impl.remove_at(1)
    assert val == 7, f"Value should be 7, but is {val}"
    assert len(list_impl) == 2, f"Length should be 2, but is {len(list_impl)}"

    # Remove
    list_impl.append(11)
    val = list_impl.remove_at(1)
    assert val == 9, f"Value should be 9, but is {val}"

    val = list_impl.remove(9)
    assert val is None, f"Value should be None, but is {val}"

    val = list_impl.remove_at(0)
    assert val == 5, f"Value should be 5, but is {val}"

    val = list_impl.remove_at(0)
    assert val == 11, f"Value should be 11, but is {val}"
    assert len(list_impl) == 0, f"Length should be 0, but is {len(list_impl)}"

    # Prepend
    list_impl.prepend(5)
    list_impl.prepend(7)
    list_impl.prepend(9)

    val = list_impl.get(2)
    assert val == 5, f"Value should be 5, but is {val}"

    val = list_impl.get(0)
    assert val == 9, f"Value should be 9, but is {val}"

    val = list_impl.remove(9)
    assert val == 9, f"Value should be 9, but is {val}"
    assert len(list_impl) == 2, f"Length should be 2, but is {len(list_impl)}"

    val = list_impl.get(0)
    assert val == 7, f"Value should be 7, but is {val}"

    # Insert
    list_impl.insert_at(10, 1)

    val = list_impl.get(1)
    assert val == 10, f"Value should be 10, but is {val}"

    val = list_impl.get(2)
    assert val == 5, f"Value should be 5, but is {val}"

    list_impl.insert_at(20, 2)
    val = list_impl.get(2)
    assert val == 20, f"Value should be 20, but is {val}"
    val = list_impl.get(3)
    assert val == 5, f"Value should be 5, but is {val}"

    list_impl.insert_at(30, 4)
    val = list_impl.get(4)
    assert val == 30, f"Value should be 30, but is {val}"
    val = list_impl.get(3)
    assert val == 5, f"Value should be 5, but is {val}"


# Graph structures
@dataclass
class GraphEdge:
    """Represents a weighted edge in a graph."""
    to: int
    weight: int


WeightedAdjacencyList = list[list[GraphEdge]]
WeightedAdjacencyMatrix = list[list[int]]


# Tree structures
@dataclass
class BinaryNode(Generic[T]):
    """Binary tree node."""
    value: T
    left: Optional['BinaryNode[T]'] = None
    right: Optional['BinaryNode[T]'] = None


# Maze structures
@dataclass
class Point:
    """2D point."""
    x: int
    y: int
