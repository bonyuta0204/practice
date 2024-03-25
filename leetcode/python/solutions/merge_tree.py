
from collections import deque
from data_structures.tree import TreeNode


class Solution(object):
    def mergeTrees(self, root1, root2):
        """
        :type root1: TreeNode
        :type root2: TreeNode
        :rtype: TreeNode
        """
        return self.dfs(root1, root2)

    def dfs(self, root1, root2):

        if root1 is None and root2 is None:
            return None

        if root1 is None:
            return root2
        if root2 is None:
            return root1
        else:
            sum_root = TreeNode(root1.val + root2.val)
            sum_root.left = self.dfs(root1.left,  root2.left)
            sum_root.right = self.dfs(root1.right, root2.right)
            return sum_root


if __name__ == "__main__":

    tree1 = TreeNode.from_array([1, 3, 2, 5])
    tree2 = TreeNode.from_array([3, 4, 5, 5, 4, None, 7])

    Solution().mergeTrees(tree1, tree2).print_tree()
