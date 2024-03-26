
from data_structures.tree import TreeNode


class Solution(object):
    def isValidBST(self, root):
        """
        :type root: TreeNode
        :rtype: bool
        """

        if self.dfs(root):
            return True
        else:
            return False

    def dfs(self, root):
        # return [min_val, max_val] when valid BST
        # otherwise return None
        if not root:
            return None

        min_val = root.val
        max_val = root.val

        if not root.left and not root.right:
            # leaf node
            return [min_val, max_val]

        left = self.dfs(root.left)
        right = self.dfs(root.right)

        if root.left:
            if not left:
                return None
            elif left[1] >= root.val:
                return None
            else:
                min_val = left[0]

        if root.right:
            if not right:
                return None
            elif right[0] <= root.val:
                return None
            else:
                max_val = right[1]
        return min_val, max_val


if __name__ == "__main__":

    tree1 = TreeNode.from_array([2, 1, 3])

    assert Solution().isValidBST(tree1)

    tree2 = TreeNode.from_array([5, 1, 4, None, None, 3, 6])

    assert Solution().isValidBST(tree2) is False
