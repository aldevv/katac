"""
Two Crystal Balls problem.

Given two crystal balls that will break if dropped from high enough distance,
determine the exact spot in which it will break in the most optimized way.

The input is a list of booleans representing floors. False means the ball won't break,
True means it will break. Find the first index where the ball breaks.

Returns the index of the first True value, or -1 if no True value exists.

The optimal approach uses square root jumps to minimize the worst case.
"""

import math
from typing import List


def two_crystal_balls(breaks: List[bool]) -> int:
    """
    Find the first breaking point using two crystal balls.

    Uses sqrt(n) jump intervals to optimize for worst case of O(sqrt(n)).

    Args:
        breaks: List of booleans where True indicates the ball breaks

    Returns:
        Index of first True value, or -1 if no True value exists
    """
    jump_amount = int(math.floor(math.sqrt(len(breaks))))

    # Jump forward by sqrt(n) until we find a break
    i = jump_amount
    while i < len(breaks):
        if breaks[i]:
            break
        i += jump_amount

    # Go back one jump and linearly search
    i -= jump_amount

    # Linear search within the jump interval
    for j in range(jump_amount):
        if i >= len(breaks):
            break
        if breaks[i]:
            return i
        i += 1

    return -1
