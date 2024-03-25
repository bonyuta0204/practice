from collections import deque


class Solution(object):
    def maze_solver(self, maze, start, target):
        """
        Solves the maze by finding a path from start to target using BFS.

        :param list[list[int]] maze: A 2D list representing the maze, where 0 is an open path, and 1 is a wall.
        :param list[int] start: A list of two integers representing the starting coordinates (row, column) in the maze.
        :param list[int] target: A list of two integers representing the target coordinates (row, column) in the maze.
        :return: Returns True if there is a path from start to target, otherwise False.
        :rtype: bool
        """

        if not len(maze):
            return False

        to_search = deque([tuple(start)])
        self.height = len(maze)
        self.width = len(maze[0])
        self.visited = set()

        while len(to_search):
            point = to_search.popleft()
            self.visited.add(point)

            if point[0] == target[0] and point[1] == target[1]:
                return True

            if maze[point[0]][point[1]] == 1:
                continue
            elif maze[point[0]][point[1]] == 0:
                adjacents = self.adjacent((point[0], point[1]))

                for a in adjacents:
                    if maze[a[0]][a[1]] == 0:
                        to_search.append(tuple(a))
        return False

    def adjacent(self, point):
        y = point[0]
        x = point[1]

        adjacents = []

        if y - 1 >= 0:
            adjacents.append((y-1, x))
        if y + 1 < self.height:
            adjacents.append((y+1, x))
        if x - 1 >= 0:
            adjacents.append((y, x - 1))
        if x + 1 < self.width:
            adjacents.append((y, x + 1))

        return list(filter(lambda x: x not in self.visited, adjacents))


if __name__ == "__main__":
    # Test Case 1
    maze1 = [
        [0, 1, 0, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 0, 1, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 1, 1, 0]
    ]
    assert Solution().maze_solver(
        maze1, [0, 0], [4, 4]) == True, "Test case 1 failed"
