from typing import List
from collections import deque
from data_structures.tree import TreeNode


class Solution(object):
    def buildTree(self, preorder, inorder):
        """
        :type preorder: List[int]
        :type inorder: List[int]
        :rtype: TreeNode
        """

        preorder = deque(preorder)

        return self.build(preorder, inorder)

    def build(self, preorder: deque, inorder: List):
        next_node = preorder.popleft()

        node = TreeNode(next_node)

        if not len(inorder):
            return node

        split_index = inorder.index(next_node)

        left_inorder = inorder[:split_index]
        right_inorder = inorder[split_index+1:]

        if not inorder:
            return node

        if preorder:
            node.left = self.build(preorder, left_inorder)
        if preorder:
            node.right = self.build(preorder, right_inorder)

        return node


if __name__ == "__main__":

    preorder = [3, 9, 20, 15, 7]
    in_order = [9, 3, 15, 20, 7]

    Solution().buildTree(preorder, in_order).print_tree()
