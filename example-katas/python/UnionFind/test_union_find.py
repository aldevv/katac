"""Tests for UnionFind implementation."""

from union_find import UnionFind


def test_union_find():
    """Test UnionFind with various operations."""
    uf = UnionFind(10)

    assert uf.count() == 10, f"Initial count should be 10, got {uf.count()}"

    assert not uf.connected(0, 1), "0 and 1 should not be connected initially"

    uf.union(0, 1)
    assert uf.connected(0, 1), "0 and 1 should be connected after union"
    assert uf.count() == 9, f"Count should be 9 after one union, got {uf.count()}"

    uf.union(1, 2)
    assert uf.connected(0, 2), "0 and 2 should be connected"
    assert uf.connected(1, 2), "1 and 2 should be connected"
    assert uf.count() == 8, f"Count should be 8 after two unions, got {uf.count()}"

    uf.union(3, 4)
    assert uf.connected(3, 4), "3 and 4 should be connected"
    assert not uf.connected(0, 3), "0 and 3 should not be connected"
    assert uf.count() == 7, f"Count should be 7 after three unions, got {uf.count()}"

    uf.union(0, 4)
    assert uf.connected(0, 4), "0 and 4 should be connected"
    assert uf.connected(1, 3), "1 and 3 should be connected through the union"
    assert uf.connected(2, 4), "2 and 4 should be connected through the union"
    assert uf.count() == 6, f"Count should be 6 after merging two components, got {uf.count()}"

    root0 = uf.find(0)
    root1 = uf.find(1)
    root2 = uf.find(2)
    root3 = uf.find(3)
    root4 = uf.find(4)

    assert root0 == root1, "0 and 1 should have the same root"
    assert root0 == root2, "0 and 2 should have the same root"
    assert root0 == root3, "0 and 3 should have the same root"
    assert root0 == root4, "0 and 4 should have the same root"

    assert not uf.connected(5, 6), "5 and 6 should not be connected"
    assert uf.count() == 6, f"Count should still be 6, got {uf.count()}"

    print("All UnionFind tests passed!")


if __name__ == "__main__":
    test_union_find()
