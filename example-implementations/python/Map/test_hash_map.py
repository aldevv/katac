"""Tests for HashMap implementation."""

from hash_map import HashMap


def test_hash_map():
    """Test HashMap with various operations."""
    m = HashMap(capacity=8)

    m.set("foo", 55)
    assert len(m) == 1, f"Expected: 1, got: {len(m)}"

    m.set("fool", 75)
    assert len(m) == 2, f"Expected: 2, got: {len(m)}"

    m.set("foolish", 105)
    assert len(m) == 3, f"Expected: 3, got: {len(m)}"

    m.set("bar", 69)
    assert len(m) == 4, f"Expected: 4, got: {len(m)}"

    result = m.get("bar")
    assert result == 69, f"Expected: 69, got: {result}"

    result = m.get("blaz")
    assert result is None, f"Expected: None, got: {result}"

    m.delete("barblarbr")
    assert len(m) == 4, f"Expected: 4, got: {len(m)}"

    result = m.delete("meh")
    assert result is None, f"Expected: None, got: {result}"

    m.set("meh", 420)
    assert len(m) == 5, f"Expected: 5, got: {len(m)}"

    result = m.get("meh")
    assert result == 420, f"Expected: 420, got: {result}"

    m.delete("bar")
    assert len(m) == 4, f"Expected: 4, got: {len(m)}"

    result = m.get("bar")
    assert result is None, f"Expected: None, got: {result}"

    m.set("heh", 255)
    assert len(m) == 5, f"Expected: 5, got: {len(m)}"

    m.set("doggo", 1020)
    assert len(m) == 6, f"Expected: 6, got: {len(m)}"

    m.set("monst", 1020)
    assert len(m) == 7, f"Expected: 7, got: {len(m)}"

    m.set("oothe", 1)
    assert len(m) == 8, f"Expected: 8, got: {len(m)}"

    m.set("other", 128)
    assert len(m) == 9, f"Expected: 9, got: {len(m)}"

    m.set("some", 514)
    assert len(m) == 10, f"Expected: 10, got: {len(m)}"

    m.set("same", 12)
    assert len(m) == 11, f"Expected: 11, got: {len(m)}"

    print("All HashMap tests passed!")


if __name__ == "__main__":
    test_hash_map()
