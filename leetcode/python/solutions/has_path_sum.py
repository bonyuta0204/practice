from data_structures.tree import TreeNode


class Solution(object):
    def hasPathSum(self, root, targetSum):
        """
        :type root: TreeNode
        :type targetSum: int
        :rtype: bool
        """

        if root is None:
            return False
        if root.left is None and root.right is None:
            return targetSum == root.val
        else:
            return self.hasPathSum(root.left, targetSum - root.val) or self.hasPathSum(root.right, targetSum - root.val)


if __name__ == "__main__":
    tree = TreeNode.from_array(
        [5, 4, 8, 11, None, 13, 4, 7, 2, None, None, None, 1])

    assert Solution().hasPathSum(tree, 22) == True
