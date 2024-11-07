def min_cost_to_reach_end(grid, amount):
    """
    Finds the minimum cost to reach the bottom-right corner of a grid.
    Start at 0,0 and move to the right or down.

    Args:
        grid: A 2D list of integers representing costs at each position.
        amount: The maximum amount that can be spent.

    Returns:
        The minimum cost to reach the end, or -1 if it's not possible.
    """

    # Create a 2D DP array to store the minimum cost to reach each cell.
    n, m = len(grid), len(grid[0])
    # Initialize the starting cell's cost to the cost at that position.
    dp = [[float('inf')] * m for _ in range(n)]

    # Base case: Starting position
    dp[0][0] = grid[0][0]

    # Iterate over each cell in the grid, excluding the starting cell.
    # Calculate the minimum cost to reach the current cell from adjacent cells.
    # Update the current cell's cost if it's within the amount limit.

    for i in range(n):
        for j in range(m):
            # Skip the starting position
            if i == 0 and j == 0:
                continue

            # Calculate the minimum cost from adjacent cells
            min_cost = min(
                dp[i - 1][j - 1] if i > 0 and j > 0 else float('inf'),
                dp[i - 1][j] if i > 0 else float('inf'),
                dp[i][j - 1] if j > 0 else float('inf')
            )

            # Update the current cell's cost if it's within the amount limit
            dp[i][j] = min(dp[i][j], min_cost + grid[i][j])
    # If the final cell's cost is within the amount limit, return it.
    # Otherwise, return -1 indicating it's not possible to reach the end.
    return dp[-1][-1] if dp[-1][-1] <= amount else -1


def find_min_element(arr):
    min_val = float('inf')
    for num in arr:
        min_val = min(min_val, num)
    return min_val


def main():
    grid = [[1, 1], [1, 1]]
    amount = 1
    min_cost = min_cost_to_reach_end(grid, amount)
    print(min_cost)


if __name__ == "__main__":
    main()
