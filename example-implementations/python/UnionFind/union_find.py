"""
Union-Find (Disjoint Set Union) data structure.
Efficiently tracks connected components with path compression and union by rank.
"""

from typing import List


class UnionFind:
    """Union-Find data structure for tracking disjoint sets."""

    def __init__(self, size: int):
        self._count = size
        self._parent: List[int] = list(range(size))
        self._rank: List[int] = [0] * size

    def count(self) -> int:
        """Return the number of disjoint sets."""
        return self._count

    def find(self, p: int) -> int:
        """Find the root of element p with path compression."""
        if self._parent[p] != p:
            self._parent[p] = self.find(self._parent[p])
        return self._parent[p]

    def union(self, p: int, q: int) -> None:
        """Unite the sets containing p and q using union by rank."""
        root_p = self.find(p)
        root_q = self.find(q)

        if root_p == root_q:
            return

        # Union by rank: attach smaller tree to larger tree
        if self._rank[root_p] < self._rank[root_q]:
            self._parent[root_p] = root_q
        elif self._rank[root_p] > self._rank[root_q]:
            self._parent[root_q] = root_p
        else:
            self._parent[root_q] = root_p
            self._rank[root_p] += 1

        self._count -= 1

    def connected(self, p: int, q: int) -> bool:
        """Check if p and q are in the same set."""
        return self.find(p) == self.find(q)
