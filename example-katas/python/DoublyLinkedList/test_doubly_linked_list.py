"""Tests for DoublyLinkedList implementation."""

import sys
import os

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from dsa_base import test_list
from doubly_linked_list import DoublyLinkedList


def test_doubly_linked_list():
    """Test DoublyLinkedList with standard list tests."""
    list_impl = DoublyLinkedList()
    test_list(list_impl)
    print("All DoublyLinkedList tests passed!")


if __name__ == "__main__":
    test_doubly_linked_list()
