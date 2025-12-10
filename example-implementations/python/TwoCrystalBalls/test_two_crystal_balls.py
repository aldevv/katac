"""Tests for TwoCrystalBalls implementation."""

import random
from two_crystal_balls import two_crystal_balls


def test_two_crystal_balls():
    """Test TwoCrystalBalls with random data."""
    random.seed()

    idx = random.randint(0, 10000)

    data = [False] * 10000
    for i in range(idx, len(data)):
        data[i] = True

    result = two_crystal_balls(data)
    assert result == idx, f"Expected: {idx}, got: {result}"

    data = [False] * 821
    result = two_crystal_balls(data)
    assert result == -1, f"Expected: -1, got: {result}"

    print("All TwoCrystalBalls tests passed!")


if __name__ == "__main__":
    test_two_crystal_balls()
