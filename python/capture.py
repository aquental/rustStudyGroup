import pytest
1


def max_captures(N, knight_pos, opponents, K):
    """
    Calculates the maximum number of opponent pieces a knight can capture in K moves.

    Args:
        N: Size of the chessboard.
        knight_pos: Initial position of the knight (tuple).
        opponents: List of opponent positions (tuples).
        K: Maximum number of moves.

    Returns:
        Maximum number of captures.
    """

    def is_valid(pos):
        x, y = pos
        return 0 <= x < N and 0 <= y < N

    def get_neighbors(pos):
        x, y = pos
        return [(x + 2, y + 1), (x + 2, y - 1), (x - 2, y + 1), (x - 2, y - 1),
                (x + 1, y + 2), (x + 1, y - 2), (x - 1, y + 2), (x - 1, y - 2)]

    queue = [(knight_pos, 0)]  # (position, captures)
    visited = set()
    max_captures = 0

    while queue and K > 0:
        new_queue = []
        for pos, captures in queue:
            if pos in visited:
                continue
            visited.add(pos)
            max_captures = max(max_captures, captures + opponents.count(pos))
            for neighbor in get_neighbors(pos):
                if is_valid(neighbor):
                    new_queue.append(
                        (neighbor, captures + opponents.count(neighbor)))
        queue = new_queue
        K -= 1

    return max_captures


def test_empty_board():
    assert max_captures(5, (2, 2), [], 3) == 0


def test_single_opponent():
    assert max_captures(5, (2, 2), [(3, 3)], 2) == 1


def test_multiple_opponents():
    assert max_captures(5, (2, 2), [(3, 3), (1, 1), (4, 4)], 3) == 3


def test_no_reachable_opponents():
    assert max_captures(5, (2, 2), [(0, 0), (4, 4)], 1) == 0


def test_knight_starts_on_opponent():
    assert max_captures(5, (2, 2), [(2, 2)], 1) == 1


def test_zero_moves():
    assert max_captures(5, (2, 2), [(3, 3)], 0) == 0


def test_invalid_knight_position():
    with pytest.raises(ValueError):
        max_captures(5, (6, 6), [(3, 3)], 2)


def test_invalid_opponent_positions():
    with pytest.raises(ValueError):
        max_captures(5, (2, 2), [(6, 6)], 2)


def test_large_board_and_many_moves():
    assert max_captures(100, (50, 50), [(49, 49), (51, 51)], 10) == 2


def main():
    test_empty_board()
    test_single_opponent()
    test_multiple_opponents()
    test_no_reachable_opponents()
    test_knight_starts_on_opponent()
    test_zero_moves()
    test_invalid_knight_position()
    test_invalid_opponent_positions()
    test_large_board_and_many_moves()


if __name__ == "__main__":
    main()
