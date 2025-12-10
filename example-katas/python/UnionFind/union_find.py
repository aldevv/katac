"""
Union-Find (Disjoint Set Union) data structure.
Efficiently tracks connected components with path compression and union by rank.
"""

from typing import List


class UnionFind:
    """Union-Find data structure for tracking disjoint sets."""

    def __init__(self, size: int):
        self._count = size
        # TODO: Initialize parent and rank arrays

    def count(self) -> int:
        """Return the number of disjoint sets."""
        return self._count

    def find(self, p: int) -> int:
        """Find the root of element p with path compression."""
        pass

    def union(self, p: int, q: int) -> None:
        """Unite the sets containing p and q."""
        pass

    def connected(self, p: int, q: int) -> bool:
        """Check if p and q are in the same set."""
        pass
