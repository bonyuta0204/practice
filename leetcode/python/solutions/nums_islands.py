class Solution(object):
    def numIslands(self, grid):
        """
        :type grid: List[List[str]]
        :rtype: int
        """

        self.height = len(grid)
        self.width = len(grid[0])
        self.points = set([(y, x) for y in range(self.height)
                          for x in range(self.width)])

        count = 0

        while len(self.points):

            point = self.points.pop()

            if grid[point[0]][point[1]] == "0":
                continue
            else:

                count += 1
                to_visit = set([point])

                while len(to_visit):
                    visiting = to_visit.pop()

                    if grid[visiting[0]][visiting[1]] == "1":
                        for adjacent in self.adjacent(visiting):
                            if adjacent in self.points:
                                self.points.remove(adjacent)
                                to_visit.add(adjacent)
        return count

    def adjacent(self, point):
        y = point[0]
        x = point[1]

        points = []

        if x - 1 >= 0:
            points.append((y, x - 1))
        if x + 1 < self.width:
            points.append((y, x + 1))
        if y - 1 >= 0:
            points.append((y - 1, x))
        if y + 1 < self.height:
            points.append((y + 1, x))

        return points


if __name__ == "__main__":
    assert Solution().numIslands([
        ["1", "1", "1", "1", "0"],
        ["1", "1", "0", "1", "0"],
        ["1", "1", "0", "0", "0"],
        ["0", "0", "0", "0", "0"]
    ]) == 1
