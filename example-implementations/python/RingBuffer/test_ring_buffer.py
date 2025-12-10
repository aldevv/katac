"""Tests for RingBuffer implementation."""

from ring_buffer import RingBuffer


def test_ring_buffer():
    """Test RingBuffer with various operations."""
    rb = RingBuffer(capacity=3)

    assert rb.is_empty(), "RingBuffer should be empty initially"
    assert not rb.is_full(), "RingBuffer should not be full initially"

    rb.push(1)
    rb.push(2)
    rb.push(3)

    assert rb.is_full(), "RingBuffer should be full after 3 pushes"
    assert len(rb) == 3, f"Length should be 3, got {len(rb)}"

    val = rb.get(0)
    assert val == 1, f"Get(0) should return 1, got {val}"

    val = rb.get(2)
    assert val == 3, f"Get(2) should return 3, got {val}"

    rb.push(4)
    assert rb.is_full(), "RingBuffer should still be full"
    assert len(rb) == 3, f"Length should still be 3, got {len(rb)}"

    val = rb.get(0)
    assert val == 2, f"Get(0) should now return 2 (oldest was overwritten), got {val}"

    val = rb.pop()
    assert val == 2, f"Pop should return 2, got {val}"
    assert len(rb) == 2, f"Length should be 2 after pop, got {len(rb)}"

    val = rb.pop()
    assert val == 3, f"Pop should return 3, got {val}"

    val = rb.pop()
    assert val == 4, f"Pop should return 4, got {val}"

    assert rb.is_empty(), "RingBuffer should be empty after all pops"

    val = rb.pop()
    assert val is None, f"Pop on empty buffer should return None, got {val}"

    print("All RingBuffer tests passed!")


if __name__ == "__main__":
    test_ring_buffer()
