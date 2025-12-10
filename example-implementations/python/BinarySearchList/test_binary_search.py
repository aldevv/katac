"""Tests for BinarySearch implementation."""

from binary_search import binary_search


def test_binary_search():
    """Test BinarySearch with various test cases."""
    foo = [1, 3, 4, 69, 71, 81, 90, 99, 420, 1337, 69420]

    result = binary_search(foo, 69)
    assert result is True, f"Expected: True, got: {result}"

    result = binary_search(foo, 1336)
    assert result is False, f"Expected: False, got: {result}"

    result = binary_search(foo, 69420)
    assert result is True, f"Expected: True, got: {result}"

    result = binary_search(foo, 69421)
    assert result is False, f"Expected: False, got: {result}"

    result = binary_search(foo, 1)
    assert result is True, f"Expected: True, got: {result}"

    result = binary_search(foo, 0)
    assert result is False, f"Expected: False, got: {result}"

    print("All BinarySearch tests passed!")


if __name__ == "__main__":
    test_binary_search()
