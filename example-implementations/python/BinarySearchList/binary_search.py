"""
Binary Search implementation.
Search for an element in a sorted list in O(log n) time.
"""

from typing import List


def binary_search(haystack: List[int], needle: int) -> bool:
    """
    Search for needle in sorted haystack using binary search.

    Args:
        haystack: Sorted list of integers
        needle: Value to search for

    Returns:
        True if needle is found, False otherwise
    """
    lo = 0
    hi = len(haystack)

    while lo < hi:
        mid = lo + (hi - lo) // 2
        val = haystack[mid]

        if val == needle:
            return True
        elif val < needle:
            lo = mid + 1
        else:
            hi = mid

    return False
