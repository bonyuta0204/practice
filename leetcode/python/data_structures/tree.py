from typing import List
from collections import deque


class TreeNode(object):
    def __init__(self, val=0, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    @classmethod
    def from_array(cls, array: List[int]):
        """
        Build tree node from array expression.
        The array is a level-order traversal of the tree.
        """

        if not array:
            return None

        root = TreeNode(array[0])
        queue = deque([root])
        i = 1

        while i < len(array):
            node = queue.popleft()

            if array[i] is not None:
                node.left = TreeNode(array[i])
                queue.append(node.left)
            i += 1

            if i < len(array) and array[i] is not None:
                node.right = TreeNode(array[i])
                queue.append(node.right)
            i += 1

        return root

    def print_tree(self):
        """
        Print the tree in a visually organized way.
        """
        levels = self._level_order_traversal()
        for level in levels:
            print(" ".join(level))

    def _level_order_traversal(self):
        """
        Helper method to generate level order traversal as a list of lists.
        This will be used to print the tree.
        """
        if not self:
            return []

        result, current = [], deque([self])

        while current:
            level_size = len(current)
            current_level = []

            for _ in range(level_size):
                node = current.popleft()
                if node:
                    current_level.append(str(node.val))
                    current.append(node.left)
                    current.append(node.right)
                else:
                    current_level.append(" ")

            if any(node != " " for node in current_level):
                result.append(current_level)

        # Adjust spacing for visual alignment
        tree_height = len(result)
        for i in range(tree_height):
            spaces = " " * (2 ** (tree_height - i - 1) - 1)
            result[i] = spaces.join(result[i])

        return result


if __name__ == "__main__":
    root = TreeNode.from_array([3, 9, 20, None, None, 15, 7])
    root.print_tree()
