"""Tests for ArrayList implementation."""

import sys
import os

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from dsa_base import test_list
from array_list import ArrayList


def test_array_list():
    """Test ArrayList with standard list tests."""
    list_impl = ArrayList(capacity=3)
    test_list(list_impl)
    print("All ArrayList tests passed!")


if __name__ == "__main__":
    test_array_list()
