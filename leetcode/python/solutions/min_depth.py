
from data_structures.tree import TreeNode


class Solution(object):
    def minDepth(self, root):
        """
        :type root: TreeNode
        :rtype: int
        """
        # use dfs for max_node

        if root is None:
            return 0
        else:
            left_length = self.minDepth(root.left)
            right_length = self.minDepth(root.right)

            if left_length > 0 and right_length > 0:
                return min(left_length, right_length) + 1
            elif left_length == 0 or right_length == 0:
                return max(left_length, right_length) + 1
            else:
                return 1



if __name__ == "__main__":

    tree = TreeNode.from_array([3, 9, 20, None, None, 15, 7])

    assert Solution().minDepth(tree) == 2
