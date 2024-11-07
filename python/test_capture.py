import pytest
import capture


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
