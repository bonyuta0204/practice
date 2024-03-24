
from data_structures.tree import TreeNode


class Solution(object):
    def maxDepth(self, root):
        """
        :type root: TreeNode
        :rtype: int
        """
        # use dfs for max_node

        if root is None:
            return 0
        else:
            return max(self.maxDepth(root.left), self.maxDepth(root.right)) + 1


if __name__ == "__main__":

    tree = TreeNode.from_array([3, 9, 20, None, None, 15, 7])

    assert Solution().maxDepth(tree) == 3
