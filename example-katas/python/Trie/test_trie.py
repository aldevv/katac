"""Tests for Trie implementation."""

from trie import Trie


def test_trie():
    """Test Trie with various operations."""
    t = Trie()

    t.insert("foo")
    t.insert("fool")
    t.insert("foolish")
    t.insert("bar")

    result = t.find("fo")
    expected = ["foo", "fool", "foolish"]
    result.sort()
    expected.sort()
    assert result == expected, f"Expected: {expected}, got: {result}"

    t.delete("fool")
    result = t.find("fo")
    expected = ["foo", "foolish"]
    result.sort()
    expected.sort()
    assert result == expected, f"Expected: {expected}, got: {result}"

    print("All Trie tests passed!")


if __name__ == "__main__":
    test_trie()
