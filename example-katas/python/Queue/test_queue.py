"""Tests for Queue implementation."""

from queue import Queue


def test_queue():
    """Test Queue with standard queue operations."""
    q = Queue()

    q.enqueue(5)
    q.enqueue(7)
    q.enqueue(9)

    val = q.deque()
    assert val == 5, f"Value should be 5, but is {val}"
    assert q.length == 2, f"Length should be 2, but is {q.length}"

    q.enqueue(11)

    val = q.deque()
    assert val == 7, f"Value should be 7, but is {val}"

    val = q.deque()
    assert val == 9, f"Value should be 9, but is {val}"

    val = q.peek()
    assert val == 11, f"Value should be 11, but is {val}"

    val = q.deque()
    assert val == 11, f"Value should be 11, but is {val}"

    val = q.deque()
    assert val is None, f"Value should be None, but is {val}"
    assert q.length == 0, f"Length should be 0, but is {q.length}"

    q.enqueue(69)

    val = q.peek()
    assert val == 69, f"Value should be 69, but is {val}"
    assert q.length == 1, f"Length should be 1, but is {q.length}"

    print("All Queue tests passed!")


if __name__ == "__main__":
    test_queue()
