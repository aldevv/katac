"""Tests for LRU implementation."""

from lru import LRU


def test_lru():
    """Test LRU cache with various operations."""
    cache = LRU(capacity=3)

    result = cache.get("foo")
    assert result is None, f"Expected: None, got: {result}"

    cache.update("foo", 69)
    result = cache.get("foo")
    assert result == 69, f"Expected: 69, got: {result}"

    cache.update("bar", 420)
    result = cache.get("bar")
    assert result == 420, f"Expected: 420, got: {result}"

    cache.update("baz", 1337)
    result = cache.get("baz")
    assert result == 1337, f"Expected: 1337, got: {result}"

    cache.update("ball", 69420)
    result = cache.get("ball")
    assert result == 69420, f"Expected: 69420, got: {result}"

    # "foo" should be evicted
    result = cache.get("foo")
    assert result is None, f"Expected: None, got: {result}"

    result = cache.get("bar")
    assert result == 420, f"Expected: 420, got: {result}"

    cache.update("foo", 69)
    result = cache.get("bar")
    assert result == 420, f"Expected: 420, got: {result}"

    result = cache.get("foo")
    assert result == 69, f"Expected: 69, got: {result}"

    # "baz" should be evicted
    result = cache.get("baz")
    assert result is None, f"Expected: None, got: {result}"

    print("All LRU tests passed!")


if __name__ == "__main__":
    test_lru()
