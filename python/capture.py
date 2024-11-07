
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


def main():
    print(max_captures(5, (2, 2), [], 3))  # 0
    print(max_captures(5, (2, 2), [(3, 3)], 2))  # 1
    print(max_captures(5, (2, 2), [(3, 3), (1, 1), (4, 4)], 3))  # 3


if __name__ == "__main__":
    main()
