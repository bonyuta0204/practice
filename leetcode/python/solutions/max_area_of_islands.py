class Solution(object):
    def maxAreaOfIsland(self, grid):
        """
        :type grid: List[List[str]]
        :rtype: int
        """

        if not grid:
            return 0

        self.grid = grid
        self.height = len(grid)
        self.width = len(grid[0])

        max = 0

        for y in range(self.height):
            for x in range(self.width):
                result = self.dfs(y, x, 0)
                if max < result:
                    max = result
        return max

    def dfs(self, y, x, count):
        if x < 0 or x >= self.width or y < 0 or y >= self.height or self.grid[y][x] == 0:
            return count
        elif self.grid[y][x] == 1:
            self.grid[y][x] = 0
            count += 1

            count = self.dfs(y-1, x, count)
            count = self.dfs(y+1, x, count)
            count = self.dfs(y, x-1, count)
            count = self.dfs(y, x+1, count)
            return count


if __name__ == "__main__":
    assert Solution().maxAreaOfIsland(
        [
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0],
            [0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0]
        ]
    ) == 6
