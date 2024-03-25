from data_structures.tree import TreeNode
from collections import deque


class Solution(object):
    def zigzagLevelOrder(self, root):
        """
        :type root: TreeNode
        :rtype: List[List[int]]
        """

        if not root:
            return []

        self.current_level = 0
        self.current_stack = []
        self.next_stack = []

        self.next_stack.append(root)

        result = []

        while self.next_stack:
            self.current_stack = self.next_stack
            self.next_stack = []
            if len(result) < self.current_level + 1:
                result.append([])

            while self.current_stack:
                node = self.current_stack.pop()
                result[self.current_level].append(node.val)

                if self.current_level % 2 == 0:
                    if node.left:
                        self.next_stack.append(node.left)
                    if node.right:
                        self.next_stack.append(node.right)
                else:
                    if node.right:
                        self.next_stack.append(node.right)
                    if node.left:
                        self.next_stack.append(node.left)
            self.current_level += 1

        return result


if __name__ == "__main__":
    tree = TreeNode.from_array(
        [3, 9, 20, None, None, 15, 7]
    )

    print(Solution().zigzagLevelOrder(tree))
