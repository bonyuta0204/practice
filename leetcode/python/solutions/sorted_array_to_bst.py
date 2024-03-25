from data_structures.tree import TreeNode


class Solution(object):
    def sortedArrayToBST(self, nums) -> TreeNode:
        """
        :type nums: List[int]
        :rtype: TreeNode
        """

        if not nums:
            return None

        center_index = len(nums) // 2

        center = nums[center_index]

        left = nums[:center_index]

        right = nums[center_index+1:]

        node = TreeNode(center)
        node.left = self.sortedArrayToBST(left)
        node.right = self.sortedArrayToBST(right)

        return node


if __name__ == "__main__":
    Solution().sortedArrayToBST([1, 2, 3, 4]).print_tree()
